use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::rngs::OsRng;
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::pbkdf2;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

const ARGON2_MEMORY_COST: u32 = 65536; // 64 MB
const ARGON2_TIME_COST: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;

// PBKDF2 iterations - high enough for security, compatible with Web Crypto
const PBKDF2_ITERATIONS: u32 = 600_000;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedPayload {
    pub salt: String,      // Base64-encoded Argon2 salt
    pub nonce: String,     // Base64-encoded AES-GCM nonce
    pub ciphertext: String, // Base64-encoded encrypted data
}

#[derive(Debug)]
pub enum EncryptionError {
    KeyDerivation(String),
    Encryption(String),
    Decryption(String),
    InvalidData(String),
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use generic error messages to avoid information disclosure
        match self {
            EncryptionError::KeyDerivation(_) => write!(f, "Failed to process security key"),
            EncryptionError::Encryption(_) => write!(f, "Failed to encrypt data"),
            EncryptionError::Decryption(_) => write!(f, "Decryption failed - incorrect passphrase or corrupted data"),
            EncryptionError::InvalidData(_) => write!(f, "Invalid or corrupted data format"),
        }
    }
}

impl EncryptionError {
    /// Returns detailed error for logging (not for display to users)
    #[allow(dead_code)]
    pub fn detail(&self) -> &str {
        match self {
            EncryptionError::KeyDerivation(msg) => msg,
            EncryptionError::Encryption(msg) => msg,
            EncryptionError::Decryption(msg) => msg,
            EncryptionError::InvalidData(msg) => msg,
        }
    }
}

impl std::error::Error for EncryptionError {}

/// Derives a 256-bit key from a passphrase using Argon2id
pub fn derive_key(passphrase: &str, salt: &[u8]) -> Result<[u8; 32], EncryptionError> {
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            ARGON2_MEMORY_COST,
            ARGON2_TIME_COST,
            ARGON2_PARALLELISM,
            Some(32),
        )
        .map_err(|e| EncryptionError::KeyDerivation(e.to_string()))?,
    );

    let mut key = [0u8; 32];
    argon2
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|e| EncryptionError::KeyDerivation(e.to_string()))?;

    Ok(key)
}

/// Generates a random 16-byte salt for Argon2
pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    use rand::RngCore;
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Encrypts plaintext using AES-256-GCM with a passphrase
pub fn encrypt(plaintext: &str, passphrase: &str) -> Result<EncryptedPayload, EncryptionError> {
    // Generate random salt and nonce
    let salt = generate_salt();
    let mut nonce_bytes = [0u8; 12];
    use rand::RngCore;
    OsRng.fill_bytes(&mut nonce_bytes);

    // Derive key from passphrase
    let key_bytes = derive_key(passphrase, &salt)?;

    // Create AES-256-GCM key
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::Encryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    // Encrypt
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let mut in_out = plaintext.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| EncryptionError::Encryption("Encryption failed".into()))?;

    Ok(EncryptedPayload {
        salt: BASE64.encode(salt),
        nonce: BASE64.encode(nonce_bytes),
        ciphertext: BASE64.encode(in_out),
    })
}

/// Encrypts using PBKDF2 key derivation (compatible with Web Crypto API)
/// Use this for data that needs to be decrypted in a browser
pub fn encrypt_for_browser(plaintext: &str, passphrase: &str) -> Result<EncryptedPayload, EncryptionError> {
    // Generate random salt and nonce
    let salt = generate_salt();
    let mut nonce_bytes = [0u8; 12];
    use rand::RngCore;
    OsRng.fill_bytes(&mut nonce_bytes);

    // Derive key using PBKDF2 (Web Crypto compatible)
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &salt,
        passphrase.as_bytes(),
        &mut key_bytes,
    );

    // Create AES-256-GCM key
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::Encryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    // Encrypt
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let mut in_out = plaintext.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| EncryptionError::Encryption("Encryption failed".into()))?;

    Ok(EncryptedPayload {
        salt: BASE64.encode(salt),
        nonce: BASE64.encode(nonce_bytes),
        ciphertext: BASE64.encode(in_out),
    })
}

