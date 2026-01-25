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
