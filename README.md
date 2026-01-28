# Honey-Did

A secure legacy document creator for recording important information that loved ones will need in your absence.  The exported document is a simple HTML file with encrypted data embedded.  It can be opened on any platform, with any modern browser that supports Web Crypto API.  Secure it with special questions, a passphrase, or both to ensure your loved one is able to open it when needed.

> NOTE: This application was partially generated with the help of Claude Code Opus 4.5

## Features

- **11 Built-in Categories**: Financial, Insurance, Bills, Property, Legal, Digital Life, Household, Personal, Contacts, Medical, and Pets
- **Custom Sections**: Create your own top-level sections with custom subsections and user-defined fields
- **Custom Subsections**: Add custom subsections to any built-in category with flexible field definitions
- **Sidebar Navigation**: Easy navigation between sections with custom sections appearing alongside built-in ones
- **Dark Mode**: Automatic system preference detection or manual light/dark theme selection
- **Guided Setup Wizard**: Step-by-step walkthrough to help populate your document
- **Strong Encryption**: AES-256-GCM with PBKDF2 key derivation (600,000 iterations)
- **Self-Contained Export**: Creates a single HTML file that can be opened in any modern browser
- **Browser Decryption**: Exported files decrypt client-side using Web Crypto API
- **Print Support**: Generate print-friendly unencrypted versions
- **Local Storage**: Documents encrypted locally using OS keyring
- **Password Protection**: Lock app with password, optional clear-on-exit for sensitive environments

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