/// Decrypts using PBKDF2 key derivation (compatible with Web Crypto API)
/// Use this for data encrypted with encrypt_for_browser
pub fn decrypt_from_browser(payload: &EncryptedPayload, passphrase: &str) -> Result<String, EncryptionError> {
    // Decode base64 values
    let salt = BASE64
        .decode(&payload.salt)
        .map_err(|_| EncryptionError::InvalidData("Invalid salt".into()))?;
    let nonce_bytes: [u8; 12] = BASE64
        .decode(&payload.nonce)
        .map_err(|_| EncryptionError::InvalidData("Invalid nonce".into()))?
        .try_into()
        .map_err(|_| EncryptionError::InvalidData("Nonce wrong length".into()))?;
    let mut ciphertext = BASE64
        .decode(&payload.ciphertext)
        .map_err(|_| EncryptionError::InvalidData("Invalid ciphertext".into()))?;

    // Derive key using PBKDF2 (Web Crypto compatible)
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &salt,
        passphrase.as_bytes(),
        &mut key_bytes,
    );

    // Create AES-256-GCM key
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::Decryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    // Decrypt
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let plaintext = key
        .open_in_place(nonce, Aad::empty(), &mut ciphertext)
        .map_err(|_| EncryptionError::Decryption("Decryption failed - wrong passphrase?".into()))?;

    String::from_utf8(plaintext.to_vec())
        .map_err(|_| EncryptionError::Decryption("Invalid UTF-8".into()))
}

/// Generates a random 32-byte document key
pub fn generate_document_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    use rand::RngCore;
    OsRng.fill_bytes(&mut key);
    key
}

/// Encrypts raw bytes with a raw key using AES-256-GCM
pub fn encrypt_with_raw_key(plaintext: &[u8], key: &[u8; 32]) -> Result<EncryptedPayload, EncryptionError> {
    let mut nonce_bytes = [0u8; 12];
    use rand::RngCore;
    OsRng.fill_bytes(&mut nonce_bytes);

    let unbound_key = UnboundKey::new(&AES_256_GCM, key)
        .map_err(|_| EncryptionError::Encryption("Failed to create key".into()))?;
    let aead_key = LessSafeKey::new(unbound_key);

    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let mut in_out = plaintext.to_vec();
    aead_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| EncryptionError::Encryption("Encryption failed".into()))?;

    Ok(EncryptedPayload {
        salt: String::new(), // Not used for raw key encryption
        nonce: BASE64.encode(nonce_bytes),
        ciphertext: BASE64.encode(in_out),
    })
}

/// Encrypts the document key with a passphrase-derived key using PBKDF2
pub fn encrypt_key_with_passphrase(document_key: &[u8; 32], passphrase: &str) -> Result<EncryptedPayload, EncryptionError> {
    let salt = generate_salt();
    let mut nonce_bytes = [0u8; 12];
    use rand::RngCore;
    OsRng.fill_bytes(&mut nonce_bytes);

    // Derive key using PBKDF2
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &salt,
        passphrase.as_bytes(),
        &mut key_bytes,
    );

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::Encryption("Failed to create key".into()))?;
    let aead_key = LessSafeKey::new(unbound_key);

    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let mut in_out = document_key.to_vec();
    aead_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|_| EncryptionError::Encryption("Encryption failed".into()))?;

    Ok(EncryptedPayload {
        salt: BASE64.encode(salt),
        nonce: BASE64.encode(nonce_bytes),
        ciphertext: BASE64.encode(in_out),
    })
}

/// Decrypts the document key from an encrypted payload using PBKDF2 + passphrase
pub fn decrypt_key_with_passphrase(payload: &EncryptedPayload, passphrase: &str) -> Result<[u8; 32], EncryptionError> {
    let salt = BASE64
        .decode(&payload.salt)
        .map_err(|_| EncryptionError::InvalidData("Invalid salt".into()))?;
    let nonce_bytes: [u8; 12] = BASE64
        .decode(&payload.nonce)
        .map_err(|_| EncryptionError::InvalidData("Invalid nonce".into()))?
        .try_into()
        .map_err(|_| EncryptionError::InvalidData("Nonce wrong length".into()))?;
    let mut ciphertext = BASE64
        .decode(&payload.ciphertext)
        .map_err(|_| EncryptionError::InvalidData("Invalid ciphertext".into()))?;

    // Derive key using PBKDF2
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        &salt,
        passphrase.as_bytes(),
        &mut key_bytes,
    );

    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::Decryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let plaintext = key
        .open_in_place(nonce, Aad::empty(), &mut ciphertext)
        .map_err(|_| EncryptionError::Decryption("Decryption failed - wrong passphrase?".into()))?;

    plaintext.try_into()
        .map_err(|_| EncryptionError::Decryption("Decrypted key has wrong length".into()))
}

