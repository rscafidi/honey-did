# honey-did Design Document

A cross-platform desktop application that guides users through creating a secure "legacy document" containing critical information their spouse or family would need in the event of their death.

## Overview

**Problem:** When a spouse dies unexpectedly, the surviving partner often lacks critical information — bank accounts, insurance policies, how to pay bills, medical details, household maintenance knowledge. This information exists only in the deceased's head.

**Solution:** A guided wizard that helps users document everything their spouse would need to know, packaged into a secure, portable, easy-to-open file.

## Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Spouse A uses  │ ──▶ │  Fills out info │ ──▶ │  Exports secure │
│  desktop app    │     │  via wizard     │     │  HTML file      │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                        │
                              ┌─────────────────────────┘
                              ▼
                        ┌─────────────────┐
                        │  Spouse B opens │
                        │  in any browser │
                        └─────────────────┘
```

### Tech Stack

- **Tauri 2.0** — Rust backend, native webview frontend
- **Svelte** — Wizard UI framework
- **Rust crypto (ring or RustCrypto)** — AES-256-GCM encryption
- **Argon2id** — Password key derivation
- **Output** — Self-contained HTML file with embedded encrypted data

## Information Categories

| Category | Example Fields |
|----------|----------------|
| **Financial** | Bank accounts, credit cards, investments, debts, safe deposit boxes |
| **Insurance** | Life, health, home, auto, long-term care — policy numbers, contacts, locations |
| **Bills & Subscriptions** | Recurring payments, due dates, autopay status, login hints |
| **Property & Assets** | Home details, vehicles, valuables, storage units, document locations |
| **Legal** | Will location, attorney contact, power of attorney, trusts |
| **Digital Life** | Email accounts, social media, password manager master hint, 2FA recovery |
| **Household** | Maintenance schedules, contractors, "how things work" notes |
| **Personal Wishes** | Funeral preferences, obituary notes, messages to loved ones |
| **Emergency Contacts** | Family, friends, doctors, professionals — who to call for what |
| **Medical** | Family members' doctors, medications, conditions, allergies, pharmacy |
| **Pets** | Vet info, medications, feeding routines, care instructions, emergency contacts |

Each section includes:
- Guided prompts (e.g., "Do you have a safe deposit box?")
- Free-form notes field for anything that doesn't fit the structure
- Visual indicator showing completion status

## User Interface (Desktop App)

### Main Wizard Layout

```
┌─────────────────────────────────────────────────────────────┐
│  honey-did                                        [—][□][×] │
├──────────────────┬──────────────────────────────────────────┤
│                  │                                          │
│  ○ Financial     │   Bank Accounts                          │
│  ◐ Insurance     │   ─────────────────────────────────────  │
│  ● Bills         │                                          │
│  ○ Property      │   [+ Add Account]                        │
│  ○ Legal         │                                          │
│  ○ Digital Life  │   ┌─────────────────────────────────┐    │
│  ○ Household     │   │ Bank: Chase Checking            │    │
│  ○ Personal      │   │ Account #: ****4521             │    │
│  ○ Contacts      │   │ Notes: Joint account, auto...   │    │
│  ○ Medical       │   └─────────────────────────────────┘    │
│  ○ Pets          │                                          │
│                  │   Section notes:                         │
│──────────────────│   [                                   ]  │
│ [Import File]    │                                          │
│ [Export]         │                                          │
└──────────────────┴──────────────────────────────────────────┘

