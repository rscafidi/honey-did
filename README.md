# Honey Did

A secure, cross-platform legacy document app. Honey Did helps you organize critical life information -- financial accounts, insurance policies, legal documents, medical records, contacts, and more -- so your loved ones have what they need in your absence.

When you're ready, export everything into a single encrypted HTML file that anyone can open in a browser. No special software required.

> NOTE: This application was partially generated with the help of Claude Code

## Features

- **11 built-in sections** -- Financial, Insurance, Bills, Property, Legal, Digital Life, Household, Personal, Contacts, Medical, Pets
- **Custom sections** -- Create your own sections with a drag-and-drop form builder (text, number, date, yes/no fields, dividers, headers)
- **Custom subsections** -- Add custom subsections to any built-in category with flexible field definitions
- **File attachments** -- Attach PDFs, images, documents, and spreadsheets to any subsection (10 MB per file limit)
- **Encrypted export** -- AES-256-GCM encryption with PBKDF2 key derivation (600,000 iterations); the exported HTML file decrypts itself in any modern browser via Web Crypto API
- **Question-based unlock** -- Protect exports with personal security questions instead of (or in addition to) a passphrase
- **Welcome screen** -- Add message slides and security questions that display before the recipient unlocks the document
- **Search** -- Full-text search in exported documents with exact, contains, spelling, and phonetic matching
- **Print support** -- Clean print layout with proper page breaks for exported documents
- **App password** -- Lock the desktop app with a password; auto-locks after 1 hour of inactivity
- **Local encryption** -- Data at rest encrypted with Argon2id (64 MB memory cost) + AES-256-GCM using OS keyring
- **Dark mode** -- Light, dark, and auto theme support
- **Guided wizard** -- Step-by-step onboarding for first-time users
- **Cross-platform** -- Windows, macOS, Linux (desktop), Android (mobile), and web browser

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | Svelte 5, TypeScript, Vite 8 |
| Desktop | Tauri 2 (Rust) |
| Encryption | ring (AES-256-GCM, PBKDF2), argon2 (Argon2id) |
| Key storage | OS keyring (desktop), app-private dir (Android) |
| Testing | Vitest, Testing Library, Playwright |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (stable toolchain)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform

### Install

```bash
npm install
```

### Development

```bash
# Frontend only (web mode, no Tauri backend)
npm run dev

# Desktop app (Tauri + Vite)
npm run tauri dev
```

In dev mode, a **Fill Test Data** button appears in the sidebar to populate all sections with realistic sample data for testing exports.

### Build

```bash
# Frontend only
npm run build

# Desktop app
npm run tauri build
```

## Testing

### Unit & Component Tests

Uses Vitest + Testing Library + jsdom.

```bash
# Run all unit tests
npm test

# Watch mode
npm run test:watch

# With coverage report
npm run test:coverage
```

**Coverage includes:**
- Document store utilities (`isDocumentEmpty`, `getFieldElements`, `migrateSubsection`, `debounce`)
- Theme store (preference cycling, localStorage persistence, derived resolution)
- UI components (FormField, FormBuilder, FormPreview, ItemCard, AddButton, NotesField)
- Data operations for all 11 built-in sections, custom sections, and full document assembly

### E2E Tests

Uses Playwright against the Vite dev server.

```bash
# Run E2E tests (starts dev server automatically)
npm run test:e2e

# Interactive UI mode
npm run test:e2e:ui
```

**Coverage includes:**
- Section navigation across all built-in sections
- Adding data to every form/field type
- Data persistence via localStorage
- Export dialog flow and passphrase validation
- Full workflow: populating all sections end-to-end

## Project Structure

```
src/
  App.svelte                    # Root component, navigation, modals
  main.ts                       # Entry point
  lib/
    components/                 # Reusable UI components
      FileAttachments.svelte    #   File attachment picker + list
      FormField.svelte          #   Text / textarea / checkbox input
      FormBuilder.svelte        #   Drag-and-drop form element editor
      FormPreview.svelte        #   Custom form item display + editing
      CustomSectionEditor.svelte
      CustomSubsections.svelte
      ItemCard.svelte
      AddButton.svelte
      NotesField.svelte
      ExportDialog.svelte
      ImportDialog.svelte
      SettingsModal.svelte
      HelpModal.svelte
      LockScreen.svelte
      SetPasswordModal.svelte
    sections/                   # One page component per section
    stores/
      document.ts               # Document state, section stores, types
      theme.ts                  # Theme preference store
    wizard/                     # Guided setup wizard
    testData.ts                 # Dev-only test data generator
  test/
    setup.ts                    # Vitest setup (Tauri mocks, jsdom config)

src-tauri/
  src/
    lib.rs                      # Tauri commands + input validation
    models.rs                   # Data models (mirrors TypeScript types)
    encryption.rs               # AES-256-GCM, PBKDF2, Argon2id
    export.rs                   # Encrypted HTML export generation
    storage.rs                  # File I/O + keyring integration
  capabilities/default.json     # Tauri permissions

tests/e2e/
  app.spec.ts                   # Playwright E2E tests
```

## Security

- **Export encryption**: PBKDF2-HMAC-SHA256 (600,000 iterations) + AES-256-GCM
- **Local encryption**: Argon2id (64 MB memory, 3 iterations) + AES-256-GCM
- Random 16-byte salt + 12-byte nonce per encryption operation
- Passphrases are never stored
- OS keyring for local key storage (desktop); app-private directory (Android)
- 1-hour inactivity auto-lock with optional clear-on-exit
- Question-based export uses dual-key encryption: document key encrypted with both question-answer key and optional fallback passphrase

## Export Format

Exported documents are self-contained HTML files:

- Decrypts entirely client-side using Web Crypto API -- no server, no dependencies
- Responsive sidebar layout with section navigation
- Full-text search with fuzzy/phonetic matching
- File attachments embedded as base64 with blob-based download
- Clean print stylesheet with proper page breaks

## Supported Attachment Types

| Category | Extensions |
|----------|-----------|
| Images | PNG, JPG, JPEG, GIF, WEBP |
| Documents | PDF, DOC, DOCX, TXT |
| Spreadsheets | XLS, XLSX, CSV |

Maximum 10 MB per file.

## License

[MIT](LICENSE)
