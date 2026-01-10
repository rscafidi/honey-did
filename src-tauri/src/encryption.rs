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