/// Decrypts data using a raw 32-byte key (no salt needed)
pub fn decrypt_with_raw_key(nonce: &str, ciphertext: &str, key: &[u8; 32]) -> Result<String, EncryptionError> {
    let nonce_bytes: [u8; 12] = BASE64
        .decode(nonce)
        .map_err(|_| EncryptionError::InvalidData("Invalid nonce".into()))?
        .try_into()
        .map_err(|_| EncryptionError::InvalidData("Nonce wrong length".into()))?;
    let mut ciphertext_bytes = BASE64
        .decode(ciphertext)
        .map_err(|_| EncryptionError::InvalidData("Invalid ciphertext".into()))?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, key)
        .map_err(|_| EncryptionError::Decryption("Failed to create key".into()))?;
    let aead_key = LessSafeKey::new(unbound_key);

    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let plaintext = aead_key
        .open_in_place(nonce, Aad::empty(), &mut ciphertext_bytes)
        .map_err(|_| EncryptionError::Decryption("Decryption failed".into()))?;

    String::from_utf8(plaintext.to_vec())
        .map_err(|_| EncryptionError::Decryption("Invalid UTF-8".into()))
}

/// Decrypts an encrypted payload using a passphrase
pub fn decrypt(payload: &EncryptedPayload, passphrase: &str) -> Result<String, EncryptionError> {
    // Decode base64 values
    let salt = BASE64
        .decode(&payload.salt)
        .map_err(|_| EncryptionError::InvalidData("Invalid salt".into()))?;
    let nonce_bytes: [u8; 12] = BASE64
        .decode(&payload.nonce)
        .map_err(|_| EncryptionError::InvalidData("Invalid nonce".into()))?
        .try_into()
        .map_err(|_| EncryptionError::InvalidData("Nonce wrong length".into()))?;
    let mut ciphertext = BASE64
        .decode(&payload.ciphertext)
        .map_err(|_| EncryptionError::InvalidData("Invalid ciphertext".into()))?;

    // Derive key from passphrase
    let key_bytes = derive_key(passphrase, &salt)?;

    // Create AES-256-GCM key
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::Decryption("Failed to create key".into()))?;
    let key = LessSafeKey::new(unbound_key);

    // Decrypt
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let plaintext = key
        .open_in_place(nonce, Aad::empty(), &mut ciphertext)
        .map_err(|_| EncryptionError::Decryption("Decryption failed - wrong passphrase?".into()))?;

    String::from_utf8(plaintext.to_vec())
        .map_err(|_| EncryptionError::Decryption("Invalid UTF-8".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let plaintext = "Hello, this is secret data!";
        let passphrase = "correct-horse-battery-staple";

        let encrypted = encrypt(plaintext, passphrase).expect("encryption should succeed");
        let decrypted = decrypt(&encrypted, passphrase).expect("decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_passphrase_fails() {
        let plaintext = "Secret message";
        let encrypted = encrypt(plaintext, "correct-passphrase").expect("encryption should succeed");

        let result = decrypt(&encrypted, "wrong-passphrase");
        assert!(result.is_err());
    }

    #[test]
    fn test_different_encryptions_produce_different_output() {
        let plaintext = "Same message";
        let passphrase = "same-passphrase";

        let encrypted1 = encrypt(plaintext, passphrase).expect("encryption should succeed");
        let encrypted2 = encrypt(plaintext, passphrase).expect("encryption should succeed");

        // Salt and nonce should differ, making ciphertext different
        assert_ne!(encrypted1.salt, encrypted2.salt);
        assert_ne!(encrypted1.nonce, encrypted2.nonce);
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
    }

    #[test]
    fn test_empty_plaintext() {
        let encrypted = encrypt("", "passphrase").expect("encryption should succeed");
        let decrypted = decrypt(&encrypted, "passphrase").expect("decryption should succeed");
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_unicode_content() {
        let plaintext = "Hello 世界 🔒 émojis";
        let passphrase = "unicode-passphrase-日本語";

        let encrypted = encrypt(plaintext, passphrase).expect("encryption should succeed");
        let decrypted = decrypt(&encrypted, passphrase).expect("decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_browser_encrypt_decrypt_roundtrip() {
        let plaintext = "Browser-compatible encryption test!";
        let passphrase = "browser-test-passphrase";

        let encrypted = encrypt_for_browser(plaintext, passphrase)
            .expect("browser encryption should succeed");
        let decrypted = decrypt_from_browser(&encrypted, passphrase)
            .expect("browser decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_browser_wrong_passphrase_fails() {
        let plaintext = "Secret data";
        let encrypted = encrypt_for_browser(plaintext, "correct")
            .expect("encryption should succeed");

        let result = decrypt_from_browser(&encrypted, "wrong");
        assert!(result.is_err());
    }

    #[test]
    fn test_browser_encryption_produces_unique_output() {
        let plaintext = "Same message";
        let passphrase = "same-pass";

        let enc1 = encrypt_for_browser(plaintext, passphrase).unwrap();
        let enc2 = encrypt_for_browser(plaintext, passphrase).unwrap();

        // Salt and nonce should differ
        assert_ne!(enc1.salt, enc2.salt);
        assert_ne!(enc1.nonce, enc2.nonce);
        assert_ne!(enc1.ciphertext, enc2.ciphertext);
    }
}