○ = not started   ◐ = in progress   ● = complete
```

### Key Behaviors

- **Auto-save** — Changes persist automatically to local encrypted storage
- **Flexible navigation** — Click any section in sidebar, complete in any order
- **Progress indicators** — Visual cues show which sections are done
- **Import** — Load a previously exported file to edit (requires password)
- **Export** — Generate the encrypted HTML (and optional print)

## Export Flow

### Export Dialog

```
┌─────────────────────────────────────────────┐
│  Create Your Secure File                    │
├─────────────────────────────────────────────┤
│                                             │
│  Choose a passphrase:                       │
│  ┌─────────────────────────────────────┐    │
│  │ gentle-river-climbing-tuesday       │    │
│  └─────────────────────────────────────┘    │
│  [Generate New Suggestion]                  │
│                                             │
│  Confirm passphrase:                        │
│  ┌─────────────────────────────────────┐    │
│  │ ••••••••••••••••••••••••••••        │    │
│  └─────────────────────────────────────┘    │
│                                             │
│  Strength: ████████░░ Strong                │
│                                             │
│  ☑ Also print a physical copy               │
│    ⚠️  Printed copies can be found by       │
│       anyone. Store securely.               │
│                                             │
│  [Cancel]                    [Export File]  │
└─────────────────────────────────────────────┘
```

### Encryption Details

- **Algorithm**: AES-256-GCM (authenticated encryption)
- **Key derivation**: Argon2id (memory-hard, resistant to brute force)
- **Performed by**: Rust backend (not JavaScript)
- **Output**: Single `.html` file containing encrypted payload + decryption UI

### Print Option

- Direct print to system printer via Tauri print dialog
- No unencrypted file ever saved to disk
- Warning displayed about physical security of printed copies

## Output File (Spouse B's Experience)

### Password Prompt

```
┌─────────────────────────────────────────────────────────────┐
│  🔒 honey-did                                               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│           This file was created by [Name]                   │
│           to help you in their absence.                     │
│                                                             │
│           Enter the passphrase to unlock:                   │
│           ┌─────────────────────────────────────┐           │
│           │                                     │           │
│           └─────────────────────────────────────┘           │
│                        [Unlock]                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Unlocked View

```
┌─────────────────────────────────────────────────────────────┐
│  honey-did                              [🔍 Search] [Print] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  TABLE OF CONTENTS                                          │
│  ─────────────────                                          │
│  • Financial                                                │
│  • Insurance                                                │
│  • Bills & Subscriptions                                    │
│  ... (clickable links)                                      │
│                                                             │
│  ═══════════════════════════════════════════════════════    │
│                                                             │
│  FINANCIAL                                                  │
│  ─────────                                                  │
│  Bank Accounts                                              │
│  ┌────────────────────────────────────────────────────┐     │
│  │ Chase Checking - ****4521                          │     │
│  │ Joint account, autopay for mortgage                │     │
│  └────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### Features

- **Table of contents** — Jump to any section
- **Search** — Find specific info quickly
- **Print button** — Print directly from browser
- **Responsive layout** — Works on any screen size
- **Fully offline** — No internet required

## Security Model

### Data at Rest

- Working data stored in app data directory
- Encrypted with local key from OS secure storage (Tauri credential store)
- Never stored in plain text

### Data in Transit

- None — app is fully offline, no network calls

### Exported File

- AES-256-GCM encryption (authenticated, tamper-evident)
- Argon2id key derivation (100ms+ computation, GPU-resistant)
- Passphrase never stored

### Threat Model

| Threat | Protected? |
|--------|------------|
| Someone finds HTML file without passphrase | ✅ Yes |
| Stolen computer while app is closed | ✅ Yes |
| Brute-force password guessing | ✅ Yes (Argon2id) |
| Access to unlocked computer with app open | ⚠️ Partial |
| Nation-state attacker with physical access | ❌ Out of scope |

### Out of Scope

- Physical security of printed copies
- Passphrase shared insecurely
- User choosing weak passphrase

## Project Structure

```
honey-did/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # App entry point
│   │   ├── encryption.rs   # AES-256-GCM + Argon2id
│   │   ├── storage.rs      # Local encrypted storage
│   │   ├── export.rs       # HTML file generation
│   │   └── print.rs        # System print dialog
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/     # Reusable UI components
│   │   ├── sections/       # Category components
│   │   └── stores/         # Svelte stores
│   ├── App.svelte
│   └── main.ts
│
├── templates/
│   └── output.html         # Encrypted output template
│
├── package.json
└── README.md
```

### Key Dependencies

**Rust:**
- `tauri`
- `ring` or `rust-crypto`
- `argon2`
- `serde`

**Frontend:**
- `svelte`
- `@tauri-apps/api`

### Build Outputs

- Windows: `.msi` installer
- macOS: `.dmg`
- Linux: `.AppImage` or `.deb`
