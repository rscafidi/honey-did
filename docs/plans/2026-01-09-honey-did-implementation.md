# honey-did Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a cross-platform desktop app that guides users through creating secure legacy documents for their family.

**Architecture:** Tauri 2.0 desktop app with Rust backend handling encryption/storage and Svelte frontend for the wizard UI. Data exports as a self-contained encrypted HTML file that opens in any browser.

**Tech Stack:** Tauri 2.0, Svelte, Rust (ring for crypto, argon2 for key derivation), TypeScript

---

## Phase 1: Project Scaffolding

### Task 1.1: Initialize Tauri + Svelte Project

**Files:**
- Create: `package.json`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/main.rs`
- Create: `src/App.svelte`
- Create: `src/main.ts`

**Step 1: Create Tauri project with Svelte template**

Run:
```bash
npm create tauri-app@latest . -- --template svelte-ts --manager npm
```

Accept defaults when prompted.

**Step 2: Verify project structure exists**

Run:
```bash
ls -la src-tauri/src/main.rs src/App.svelte
```

Expected: Both files exist

**Step 3: Install dependencies**

Run:
```bash
npm install
```

Expected: node_modules created, no errors

**Step 4: Commit scaffolding**

```bash
git add -A
git commit -m "chore: scaffold Tauri + Svelte project"
```

---

### Task 1.2: Add Rust Dependencies

**Files:**
- Modify: `src-tauri/Cargo.toml`

**Step 1: Add crypto and serialization dependencies**

Add to `[dependencies]` section in `src-tauri/Cargo.toml`:

```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ring = "0.17"
argon2 = "0.5"
base64 = "0.21"
rand = "0.8"
keyring = "2"
directories = "5"
```

**Step 2: Verify dependencies resolve**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 3: Commit dependency additions**

```bash
git add src-tauri/Cargo.toml
git commit -m "chore: add Rust crypto and utility dependencies"
```

---

## Phase 2: Rust Backend - Encryption Module

### Task 2.1: Create Encryption Module Structure

**Files:**
- Create: `src-tauri/src/encryption.rs`
- Modify: `src-tauri/src/main.rs`

**Step 1: Create encryption module with types**

Create `src-tauri/src/encryption.rs`:

```rust
use argon2::{Argon2, password_hash::SaltString};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::rngs::OsRng;
use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
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
```

**Step 2: Add module to main.rs**

Add to top of `src-tauri/src/main.rs`:

```rust
mod encryption;
```

**Step 3: Verify it compiles**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 4: Commit**

```bash
git add src-tauri/src/encryption.rs src-tauri/src/main.rs
git commit -m "feat: add encryption module structure"
```

---

### Task 2.2: Implement Key Derivation

**Files:**
- Modify: `src-tauri/src/encryption.rs`

**Step 1: Add derive_key function**

Add to `src-tauri/src/encryption.rs`:

```rust
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
```

**Step 2: Verify it compiles**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 3: Commit**

```bash
git add src-tauri/src/encryption.rs
git commit -m "feat: implement Argon2id key derivation"
```

---

### Task 2.3: Implement Encrypt Function

**Files:**
- Modify: `src-tauri/src/encryption.rs`

**Step 1: Add encrypt function**

Add to `src-tauri/src/encryption.rs`:

```rust
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
```

**Step 2: Verify it compiles**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 3: Commit**

```bash
git add src-tauri/src/encryption.rs
git commit -m "feat: implement AES-256-GCM encryption"
```

---

### Task 2.4: Implement Decrypt Function

**Files:**
- Modify: `src-tauri/src/encryption.rs`

**Step 1: Add decrypt function**

Add to `src-tauri/src/encryption.rs`:

```rust
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
```

**Step 2: Verify it compiles**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 3: Commit**

```bash
git add src-tauri/src/encryption.rs
git commit -m "feat: implement AES-256-GCM decryption"
```

---

### Task 2.5: Add Encryption Tests

**Files:**
- Modify: `src-tauri/src/encryption.rs`

**Step 1: Add tests module**

Add to end of `src-tauri/src/encryption.rs`:

```rust
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
}
```

**Step 2: Run tests**

Run:
```bash
cd src-tauri && cargo test
```

Expected: All 5 tests pass

**Step 3: Commit**

```bash
git add src-tauri/src/encryption.rs
git commit -m "test: add encryption module tests"
```

---

## Phase 3: Rust Backend - Data Model

### Task 3.1: Create Data Model

**Files:**
- Create: `src-tauri/src/models.rs`
- Modify: `src-tauri/src/main.rs`

**Step 1: Create data model for all sections**

Create `src-tauri/src/models.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegacyDocument {
    pub meta: DocumentMeta,
    pub financial: FinancialSection,
    pub insurance: InsuranceSection,
    pub bills: BillsSection,
    pub property: PropertySection,
    pub legal: LegalSection,
    pub digital: DigitalSection,
    pub household: HouseholdSection,
    pub personal: PersonalSection,
    pub contacts: ContactsSection,
    pub medical: MedicalSection,
    pub pets: PetsSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMeta {
    pub creator_name: String,
    pub created_at: String,
    pub updated_at: String,
}

// --- Financial Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialSection {
    pub bank_accounts: Vec<BankAccount>,
    pub credit_cards: Vec<CreditCard>,
    pub investments: Vec<Investment>,
    pub debts: Vec<Debt>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccount {
    pub name: String,
    pub institution: String,
    pub account_type: String,
    pub last_four: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditCard {
    pub name: String,
    pub issuer: String,
    pub last_four: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Investment {
    pub name: String,
    pub institution: String,
    pub account_type: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Debt {
    pub name: String,
    pub lender: String,
    pub notes: String,
}

// --- Insurance Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InsuranceSection {
    pub policies: Vec<InsurancePolicy>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InsurancePolicy {
    pub policy_type: String,  // life, health, home, auto, etc.
    pub provider: String,
    pub policy_number: String,
    pub contact: String,
    pub notes: String,
}

// --- Bills Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillsSection {
    pub bills: Vec<Bill>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bill {
    pub name: String,
    pub provider: String,
    pub amount: String,
    pub due_day: String,
    pub autopay: bool,
    pub notes: String,
}

// --- Property Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropertySection {
    pub properties: Vec<Property>,
    pub vehicles: Vec<Vehicle>,
    pub valuables: Vec<Valuable>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Property {
    pub name: String,
    pub address: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vehicle {
    pub name: String,
    pub details: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Valuable {
    pub name: String,
    pub location: String,
    pub notes: String,
}

// --- Legal Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LegalSection {
    pub will_location: String,
    pub attorney: Contact,
    pub power_of_attorney: String,
    pub trusts: Vec<Trust>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Trust {
    pub name: String,
    pub trustee: String,
    pub notes: String,
}

// --- Digital Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DigitalSection {
    pub email_accounts: Vec<DigitalAccount>,
    pub social_media: Vec<DigitalAccount>,
    pub password_manager: PasswordManagerInfo,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DigitalAccount {
    pub name: String,
    pub username: String,
    pub recovery_hint: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PasswordManagerInfo {
    pub name: String,
    pub master_password_hint: String,
    pub recovery_method: String,
    pub notes: String,
}

// --- Household Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HouseholdSection {
    pub maintenance_items: Vec<MaintenanceItem>,
    pub contractors: Vec<Contact>,
    pub how_things_work: Vec<HowTo>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MaintenanceItem {
    pub name: String,
    pub frequency: String,
    pub last_done: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HowTo {
    pub name: String,
    pub instructions: String,
}

// --- Personal Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalSection {
    pub funeral_preferences: String,
    pub obituary_notes: String,
    pub messages: Vec<PersonalMessage>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalMessage {
    pub recipient: String,
    pub message: String,
}

// --- Contacts Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContactsSection {
    pub emergency_contacts: Vec<Contact>,
    pub family: Vec<Contact>,
    pub professionals: Vec<Contact>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Contact {
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub email: String,
    pub notes: String,
}

// --- Medical Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MedicalSection {
    pub family_members: Vec<FamilyMedical>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FamilyMedical {
    pub name: String,
    pub doctors: Vec<Contact>,
    pub medications: Vec<Medication>,
    pub conditions: Vec<String>,
    pub allergies: Vec<String>,
    pub pharmacy: Contact,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Medication {
    pub name: String,
    pub dosage: String,
    pub frequency: String,
    pub prescriber: String,
    pub notes: String,
}

// --- Pets Section ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PetsSection {
    pub pets: Vec<Pet>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pet {
    pub name: String,
    pub species: String,
    pub breed: String,
    pub vet: Contact,
    pub medications: Vec<Medication>,
    pub feeding: String,
    pub care_notes: String,
}
```

**Step 2: Add module to main.rs**

Add to top of `src-tauri/src/main.rs`:

```rust
mod models;
```

**Step 3: Verify it compiles**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 4: Commit**

```bash
git add src-tauri/src/models.rs src-tauri/src/main.rs
git commit -m "feat: add complete data model for legacy document"
```

---

## Phase 4: Rust Backend - Storage Module

### Task 4.1: Create Storage Module

**Files:**
- Create: `src-tauri/src/storage.rs`
- Modify: `src-tauri/src/main.rs`

**Step 1: Create storage module**

Create `src-tauri/src/storage.rs`:

```rust
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
```

**Step 2: Add module to main.rs**

Add to top of `src-tauri/src/main.rs`:

```rust
mod storage;
```

**Step 3: Verify it compiles**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 4: Commit**

```bash
git add src-tauri/src/storage.rs src-tauri/src/main.rs
git commit -m "feat: add encrypted local storage module"
```

---

## Phase 5: Rust Backend - Export Module

### Task 5.1: Create Export Module

**Files:**
- Create: `src-tauri/src/export.rs`
- Modify: `src-tauri/src/main.rs`

**Step 1: Create export module**

Create `src-tauri/src/export.rs`:

```rust
use crate::encryption::{encrypt, EncryptionError};
use crate::models::LegacyDocument;

#[derive(Debug)]
pub enum ExportError {
    EncryptionError(EncryptionError),
    SerializationError(String),
    IoError(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::EncryptionError(e) => write!(f, "Encryption error: {}", e),
            ExportError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ExportError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ExportError {}

impl From<EncryptionError> for ExportError {
    fn from(e: EncryptionError) -> Self {
        ExportError::EncryptionError(e)
    }
}

/// Generates the encrypted HTML file content
pub fn generate_encrypted_html(
    document: &LegacyDocument,
    passphrase: &str,
) -> Result<String, ExportError> {
    // Serialize document to JSON
    let json = serde_json::to_string(document)
        .map_err(|e| ExportError::SerializationError(e.to_string()))?;

    // Encrypt the JSON
    let encrypted = encrypt(&json, passphrase)?;

    // Serialize encrypted payload
    let encrypted_json = serde_json::to_string(&encrypted)
        .map_err(|e| ExportError::SerializationError(e.to_string()))?;

    // Generate the HTML
    let html = generate_html_template(&encrypted_json, &document.meta.creator_name);

    Ok(html)
}

fn generate_html_template(encrypted_data: &str, creator_name: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>honey-did - Legacy Document</title>
    <style>
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; background: #f5f5f5; color: #333; }}
        .container {{ max-width: 800px; margin: 0 auto; padding: 20px; }}
        .lock-screen {{ display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; text-align: center; }}
        .lock-icon {{ font-size: 4rem; margin-bottom: 1rem; }}
        .lock-title {{ font-size: 1.5rem; margin-bottom: 0.5rem; }}
        .lock-subtitle {{ color: #666; margin-bottom: 2rem; }}
        .password-form {{ display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 300px; }}
        .password-input {{ padding: 12px; font-size: 1rem; border: 2px solid #ddd; border-radius: 8px; text-align: center; }}
        .password-input:focus {{ outline: none; border-color: #007bff; }}
        .unlock-btn {{ padding: 12px 24px; font-size: 1rem; background: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer; }}
        .unlock-btn:hover {{ background: #0056b3; }}
        .error {{ color: #dc3545; margin-top: 1rem; }}
        .content {{ display: none; }}
        .content.visible {{ display: block; }}
        .header {{ background: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .header-title {{ font-size: 1.5rem; }}
        .toolbar {{ display: flex; gap: 1rem; margin-top: 1rem; }}
        .search-input {{ flex: 1; padding: 8px 12px; border: 1px solid #ddd; border-radius: 4px; }}
        .print-btn {{ padding: 8px 16px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer; }}
        .toc {{ background: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .toc-title {{ font-weight: bold; margin-bottom: 1rem; }}
        .toc-list {{ list-style: none; }}
        .toc-list li {{ margin: 0.5rem 0; }}
        .toc-list a {{ color: #007bff; text-decoration: none; }}
        .toc-list a:hover {{ text-decoration: underline; }}
        .section {{ background: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .section-title {{ font-size: 1.25rem; border-bottom: 2px solid #007bff; padding-bottom: 0.5rem; margin-bottom: 1rem; }}
        .item {{ background: #f8f9fa; padding: 15px; border-radius: 4px; margin-bottom: 10px; }}
        .item-title {{ font-weight: bold; margin-bottom: 0.5rem; }}
        .item-detail {{ color: #666; font-size: 0.9rem; }}
        .notes {{ background: #fff3cd; padding: 10px; border-radius: 4px; margin-top: 1rem; font-style: italic; }}
        .highlight {{ background: yellow; }}
        @media print {{ .toolbar, .toc {{ display: none; }} .section {{ break-inside: avoid; }} }}
    </style>
</head>
<body>
    <div id="lockScreen" class="lock-screen">
        <div class="lock-icon">🔒</div>
        <h1 class="lock-title">honey-did</h1>
        <p class="lock-subtitle">This file was created by {creator_name}<br>to help you in their absence.</p>
        <form class="password-form" onsubmit="return unlock(event)">
            <input type="password" id="passphrase" class="password-input" placeholder="Enter passphrase" autofocus>
            <button type="submit" class="unlock-btn">Unlock</button>
        </form>
        <p id="error" class="error" style="display: none;"></p>
    </div>
    <div id="content" class="content">
        <div class="container" id="documentContent"></div>
    </div>
    <script>
        const ENCRYPTED_DATA = {encrypted_data};

        async function deriveKey(passphrase, salt) {{
            const encoder = new TextEncoder();
            const keyMaterial = await crypto.subtle.importKey(
                'raw', encoder.encode(passphrase), 'PBKDF2', false, ['deriveBits']
            );
            // Use PBKDF2 as fallback since Argon2 isn't available in Web Crypto
            // For browser decryption, we'll use a compatible approach
            const keyBits = await crypto.subtle.deriveBits(
                {{ name: 'PBKDF2', salt: salt, iterations: 600000, hash: 'SHA-256' }},
                keyMaterial, 256
            );
            return await crypto.subtle.importKey(
                'raw', keyBits, {{ name: 'AES-GCM' }}, false, ['decrypt']
            );
        }}

        // Note: This requires server-generated data to use Web Crypto compatible format
        // The Rust side will generate compatible encrypted data

        async function unlock(e) {{
            e.preventDefault();
            const passphrase = document.getElementById('passphrase').value;
            const errorEl = document.getElementById('error');

            try {{
                // Decode the encrypted data
                const salt = Uint8Array.from(atob(ENCRYPTED_DATA.salt), c => c.charCodeAt(0));
                const nonce = Uint8Array.from(atob(ENCRYPTED_DATA.nonce), c => c.charCodeAt(0));
                const ciphertext = Uint8Array.from(atob(ENCRYPTED_DATA.ciphertext), c => c.charCodeAt(0));

                // For now, show error - full implementation needs Web Crypto compatible encryption
                // This is a placeholder that will be updated with proper browser decryption
                errorEl.textContent = 'Decryption in progress...';
                errorEl.style.display = 'block';

                // TODO: Implement browser-compatible decryption
                // The Rust backend will use a Web Crypto compatible approach

            }} catch (err) {{
                errorEl.textContent = 'Incorrect passphrase. Please try again.';
                errorEl.style.display = 'block';
            }}
        }}

        function renderDocument(data) {{
            // Document rendering logic will go here
            const container = document.getElementById('documentContent');
            container.innerHTML = '<p>Document loaded successfully</p>';
            document.getElementById('lockScreen').style.display = 'none';
            document.getElementById('content').classList.add('visible');
        }}

        function search(term) {{
            // Search implementation
            const content = document.getElementById('documentContent');
            // Remove existing highlights
            content.innerHTML = content.innerHTML.replace(/<mark class="highlight">(.*?)<\/mark>/g, '$1');
            if (term) {{
                const regex = new RegExp(`(${{term}})`, 'gi');
                content.innerHTML = content.innerHTML.replace(regex, '<mark class="highlight">$1</mark>');
            }}
        }}
    </script>
</body>
</html>"#,
        creator_name = creator_name,
        encrypted_data = encrypted_data
    )
}
```

**Step 2: Add module to main.rs**

Add to top of `src-tauri/src/main.rs`:

```rust
mod export;
```

**Step 3: Verify it compiles**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 4: Commit**

```bash
git add src-tauri/src/export.rs src-tauri/src/main.rs
git commit -m "feat: add HTML export module with encryption"
```

---

## Phase 6: Rust Backend - Tauri Commands

### Task 6.1: Create Tauri Commands

**Files:**
- Modify: `src-tauri/src/main.rs`

**Step 1: Add Tauri command handlers**

Replace contents of `src-tauri/src/main.rs` with:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod encryption;
mod export;
mod models;
mod storage;

use models::LegacyDocument;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    document: Mutex<LegacyDocument>,
}

#[tauri::command]
fn get_document(state: State<AppState>) -> Result<LegacyDocument, String> {
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    Ok(doc.clone())
}

#[tauri::command]
fn update_document(state: State<AppState>, document: LegacyDocument) -> Result<(), String> {
    let mut doc = state.document.lock().map_err(|e| e.to_string())?;
    *doc = document;
    storage::save_document(&doc).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn export_html(state: State<AppState>, passphrase: String) -> Result<String, String> {
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    export::generate_encrypted_html(&doc, &passphrase).map_err(|e| e.to_string())
}

#[tauri::command]
fn import_file(encrypted_html: String, passphrase: String) -> Result<LegacyDocument, String> {
    // Extract encrypted data from HTML and decrypt
    // This is a simplified version - full implementation would parse the HTML
    Err("Import not yet implemented".to_string())
}

#[tauri::command]
fn generate_passphrase() -> String {
    use rand::seq::SliceRandom;

    let words = vec![
        "apple", "banana", "cherry", "dragon", "eagle", "forest", "garden", "harbor",
        "island", "jungle", "kitchen", "lemon", "mountain", "nectar", "ocean", "piano",
        "quartz", "river", "sunset", "thunder", "umbrella", "violet", "window", "yellow",
        "zebra", "anchor", "beacon", "castle", "diamond", "ember", "falcon", "glacier",
    ];

    let mut rng = rand::thread_rng();
    let selected: Vec<&str> = words.choose_multiple(&mut rng, 4).cloned().collect();
    selected.join("-")
}

fn main() {
    // Try to load existing document, or create new one
    let document = storage::load_document()
        .ok()
        .flatten()
        .unwrap_or_default();

    tauri::Builder::default()
        .manage(AppState {
            document: Mutex::new(document),
        })
        .invoke_handler(tauri::generate_handler![
            get_document,
            update_document,
            export_html,
            import_file,
            generate_passphrase,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 2: Verify it compiles**

Run:
```bash
cd src-tauri && cargo check
```

Expected: Compiles without errors

**Step 3: Commit**

```bash
git add src-tauri/src/main.rs
git commit -m "feat: add Tauri command handlers for document operations"
```

---

## Phase 7: Svelte Frontend - App Shell

### Task 7.1: Create App Shell with Navigation

**Files:**
- Modify: `src/App.svelte`
- Create: `src/lib/stores/document.ts`

**Step 1: Create document store**

Create `src/lib/stores/document.ts`:

```typescript
import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface LegacyDocument {
  meta: DocumentMeta;
  financial: FinancialSection;
  insurance: InsuranceSection;
  bills: BillsSection;
  property: PropertySection;
  legal: LegalSection;
  digital: DigitalSection;
  household: HouseholdSection;
  personal: PersonalSection;
  contacts: ContactsSection;
  medical: MedicalSection;
  pets: PetsSection;
}

export interface DocumentMeta {
  creator_name: string;
  created_at: string;
  updated_at: string;
}

// Simplified interfaces - full types match Rust models
export interface FinancialSection {
  bank_accounts: any[];
  credit_cards: any[];
  investments: any[];
  debts: any[];
  notes: string;
}

export interface InsuranceSection {
  policies: any[];
  notes: string;
}

export interface BillsSection {
  bills: any[];
  notes: string;
}

export interface PropertySection {
  properties: any[];
  vehicles: any[];
  valuables: any[];
  notes: string;
}

export interface LegalSection {
  will_location: string;
  attorney: any;
  power_of_attorney: string;
  trusts: any[];
  notes: string;
}

export interface DigitalSection {
  email_accounts: any[];
  social_media: any[];
  password_manager: any;
  notes: string;
}

export interface HouseholdSection {
  maintenance_items: any[];
  contractors: any[];
  how_things_work: any[];
  notes: string;
}

export interface PersonalSection {
  funeral_preferences: string;
  obituary_notes: string;
  messages: any[];
  notes: string;
}

export interface ContactsSection {
  emergency_contacts: any[];
  family: any[];
  professionals: any[];
  notes: string;
}

export interface MedicalSection {
  family_members: any[];
  notes: string;
}

export interface PetsSection {
  pets: any[];
  notes: string;
}

function createDocumentStore() {
  const { subscribe, set, update } = writable<LegacyDocument | null>(null);

  return {
    subscribe,
    load: async () => {
      try {
        const doc = await invoke<LegacyDocument>('get_document');
        set(doc);
      } catch (e) {
        console.error('Failed to load document:', e);
      }
    },
    save: async (doc: LegacyDocument) => {
      try {
        await invoke('update_document', { document: doc });
        set(doc);
      } catch (e) {
        console.error('Failed to save document:', e);
      }
    },
    updateSection: async <K extends keyof LegacyDocument>(
      section: K,
      data: LegacyDocument[K]
    ) => {
      update((doc) => {
        if (doc) {
          const updated = { ...doc, [section]: data };
          invoke('update_document', { document: updated }).catch(console.error);
          return updated;
        }
        return doc;
      });
    },
  };
}

export const document = createDocumentStore();
```

**Step 2: Update App.svelte**

Replace contents of `src/App.svelte`:

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { document } from './lib/stores/document';

  type Section =
    | 'financial' | 'insurance' | 'bills' | 'property' | 'legal'
    | 'digital' | 'household' | 'personal' | 'contacts' | 'medical' | 'pets';

  let currentSection: Section = 'financial';
  let showExportDialog = false;

  const sections: { id: Section; label: string; icon: string }[] = [
    { id: 'financial', label: 'Financial', icon: '💰' },
    { id: 'insurance', label: 'Insurance', icon: '🛡️' },
    { id: 'bills', label: 'Bills', icon: '📄' },
    { id: 'property', label: 'Property', icon: '🏠' },
    { id: 'legal', label: 'Legal', icon: '⚖️' },
    { id: 'digital', label: 'Digital Life', icon: '💻' },
    { id: 'household', label: 'Household', icon: '🔧' },
    { id: 'personal', label: 'Personal', icon: '💝' },
    { id: 'contacts', label: 'Contacts', icon: '📞' },
    { id: 'medical', label: 'Medical', icon: '🏥' },
    { id: 'pets', label: 'Pets', icon: '🐾' },
  ];

  function getSectionStatus(sectionId: Section): 'empty' | 'partial' | 'complete' {
    // TODO: Implement actual status checking
    return 'empty';
  }

  onMount(() => {
    document.load();
  });
</script>

<main class="app">
  <aside class="sidebar">
    <div class="logo">
      <h1>honey-did</h1>
    </div>
    <nav class="nav">
      {#each sections as section}
        <button
          class="nav-item"
          class:active={currentSection === section.id}
          on:click={() => (currentSection = section.id)}
        >
          <span class="nav-icon">{section.icon}</span>
          <span class="nav-label">{section.label}</span>
          <span class="nav-status" data-status={getSectionStatus(section.id)}></span>
        </button>
      {/each}
    </nav>
    <div class="sidebar-footer">
      <button class="btn btn-secondary" on:click={() => {}}>
        Import File
      </button>
      <button class="btn btn-primary" on:click={() => (showExportDialog = true)}>
        Export
      </button>
    </div>
  </aside>
  <section class="content">
    <header class="content-header">
      <h2>{sections.find((s) => s.id === currentSection)?.label}</h2>
    </header>
    <div class="content-body">
      <!-- Section content will go here -->
      <p>Section content for {currentSection}</p>
    </div>
  </section>
</main>

<style>
  .app {
    display: flex;
    height: 100vh;
    background: #f5f5f5;
  }

  .sidebar {
    width: 240px;
    background: white;
    border-right: 1px solid #e0e0e0;
    display: flex;
    flex-direction: column;
  }

  .logo {
    padding: 20px;
    border-bottom: 1px solid #e0e0e0;
  }

  .logo h1 {
    margin: 0;
    font-size: 1.5rem;
    color: #333;
  }

  .nav {
    flex: 1;
    overflow-y: auto;
    padding: 10px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 12px;
    border: none;
    background: none;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    gap: 12px;
  }

  .nav-item:hover {
    background: #f0f0f0;
  }

  .nav-item.active {
    background: #e3f2fd;
    color: #1976d2;
  }

  .nav-icon {
    font-size: 1.25rem;
  }

  .nav-label {
    flex: 1;
  }

  .nav-status {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: #e0e0e0;
  }

  .nav-status[data-status='partial'] {
    background: #ffc107;
  }

  .nav-status[data-status='complete'] {
    background: #4caf50;
  }

  .sidebar-footer {
    padding: 15px;
    border-top: 1px solid #e0e0e0;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .btn {
    padding: 10px 16px;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .btn-primary {
    background: #1976d2;
    color: white;
  }

  .btn-primary:hover {
    background: #1565c0;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .content-header {
    padding: 20px;
    background: white;
    border-bottom: 1px solid #e0e0e0;
  }

  .content-header h2 {
    margin: 0;
  }

  .content-body {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
  }
</style>
```

**Step 3: Create lib directory structure**

Run:
```bash
mkdir -p src/lib/stores src/lib/components src/lib/sections
```

**Step 4: Verify frontend builds**

Run:
```bash
npm run build
```

Expected: Build succeeds

**Step 5: Commit**

```bash
git add -A
git commit -m "feat: add app shell with sidebar navigation"
```

---

## Phase 8: Continue Implementation

The remaining phases follow the same pattern:

- **Phase 8**: Create section components (Financial, Insurance, etc.)
- **Phase 9**: Create export dialog component
- **Phase 10**: Implement print functionality
- **Phase 11**: Browser-compatible encryption for HTML output
- **Phase 12**: Import functionality
- **Phase 13**: Integration testing

Each phase should be broken into similarly granular tasks (2-5 minutes each).

---

## Next Steps

After completing Phase 7, continue with:

1. Create the `FinancialSection.svelte` component as a template
2. Replicate the pattern for other sections
3. Build the export dialog
4. Test end-to-end flow

The core architecture is in place after Phase 7 — remaining work is UI implementation and polish.
