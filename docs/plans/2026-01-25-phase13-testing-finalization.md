# Phase 13: Integration Testing & Finalization

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete the honey-did app by fixing the raw string bug, adding integration tests for export/import, and verifying end-to-end functionality.

**Architecture:** Test-driven approach focusing on Rust backend tests for encryption roundtrip, export/import cycle, and storage operations. Frontend testing deferred (no test framework currently configured).

**Tech Stack:** Rust unit tests with `cargo test`, manual verification for Svelte UI.

---

## Task 1: Fix Raw String Delimiter Bug

**Files:**
- Modify: `/workspace/honey-did/src-tauri/src/export.rs:84` and `:557`

The HTML template raw string uses `r#"..."#` but contains `href="#"` patterns that prematurely terminate it.

**Step 1: Verify the bug exists**

Open the file and confirm lines 84 and 557 use `r#` and `"#` delimiters:
```bash
grep -n 'r#"<!DOCTYPE' src-tauri/src/export.rs
grep -n '</html>"#' src-tauri/src/export.rs
```

Expected output should show the old delimiters still in place or already fixed with `r##`.

**Step 2: Update opening delimiter (if not already done)**

Change line 84 from:
```rust
        r#"<!DOCTYPE html>
```
to:
```rust
        r##"<!DOCTYPE html>
```

**Step 3: Update closing delimiter (if not already done)**

Change line ~557 from:
```rust
</html>"#,
```
to:
```rust
</html>"##,
```

**Step 4: Verify syntax is correct**

Run:
```bash
cd /workspace/honey-did/src-tauri && cargo check 2>&1 | head -50
```

