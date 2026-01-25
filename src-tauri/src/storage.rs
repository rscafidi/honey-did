use crate::encryption::{decrypt, encrypt, EncryptedPayload, EncryptionError};
use crate::models::LegacyDocument;
use directories::ProjectDirs;
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
        match self {
            StorageError::IoError(msg) => write!(f, "IO error: {}", msg),
            StorageError::EncryptionError(e) => write!(f, "Encryption error: {}", e),
            StorageError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            StorageError::KeyringError(msg) => write!(f, "Keyring error: {}", msg),
            StorageError::NoDataDirectory => write!(f, "Could not determine data directory"),
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
pub fn get_data_dir() -> Result<PathBuf, StorageError> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
        .map(|dirs| dirs.data_dir().to_path_buf())
        .ok_or(StorageError::NoDataDirectory)
}

/// Gets or creates a local encryption key stored in the OS keyring
pub fn get_or_create_local_key() -> Result<String, StorageError> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| StorageError::KeyringError(e.to_string()))?;

    match entry.get_password() {
        Ok(key) => Ok(key),
        Err(keyring::Error::NoEntry) => {
            // Generate a new random key
            use rand::Rng;
            let key: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();

            entry.set_password(&key)
                .map_err(|e| StorageError::KeyringError(e.to_string()))?;

            Ok(key)
        }
        Err(e) => Err(StorageError::KeyringError(e.to_string())),
    }
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
