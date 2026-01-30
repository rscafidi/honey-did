use crate::encryption::{decrypt, encrypt, EncryptedPayload, EncryptionError};
use crate::models::LegacyDocument;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
#[cfg(not(target_os = "android"))]
use directories::ProjectDirs;
#[cfg(not(target_os = "android"))]
use keyring::Entry;
use std::fs;
use std::path::PathBuf;

const APP_QUALIFIER: &str = "com";
const APP_ORGANIZATION: &str = "honeydid";
const APP_NAME: &str = "honey-did";
const KEYRING_SERVICE: &str = "honey-did-local";
const KEYRING_USER: &str = "local-encryption-key";

#[derive(Debug)]
pub enum StorageError {
    IoError(String),
    EncryptionError(EncryptionError),
    SerializationError(String),
    KeyringError(String),
    NoDataDirectory,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use generic error messages to avoid information disclosure
        match self {
            StorageError::IoError(_) => write!(f, "Failed to read or write data"),
            StorageError::EncryptionError(e) => write!(f, "{}", e),
            StorageError::SerializationError(_) => write!(f, "Failed to process data format"),
            StorageError::KeyringError(_) => write!(f, "Failed to access secure storage"),
            StorageError::NoDataDirectory => write!(f, "Failed to access application data"),
        }
    }
}

impl StorageError {
    /// Returns detailed error for logging (not for display to users)
    #[allow(dead_code)]
    pub fn detail(&self) -> String {
        match self {
            StorageError::IoError(msg) => msg.clone(),
            StorageError::EncryptionError(e) => e.detail().to_string(),
            StorageError::SerializationError(msg) => msg.clone(),
            StorageError::KeyringError(msg) => msg.clone(),
            StorageError::NoDataDirectory => "No data directory available".to_string(),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<EncryptionError> for StorageError {
    fn from(e: EncryptionError) -> Self {
        StorageError::EncryptionError(e)
    }
}

/// Gets the application data directory
#[cfg(not(target_os = "android"))]
pub fn get_data_dir() -> Result<PathBuf, StorageError> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
        .map(|dirs| dirs.data_dir().to_path_buf())
        .ok_or(StorageError::NoDataDirectory)
}

#[cfg(target_os = "android")]
pub fn get_data_dir() -> Result<PathBuf, StorageError> {
    // On Android, use the app-private data directory.
    // Try multiple approaches in order of reliability.
    if let Ok(home) = std::env::var("HOME") {
        return Ok(PathBuf::from(home));
    }
    // Fallback: use the standard Android app data path
    let path = PathBuf::from("/data/data/com.honeydid.app/files");
    if path.exists() || std::fs::create_dir_all(&path).is_ok() {
        return Ok(path);
    }
    if let Ok(tmpdir) = std::env::var("TMPDIR") {
        let data = PathBuf::from(tmpdir).join("data");
        let _ = std::fs::create_dir_all(&data);
        return Ok(data);
    }
    Err(StorageError::NoDataDirectory)
}

/// Generates a cryptographically secure random key (128 hex characters)
fn generate_random_key() -> String {
    use rand::RngCore;
    use rand::rngs::OsRng;
    let mut key_bytes = [0u8; 64];
    OsRng.fill_bytes(&mut key_bytes);
    key_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Gets or creates a local encryption key stored in the OS keyring
#[cfg(not(target_os = "android"))]
pub fn get_or_create_local_key() -> Result<String, StorageError> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| StorageError::KeyringError(e.to_string()))?;

    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(keyring::Error::NoEntry) => {
            let key = generate_random_key();
            entry.set_password(&key)
                .map_err(|e| StorageError::KeyringError(e.to_string()))?;
            Ok(key)
        }
        Err(e) => Err(StorageError::KeyringError(e.to_string())),
    }
}

/// Gets or creates a local encryption key stored in the app-private data directory
#[cfg(target_os = "android")]
pub fn get_or_create_local_key() -> Result<String, StorageError> {
    let data_dir = get_data_dir()?;
    let key_file = data_dir.join(".local_key");

    if key_file.exists() {
        let key = fs::read_to_string(&key_file)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        if key.len() == 128 {
            return Ok(key);
        }
    }

    let key = generate_random_key();
    fs::create_dir_all(&data_dir)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    fs::write(&key_file, &key)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(key)
}

