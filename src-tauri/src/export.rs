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