Expected: No "prefix `detail` is unknown" errors. May fail on network (can't download `regex` crate) but should not have lexer errors.

**Step 5: Commit**

```bash
cd /workspace/honey-did
git add src-tauri/src/export.rs
git commit -m "fix: use r##\"...\"## delimiter for HTML template with href=\"#\" links"
```

---

## Task 2: Add Export/Import Roundtrip Test

**Files:**
- Modify: `/workspace/honey-did/src-tauri/src/export.rs` (add tests at end of file)

**Step 1: Write the failing test**

Add at the end of `/workspace/honey-did/src-tauri/src/export.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    fn create_test_document() -> LegacyDocument {
        LegacyDocument {
            meta: DocumentMeta {
                creator_name: "Test User".to_string(),
                created_at: "2026-01-25".to_string(),
                updated_at: "2026-01-25".to_string(),
            },
            financial: FinancialSection {
                bank_accounts: vec![BankAccount {
                    name: "Checking Account".to_string(),
                    institution: "Test Bank".to_string(),
                    account_type: "Checking".to_string(),
                    last_four: "1234".to_string(),
                    notes: "Primary account".to_string(),
                }],
                credit_cards: vec![],
                investments: vec![],
                debts: vec![],
                notes: "Financial notes".to_string(),
            },
            insurance: InsuranceSection::default(),
            bills: BillsSection::default(),
            property: PropertySection::default(),
            legal: LegalSection::default(),
            digital: DigitalSection::default(),
            household: HouseholdSection::default(),
            personal: PersonalSection::default(),
            contacts: ContactsSection::default(),
            medical: MedicalSection::default(),
            pets: PetsSection::default(),
        }
    }

    #[test]
    fn test_export_import_roundtrip() {
        let original = create_test_document();
        let passphrase = "test-passphrase-123";

        // Export to HTML
        let html = generate_encrypted_html(&original, passphrase)
            .expect("export should succeed");

        // Verify HTML contains expected structure
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("ENCRYPTED_DATA"));
        assert!(html.contains("Test User"));

        // Import back from HTML
        let imported = import_from_html(&html, passphrase)
            .expect("import should succeed");

        // Verify roundtrip preserves data
        assert_eq!(imported.meta.creator_name, original.meta.creator_name);
        assert_eq!(imported.financial.bank_accounts.len(), 1);
        assert_eq!(imported.financial.bank_accounts[0].name, "Checking Account");
        assert_eq!(imported.financial.bank_accounts[0].institution, "Test Bank");
        assert_eq!(imported.financial.notes, "Financial notes");
    }

    #[test]
    fn test_import_wrong_passphrase_fails() {
        let doc = create_test_document();
        let html = generate_encrypted_html(&doc, "correct-passphrase")
            .expect("export should succeed");

        let result = import_from_html(&html, "wrong-passphrase");
        assert!(result.is_err());
    }

    #[test]
    fn test_import_invalid_html_fails() {
        let result = import_from_html("<html><body>No encrypted data</body></html>", "any");
        assert!(result.is_err());
    }

    #[test]
    fn test_print_html_generation() {
        let doc = create_test_document();
        let html = generate_print_html(&doc);

        // Verify print HTML structure
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test User"));
        assert!(html.contains("Checking Account"));
        assert!(html.contains("Test Bank"));
        assert!(!html.contains("ENCRYPTED_DATA")); // Print version is unencrypted
    }
}
```

**Step 2: Run tests to verify they work (when network available)**

Run:
```bash
cd /workspace/honey-did/src-tauri && cargo test export::tests --no-fail-fast 2>&1
```

Expected: All 4 tests pass (or network error if crates.io unreachable)

**Step 3: Commit**

```bash
cd /workspace/honey-did
git add src-tauri/src/export.rs
git commit -m "test: add export/import roundtrip tests"
```

---

## Task 3: Add Storage Module Tests

**Files:**
- Modify: `/workspace/honey-did/src-tauri/src/storage.rs` (add tests at end)

**Step 1: Read current storage.rs structure**

```bash
cat /workspace/honey-did/src-tauri/src/storage.rs
```

Understand the current implementation before adding tests.

**Step 2: Write storage tests**

Add at the end of `/workspace/honey-did/src-tauri/src/storage.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_get_data_dir_exists() {
        let dir = get_data_dir();
        // Should return a path (may not exist yet, but path should be valid)
        assert!(dir.to_string_lossy().contains("honey-did"));
    }

    #[test]
    fn test_save_and_load_document() {
        let mut doc = LegacyDocument::default();
        doc.meta.creator_name = "Storage Test User".to_string();
        doc.financial.notes = "Storage test notes".to_string();

        // Save
        save_document(&doc).expect("save should succeed");

        // Load
        let loaded = load_document()
            .expect("load should succeed")
            .expect("document should exist");

        assert_eq!(loaded.meta.creator_name, "Storage Test User");
        assert_eq!(loaded.financial.notes, "Storage test notes");
    }
}
```

**Step 3: Run tests**

Run:
```bash
cd /workspace/honey-did/src-tauri && cargo test storage::tests --no-fail-fast 2>&1
```

Expected: Tests pass (keyring may need special handling in CI)

**Step 4: Commit**

```bash
cd /workspace/honey-did
git add src-tauri/src/storage.rs
git commit -m "test: add storage module tests"
```

---

## Task 4: Add Browser-Compatible Encryption Test

**Files:**
- Modify: `/workspace/honey-did/src-tauri/src/encryption.rs` (add to existing tests)

**Step 1: Add PBKDF2 encryption test**

Add to the existing `mod tests` block in encryption.rs:

```rust
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
```

**Step 2: Run encryption tests**

Run:
```bash
cd /workspace/honey-did/src-tauri && cargo test encryption::tests --no-fail-fast 2>&1
```

Expected: All 8 tests pass (5 existing + 3 new)

**Step 3: Commit**

```bash
cd /workspace/honey-did
git add src-tauri/src/encryption.rs
git commit -m "test: add browser-compatible encryption tests"
```

---

## Task 5: Verify Frontend Builds

**Files:**
- Check: `/workspace/honey-did/package.json`
- Check: `/workspace/honey-did/src/` (all .svelte and .ts files)

**Step 1: Run frontend build**

Run:
```bash
cd /workspace/honey-did && npm run build 2>&1
```

Expected: Build succeeds with output like:
```
dist/index.html                   0.46 kB
dist/assets/index-*.css          12.11 kB
dist/assets/index-*.js          114.55 kB
```

**Step 2: Run TypeScript check (if available)**

Run:
```bash
cd /workspace/honey-did && npx svelte-check 2>&1 || echo "svelte-check not configured"
```

Expected: Either passes or indicates not configured

**Step 3: Commit if any fixes needed**

If build fails, fix issues and commit:
```bash
git add -A
git commit -m "fix: resolve frontend build issues"
```

---

## Task 6: Full Integration Verification

**Step 1: Run all Rust tests**

Run:
```bash
cd /workspace/honey-did/src-tauri && cargo test 2>&1
```

Expected: All tests pass:
- encryption::tests (8 tests)
- export::tests (4 tests)
- storage::tests (2 tests)

**Step 2: Document test results**

Create test summary:
```bash
cd /workspace/honey-did/src-tauri && cargo test 2>&1 | tee ../test-results.txt
```

**Step 3: Final commit**

```bash
cd /workspace/honey-did
git add test-results.txt
git commit -m "docs: add test results from integration testing"
```

---

## Task 7: Create README for Project

**Files:**
- Create: `/workspace/honey-did/README.md`

**Step 1: Write README**

Create `/workspace/honey-did/README.md`:

```markdown
# Honey-Did

A secure legacy document creator for recording important information that loved ones will need in your absence.

## Features

- **11 Information Categories**: Financial, Insurance, Bills, Property, Legal, Digital Life, Household, Personal, Contacts, Medical, and Pets
- **Strong Encryption**: AES-256-GCM with PBKDF2 key derivation (600,000 iterations)
- **Self-Contained Export**: Creates a single HTML file that can be opened in any modern browser
- **Browser Decryption**: Exported files decrypt client-side using Web Crypto API
- **Print Support**: Generate print-friendly unencrypted versions
- **Local Storage**: Documents encrypted locally using OS keyring

## Tech Stack

- **Frontend**: Svelte 4 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.0
- **Encryption**: ring (AES-256-GCM), argon2, PBKDF2

## Development

```bash
# Install dependencies
npm install

# Run development server
npm run tauri dev

# Build for production
npm run tauri build
```

## Testing

```bash
# Run Rust tests
cd src-tauri && cargo test

# Build frontend
npm run build
```

## Security

- Passphrases are never stored
- PBKDF2 with 600,000 iterations for exported files
- Argon2id with 64MB memory cost for local storage
- Random salt and nonce for each encryption operation
- AES-256-GCM authenticated encryption

## License

MIT
```

**Step 2: Commit**

```bash
cd /workspace/honey-did
git add README.md
git commit -m "docs: add project README"
```

---

## Summary

| Task | Description | Tests Added |
|------|-------------|-------------|
| 1 | Fix raw string delimiter bug | - |
| 2 | Export/import roundtrip tests | 4 tests |
| 3 | Storage module tests | 2 tests |
| 4 | Browser encryption tests | 3 tests |
| 5 | Frontend build verification | - |
| 6 | Full integration verification | - |
| 7 | Create README | - |

**Total New Tests:** 9 tests
**Total Test Coverage:** 17 tests (8 existing + 9 new)

---

## Notes

- **Network Dependency**: Tasks 2-4, 6 require network access to crates.io to download the `regex` crate
- **Cargo.lock**: If version errors occur, change `version = 4` to `version = 3` in Cargo.lock
- **Keyring Tests**: Storage tests may fail in CI environments without keyring access