/// Saves the document to local encrypted storage
pub fn save_document(document: &LegacyDocument) -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    fs::create_dir_all(&data_dir)
        .map_err(|e| StorageError::IoError(e.to_string()))?;

    let json = serde_json::to_string(document)
        .map_err(|e| StorageError::SerializationError(e.to_string()))?;

    let local_key = get_or_create_local_key()?;
    let encrypted = encrypt(&json, &local_key)?;

    let encrypted_json = serde_json::to_string(&encrypted)
        .map_err(|e| StorageError::SerializationError(e.to_string()))?;

    let file_path = data_dir.join("document.encrypted");
    fs::write(&file_path, encrypted_json)
        .map_err(|e| StorageError::IoError(e.to_string()))?;

    Ok(())
}

/// Loads the document from local encrypted storage
pub fn load_document() -> Result<Option<LegacyDocument>, StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("document.encrypted");

    if !file_path.exists() {
        return Ok(None);
    }

    let encrypted_json = fs::read_to_string(&file_path)
        .map_err(|e| StorageError::IoError(e.to_string()))?;

    let encrypted: EncryptedPayload = serde_json::from_str(&encrypted_json)
        .map_err(|e| StorageError::SerializationError(e.to_string()))?;

    let local_key = get_or_create_local_key()?;
    let json = decrypt(&encrypted, &local_key)?;

    let document: LegacyDocument = serde_json::from_str(&json)
        .map_err(|e| StorageError::SerializationError(e.to_string()))?;

    Ok(Some(document))
}

/// Hashes a password using Argon2id
pub fn hash_password(password: &str) -> Result<String, StorageError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| StorageError::KeyringError(format!("Password hashing failed: {}", e)))?;
    Ok(hash.to_string())
}

/// Verifies a password against a stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, StorageError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| StorageError::KeyringError(format!("Invalid hash format: {}", e)))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Saves the app password hash
pub fn save_password_hash(hash: &str) -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    fs::create_dir_all(&data_dir)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    let file_path = data_dir.join("password.hash");
    fs::write(&file_path, hash)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(())
}

/// Loads the app password hash if it exists
pub fn load_password_hash() -> Result<Option<String>, StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("password.hash");
    if !file_path.exists() {
        return Ok(None);
    }
    let hash = fs::read_to_string(&file_path)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(Some(hash))
}

/// Deletes the password hash file
pub fn delete_password_hash() -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("password.hash");
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
    }
    Ok(())
}

/// Deletes the document file
pub fn delete_document() -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("document.encrypted");
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
    }
    Ok(())
}

/// Saves settings to a JSON file
pub fn save_settings(clear_on_exit: bool) -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    fs::create_dir_all(&data_dir)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    let file_path = data_dir.join("settings.json");
    let json = format!(r#"{{"clear_on_exit":{}}}"#, clear_on_exit);
    fs::write(&file_path, json)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(())
}

/// Loads settings from JSON file
pub fn load_settings() -> Result<bool, StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("settings.json");
    if !file_path.exists() {
        return Ok(false);
    }
    let json = fs::read_to_string(&file_path)
        .map_err(|e| StorageError::IoError(e.to_string()))?;
    Ok(json.contains("true"))
}

/// Deletes settings file
pub fn delete_settings() -> Result<(), StorageError> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join("settings.json");
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| StorageError::IoError(e.to_string()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_get_data_dir_returns_path() {
        let result = get_data_dir();
        // Should return a valid path containing app name
        assert!(result.is_ok());
        let dir = result.unwrap();
        assert!(dir.to_string_lossy().contains("honey-did"));
    }

    #[test]
    fn test_save_and_load_document_roundtrip() {
        // Note: This test requires keyring access and may fail in CI
        let mut doc = LegacyDocument::default();
        doc.meta.creator_name = "Storage Test User".to_string();
        doc.financial.notes = "Storage test notes".to_string();

        // Attempt save - may fail if keyring unavailable
        let save_result = save_document(&doc);
        if save_result.is_err() {
            // Skip test if keyring not available
            eprintln!("Skipping test: keyring unavailable");
            return;
        }

        // Load back
        let loaded = load_document()
            .expect("load should succeed")
            .expect("document should exist");

        assert_eq!(loaded.meta.creator_name, "Storage Test User");
        assert_eq!(loaded.financial.notes, "Storage test notes");
    }
}
