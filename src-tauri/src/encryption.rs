use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::rngs::OsRng;
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use serde::{Deserialize, Serialize};

const ARGON2_MEMORY_COST: u32 = 65536; // 64 MB
const ARGON2_TIME_COST: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;

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
        match self {
            EncryptionError::KeyDerivation(msg) => write!(f, "Key derivation error: {}", msg),
            EncryptionError::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            EncryptionError::Decryption(msg) => write!(f, "Decryption error: {}", msg),
            EncryptionError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
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
