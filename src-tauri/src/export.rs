use crate::encryption::{decrypt_from_browser, encrypt_for_browser, EncryptedPayload, EncryptionError};
use crate::models::LegacyDocument;
use regex::Regex;

#[derive(Debug)]
pub enum ExportError {
    EncryptionError(EncryptionError),
    SerializationError(String),
    IoError(String),
    ParseError(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::EncryptionError(e) => write!(f, "Encryption error: {}", e),
            ExportError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ExportError::IoError(msg) => write!(f, "IO error: {}", msg),
            ExportError::ParseError(msg) => write!(f, "Parse error: {}", msg),
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
    let encrypted = encrypt_for_browser(&json, passphrase)?;

    // Serialize encrypted payload
    let encrypted_json = serde_json::to_string(&encrypted)
        .map_err(|e| ExportError::SerializationError(e.to_string()))?;

    // Generate the HTML
    let html = generate_html_template(&encrypted_json, &document.meta.creator_name);

    Ok(html)
}

/// Imports a legacy document from an encrypted HTML file
pub fn import_from_html(html: &str, passphrase: &str) -> Result<LegacyDocument, ExportError> {
    // Extract the encrypted data from the HTML
    // Look for: const ENCRYPTED_DATA = {...};
    let re = Regex::new(r"const\s+ENCRYPTED_DATA\s*=\s*(\{[^}]+\})")
        .map_err(|e| ExportError::ParseError(format!("Regex error: {}", e)))?;

    let captures = re.captures(html)
        .ok_or_else(|| ExportError::ParseError("Could not find encrypted data in HTML file".into()))?;

    let encrypted_json = captures.get(1)
        .ok_or_else(|| ExportError::ParseError("Could not extract encrypted data".into()))?
        .as_str();

    // Parse the encrypted payload
    let payload: EncryptedPayload = serde_json::from_str(encrypted_json)
        .map_err(|e| ExportError::ParseError(format!("Invalid encrypted data format: {}", e)))?;

    // Decrypt the payload
    let decrypted_json = decrypt_from_browser(&payload, passphrase)?;

    // Parse the decrypted JSON into a LegacyDocument
    let document: LegacyDocument = serde_json::from_str(&decrypted_json)
        .map_err(|e| ExportError::SerializationError(format!("Invalid document format: {}", e)))?;

    Ok(document)
}

fn generate_html_template(encrypted_data: &str, creator_name: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
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
        const PBKDF2_ITERATIONS = 600000;

        async function deriveKey(passphrase, salt) {{
            const encoder = new TextEncoder();
            const keyMaterial = await crypto.subtle.importKey(
                'raw', encoder.encode(passphrase), 'PBKDF2', false, ['deriveKey']
            );
            return await crypto.subtle.deriveKey(
                {{ name: 'PBKDF2', salt: salt, iterations: PBKDF2_ITERATIONS, hash: 'SHA-256' }},
                keyMaterial,
                {{ name: 'AES-GCM', length: 256 }},
                false,
                ['decrypt']
            );
        }}

        async function unlock(e) {{
            e.preventDefault();
            const passphrase = document.getElementById('passphrase').value;
            const errorEl = document.getElementById('error');
            const unlockBtn = document.querySelector('.unlock-btn');

            if (!passphrase) {{
                errorEl.textContent = 'Please enter a passphrase.';
                errorEl.style.display = 'block';
                return false;
            }}

            unlockBtn.textContent = 'Decrypting...';
            unlockBtn.disabled = true;
            errorEl.style.display = 'none';

            try {{
                // Decode the base64 encrypted data
                const salt = Uint8Array.from(atob(ENCRYPTED_DATA.salt), c => c.charCodeAt(0));
                const nonce = Uint8Array.from(atob(ENCRYPTED_DATA.nonce), c => c.charCodeAt(0));
                const ciphertext = Uint8Array.from(atob(ENCRYPTED_DATA.ciphertext), c => c.charCodeAt(0));

                // Derive key using PBKDF2
                const key = await deriveKey(passphrase, salt);

                // Decrypt using AES-GCM
                const decrypted = await crypto.subtle.decrypt(
                    {{ name: 'AES-GCM', iv: nonce }},
                    key,
                    ciphertext
                );

                // Decode the decrypted data as UTF-8 JSON
                const decoder = new TextDecoder();
                const jsonString = decoder.decode(decrypted);
                const data = JSON.parse(jsonString);

                // Render the document
                renderDocument(data);

            }} catch (err) {{
                console.error('Decryption failed:', err);
                errorEl.textContent = 'Incorrect passphrase. Please try again.';
                errorEl.style.display = 'block';
                unlockBtn.textContent = 'Unlock';
                unlockBtn.disabled = false;
            }}

            return false;
        }}

        function escapeHtml(text) {{
            if (!text) return '';
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }}

        function renderContact(contact) {{
            if (!contact || !contact.name) return '';
            let html = '<div class="contact-info">';
            if (contact.name) html += '<div><strong>' + escapeHtml(contact.name) + '</strong></div>';
            if (contact.relationship) html += '<div>' + escapeHtml(contact.relationship) + '</div>';
            if (contact.phone) html += '<div>Phone: ' + escapeHtml(contact.phone) + '</div>';
            if (contact.email) html += '<div>Email: ' + escapeHtml(contact.email) + '</div>';
            if (contact.notes) html += '<div class="notes">' + escapeHtml(contact.notes) + '</div>';
            html += '</div>';
            return html;
        }}

        function renderSection(title, id, content) {{
            if (!content) return '';
            return '<div class="section" id="' + id + '"><h2 class="section-title">' + escapeHtml(title) + '</h2>' + content + '</div>';
        }}

        function renderDocument(data) {{
            const container = document.getElementById('documentContent');
            let html = '';

            // Header
            html += '<div class="header"><h1 class="header-title">Legacy Document</h1>';
            if (data.meta && data.meta.creator_name) {{
                html += '<p>Prepared by ' + escapeHtml(data.meta.creator_name) + '</p>';
            }}
            html += '<div class="toolbar">';
            html += '<input type="text" class="search-input" placeholder="Search..." oninput="search(this.value)">';
            html += '<button class="print-btn" onclick="window.print()">Print</button>';
            html += '</div></div>';

            // Table of Contents
            html += '<div class="toc"><div class="toc-title">Contents</div><ul class="toc-list">';
            const sections = ['financial', 'insurance', 'bills', 'property', 'legal', 'digital', 'household', 'personal', 'contacts', 'medical', 'pets'];
            const sectionLabels = {{'financial': 'Financial', 'insurance': 'Insurance', 'bills': 'Bills', 'property': 'Property', 'legal': 'Legal', 'digital': 'Digital Life', 'household': 'Household', 'personal': 'Personal', 'contacts': 'Contacts', 'medical': 'Medical', 'pets': 'Pets'}};
            sections.forEach(s => {{
                html += '<li><a href="#' + s + '">' + sectionLabels[s] + '</a></li>';
            }});
            html += '</ul></div>';

            // Financial Section
            if (data.financial) {{
                let content = '';
                if (data.financial.bank_accounts && data.financial.bank_accounts.length) {{
                    content += '<h3>Bank Accounts</h3>';
                    data.financial.bank_accounts.forEach(a => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(a.name) + '</div>';
                        content += '<div class="item-detail">Institution: ' + escapeHtml(a.institution) + '</div>';
                        content += '<div class="item-detail">Type: ' + escapeHtml(a.account_type) + '</div>';
                        if (a.notes) content += '<div class="notes">' + escapeHtml(a.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.financial.credit_cards && data.financial.credit_cards.length) {{
                    content += '<h3>Credit Cards</h3>';
                    data.financial.credit_cards.forEach(c => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(c.name) + '</div>';
                        content += '<div class="item-detail">Issuer: ' + escapeHtml(c.issuer) + '</div>';
                        if (c.notes) content += '<div class="notes">' + escapeHtml(c.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.financial.investments && data.financial.investments.length) {{
                    content += '<h3>Investments</h3>';
                    data.financial.investments.forEach(i => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(i.name) + '</div>';
                        content += '<div class="item-detail">Type: ' + escapeHtml(i.account_type) + '</div>';
                        content += '<div class="item-detail">Institution: ' + escapeHtml(i.institution) + '</div>';
                        if (i.notes) content += '<div class="notes">' + escapeHtml(i.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.financial.debts && data.financial.debts.length) {{
                    content += '<h3>Debts</h3>';
                    data.financial.debts.forEach(d => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(d.name) + '</div>';
                        content += '<div class="item-detail">Lender: ' + escapeHtml(d.lender) + '</div>';
                        if (d.notes) content += '<div class="notes">' + escapeHtml(d.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.financial.notes) content += '<div class="notes">' + escapeHtml(data.financial.notes) + '</div>';
                html += renderSection('Financial Information', 'financial', content);
            }}

            // Insurance Section
            if (data.insurance && data.insurance.policies && data.insurance.policies.length) {{
                let content = '';
                data.insurance.policies.forEach(p => {{
                    content += '<div class="item"><div class="item-title">' + escapeHtml(p.policy_type) + '</div>';
                    content += '<div class="item-detail">Provider: ' + escapeHtml(p.provider) + '</div>';
                    content += '<div class="item-detail">Policy #: ' + escapeHtml(p.policy_number) + '</div>';
                    if (p.contact) content += '<div class="item-detail">Contact: ' + escapeHtml(p.contact) + '</div>';
                    if (p.notes) content += '<div class="notes">' + escapeHtml(p.notes) + '</div>';
                    content += '</div>';
                }});
                if (data.insurance.notes) content += '<div class="notes">' + escapeHtml(data.insurance.notes) + '</div>';
                html += renderSection('Insurance', 'insurance', content);
            }}

            // Bills Section
            if (data.bills && data.bills.bills && data.bills.bills.length) {{
                let content = '';
                data.bills.bills.forEach(b => {{
                    content += '<div class="item"><div class="item-title">' + escapeHtml(b.name) + '</div>';
                    content += '<div class="item-detail">Provider: ' + escapeHtml(b.provider) + '</div>';
                    content += '<div class="item-detail">Amount: ' + escapeHtml(b.amount) + '</div>';
                    content += '<div class="item-detail">Due Day: ' + escapeHtml(b.due_day) + '</div>';
                    content += '<div class="item-detail">Auto-pay: ' + (b.autopay ? 'Yes' : 'No') + '</div>';
                    if (b.notes) content += '<div class="notes">' + escapeHtml(b.notes) + '</div>';
                    content += '</div>';
                }});
                if (data.bills.notes) content += '<div class="notes">' + escapeHtml(data.bills.notes) + '</div>';
                html += renderSection('Bills', 'bills', content);
            }}

            // Property Section
            if (data.property) {{
                let content = '';
                if (data.property.properties && data.property.properties.length) {{
                    content += '<h3>Properties</h3>';
                    data.property.properties.forEach(p => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(p.name) + '</div>';
                        content += '<div class="item-detail">Address: ' + escapeHtml(p.address) + '</div>';
                        if (p.notes) content += '<div class="notes">' + escapeHtml(p.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.property.vehicles && data.property.vehicles.length) {{
                    content += '<h3>Vehicles</h3>';
                    data.property.vehicles.forEach(v => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(v.name) + '</div>';
                        content += '<div class="item-detail">' + escapeHtml(v.details) + '</div>';
                        if (v.notes) content += '<div class="notes">' + escapeHtml(v.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.property.valuables && data.property.valuables.length) {{
                    content += '<h3>Valuables</h3>';
                    data.property.valuables.forEach(v => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(v.name) + '</div>';
                        content += '<div class="item-detail">Location: ' + escapeHtml(v.location) + '</div>';
                        if (v.notes) content += '<div class="notes">' + escapeHtml(v.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.property.notes) content += '<div class="notes">' + escapeHtml(data.property.notes) + '</div>';
                html += renderSection('Property', 'property', content);
            }}

            // Legal Section
            if (data.legal) {{
                let content = '';
                if (data.legal.will_location) content += '<div class="item-detail"><strong>Will Location:</strong> ' + escapeHtml(data.legal.will_location) + '</div>';
                if (data.legal.power_of_attorney) content += '<div class="item-detail"><strong>Power of Attorney:</strong> ' + escapeHtml(data.legal.power_of_attorney) + '</div>';
                if (data.legal.attorney && data.legal.attorney.name) {{
                    content += '<h3>Attorney</h3>' + renderContact(data.legal.attorney);
                }}
                if (data.legal.trusts && data.legal.trusts.length) {{
                    content += '<h3>Trusts</h3>';
                    data.legal.trusts.forEach(t => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(t.name) + '</div>';
                        content += '<div class="item-detail">Trustee: ' + escapeHtml(t.trustee) + '</div>';
                        if (t.notes) content += '<div class="notes">' + escapeHtml(t.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.legal.notes) content += '<div class="notes">' + escapeHtml(data.legal.notes) + '</div>';
                html += renderSection('Legal Documents', 'legal', content);
            }}

            // Digital Section
            if (data.digital) {{
                let content = '';
                if (data.digital.password_manager && data.digital.password_manager.name) {{
                    content += '<h3>Password Manager</h3>';
                    content += '<div class="item"><div class="item-title">' + escapeHtml(data.digital.password_manager.name) + '</div>';
                    content += '<div class="item-detail">Hint: ' + escapeHtml(data.digital.password_manager.master_password_hint) + '</div>';
                    content += '<div class="item-detail">Recovery: ' + escapeHtml(data.digital.password_manager.recovery_method) + '</div>';
                    content += '</div>';
                }}
                if (data.digital.email_accounts && data.digital.email_accounts.length) {{
                    content += '<h3>Email Accounts</h3>';
                    data.digital.email_accounts.forEach(e => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(e.name) + '</div>';
                        content += '<div class="item-detail">Username: ' + escapeHtml(e.username) + '</div>';
                        if (e.recovery_hint) content += '<div class="item-detail">Recovery: ' + escapeHtml(e.recovery_hint) + '</div>';
                        if (e.notes) content += '<div class="notes">' + escapeHtml(e.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.digital.social_media && data.digital.social_media.length) {{
                    content += '<h3>Social Media</h3>';
                    data.digital.social_media.forEach(s => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(s.name) + '</div>';
                        content += '<div class="item-detail">Username: ' + escapeHtml(s.username) + '</div>';
                        if (s.notes) content += '<div class="notes">' + escapeHtml(s.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.digital.notes) content += '<div class="notes">' + escapeHtml(data.digital.notes) + '</div>';
                html += renderSection('Digital Life', 'digital', content);
            }}

            // Household Section
            if (data.household) {{
                let content = '';
                if (data.household.maintenance_items && data.household.maintenance_items.length) {{
                    content += '<h3>Maintenance</h3>';
                    data.household.maintenance_items.forEach(m => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(m.name) + '</div>';
                        content += '<div class="item-detail">Frequency: ' + escapeHtml(m.frequency) + '</div>';
                        content += '<div class="item-detail">Last Done: ' + escapeHtml(m.last_done) + '</div>';
                        if (m.notes) content += '<div class="notes">' + escapeHtml(m.notes) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.household.contractors && data.household.contractors.length) {{
                    content += '<h3>Contractors</h3>';
                    data.household.contractors.forEach(c => {{
                        content += '<div class="item">' + renderContact(c) + '</div>';
                    }});
                }}
                if (data.household.how_things_work && data.household.how_things_work.length) {{
                    content += '<h3>How Things Work</h3>';
                    data.household.how_things_work.forEach(h => {{
                        content += '<div class="item"><div class="item-title">' + escapeHtml(h.name) + '</div>';
                        content += '<div class="item-detail">' + escapeHtml(h.instructions) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.household.notes) content += '<div class="notes">' + escapeHtml(data.household.notes) + '</div>';
                html += renderSection('Household', 'household', content);
            }}

            // Personal Section
            if (data.personal) {{
                let content = '';
                if (data.personal.funeral_preferences) {{
                    content += '<h3>Funeral Preferences</h3><div class="item">' + escapeHtml(data.personal.funeral_preferences) + '</div>';
                }}
                if (data.personal.obituary_notes) {{
                    content += '<h3>Obituary Notes</h3><div class="item">' + escapeHtml(data.personal.obituary_notes) + '</div>';
                }}
                if (data.personal.messages && data.personal.messages.length) {{
                    content += '<h3>Personal Messages</h3>';
                    data.personal.messages.forEach(m => {{
                        content += '<div class="item"><div class="item-title">To: ' + escapeHtml(m.recipient) + '</div>';
                        content += '<div class="item-detail">' + escapeHtml(m.message) + '</div>';
                        content += '</div>';
                    }});
                }}
                if (data.personal.notes) content += '<div class="notes">' + escapeHtml(data.personal.notes) + '</div>';
                html += renderSection('Personal Wishes', 'personal', content);
            }}

            // Contacts Section
            if (data.contacts) {{
                let content = '';
                if (data.contacts.emergency_contacts && data.contacts.emergency_contacts.length) {{
                    content += '<h3>Emergency Contacts</h3>';
                    data.contacts.emergency_contacts.forEach(c => {{
                        content += '<div class="item">' + renderContact(c) + '</div>';
                    }});
                }}
                if (data.contacts.family && data.contacts.family.length) {{
                    content += '<h3>Family</h3>';
                    data.contacts.family.forEach(c => {{
                        content += '<div class="item">' + renderContact(c) + '</div>';
                    }});
                }}
                if (data.contacts.professionals && data.contacts.professionals.length) {{
                    content += '<h3>Professional Contacts</h3>';
                    data.contacts.professionals.forEach(c => {{
                        content += '<div class="item">' + renderContact(c) + '</div>';
                    }});
                }}
                if (data.contacts.notes) content += '<div class="notes">' + escapeHtml(data.contacts.notes) + '</div>';
                html += renderSection('Important Contacts', 'contacts', content);
            }}

            // Medical Section
            if (data.medical && data.medical.family_members && data.medical.family_members.length) {{
                let content = '';
                data.medical.family_members.forEach(m => {{
                    content += '<div class="item"><div class="item-title">' + escapeHtml(m.name) + '</div>';
                    if (m.conditions && m.conditions.length) content += '<div class="item-detail"><strong>Conditions:</strong> ' + m.conditions.map(c => escapeHtml(c)).join(', ') + '</div>';
                    if (m.allergies && m.allergies.length) content += '<div class="item-detail"><strong>Allergies:</strong> ' + m.allergies.map(a => escapeHtml(a)).join(', ') + '</div>';
                    if (m.medications && m.medications.length) {{
                        content += '<div class="item-detail"><strong>Medications:</strong></div>';
                        m.medications.forEach(med => {{
                            content += '<div class="item-detail">&nbsp;&nbsp;' + escapeHtml(med.name) + ' - ' + escapeHtml(med.dosage) + ' (' + escapeHtml(med.frequency) + ')</div>';
                        }});
                    }}
                    if (m.pharmacy && m.pharmacy.name) content += '<div class="item-detail"><strong>Pharmacy:</strong> ' + escapeHtml(m.pharmacy.name) + ' ' + escapeHtml(m.pharmacy.phone || '') + '</div>';
                    if (m.notes) content += '<div class="notes">' + escapeHtml(m.notes) + '</div>';
                    content += '</div>';
                }});
                if (data.medical.notes) content += '<div class="notes">' + escapeHtml(data.medical.notes) + '</div>';
                html += renderSection('Medical Information', 'medical', content);
            }}

            // Pets Section
            if (data.pets && data.pets.pets && data.pets.pets.length) {{
                let content = '';
                data.pets.pets.forEach(p => {{
                    content += '<div class="item"><div class="item-title">' + escapeHtml(p.name) + '</div>';
                    content += '<div class="item-detail">Species: ' + escapeHtml(p.species) + '</div>';
                    content += '<div class="item-detail">Breed: ' + escapeHtml(p.breed) + '</div>';
                    if (p.vet && p.vet.name) content += '<div class="item-detail"><strong>Vet:</strong> ' + escapeHtml(p.vet.name) + ' ' + escapeHtml(p.vet.phone || '') + '</div>';
                    if (p.medications && p.medications.length) {{
                        content += '<div class="item-detail"><strong>Medications:</strong></div>';
                        p.medications.forEach(med => {{
                            content += '<div class="item-detail">&nbsp;&nbsp;' + escapeHtml(med.name) + ' - ' + escapeHtml(med.dosage) + '</div>';
                        }});
                    }}
                    if (p.feeding) content += '<div class="item-detail"><strong>Feeding:</strong> ' + escapeHtml(p.feeding) + '</div>';
                    if (p.care_notes) content += '<div class="notes">' + escapeHtml(p.care_notes) + '</div>';
                    content += '</div>';
                }});
                if (data.pets.notes) content += '<div class="notes">' + escapeHtml(data.pets.notes) + '</div>';
                html += renderSection('Pets', 'pets', content);
            }}

            container.innerHTML = html;
            document.getElementById('lockScreen').style.display = 'none';
            document.getElementById('content').classList.add('visible');
        }}

        function search(term) {{
            const content = document.getElementById('documentContent');
            // Remove existing highlights
            content.innerHTML = content.innerHTML.replace(/<mark class="highlight">(.*?)<\/mark>/g, '$1');
            if (term && term.length > 1) {{
                const regex = new RegExp(`(${{term.replace(/[.*+?^${{}}()|[\]\\]/g, '\\$&')}})`, 'gi');
                content.innerHTML = content.innerHTML.replace(regex, '<mark class="highlight">$1</mark>');
            }}
        }}
    </script>
</body>
</html>"##,
        creator_name = creator_name,
        encrypted_data = encrypted_data
    )
}

/// Escapes HTML special characters
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Generates a printable HTML document (unencrypted, for direct printing)
pub fn generate_print_html(document: &LegacyDocument) -> String {
    let mut sections = String::new();

    // Financial Section
    if !document.financial.bank_accounts.is_empty()
        || !document.financial.credit_cards.is_empty()
        || !document.financial.investments.is_empty()
        || !document.financial.debts.is_empty()
    {
        sections.push_str(&render_financial_section(&document.financial));
    }

    // Insurance Section
    if !document.insurance.policies.is_empty() {
        sections.push_str(&render_insurance_section(&document.insurance));
    }

    // Bills Section
    if !document.bills.bills.is_empty() {
        sections.push_str(&render_bills_section(&document.bills));
    }

    // Property Section
    if !document.property.properties.is_empty()
        || !document.property.vehicles.is_empty()
        || !document.property.valuables.is_empty()
    {
        sections.push_str(&render_property_section(&document.property));
    }

    // Legal Section
    if !document.legal.will_location.is_empty()
        || !document.legal.attorney.name.is_empty()
        || !document.legal.trusts.is_empty()
    {
        sections.push_str(&render_legal_section(&document.legal));
    }

    // Digital Section
    if !document.digital.email_accounts.is_empty()
        || !document.digital.social_media.is_empty()
        || !document.digital.password_manager.name.is_empty()
    {
        sections.push_str(&render_digital_section(&document.digital));
    }

    // Household Section
    if !document.household.maintenance_items.is_empty()
        || !document.household.contractors.is_empty()
        || !document.household.how_things_work.is_empty()
    {
        sections.push_str(&render_household_section(&document.household));
    }

    // Personal Section
    if !document.personal.funeral_preferences.is_empty()
        || !document.personal.messages.is_empty()
    {
        sections.push_str(&render_personal_section(&document.personal));
    }

    // Contacts Section
    if !document.contacts.emergency_contacts.is_empty()
        || !document.contacts.family.is_empty()
        || !document.contacts.professionals.is_empty()
    {
        sections.push_str(&render_contacts_section(&document.contacts));
    }

    // Medical Section
    if !document.medical.family_members.is_empty() {
        sections.push_str(&render_medical_section(&document.medical));
    }

    // Pets Section
    if !document.pets.pets.is_empty() {
        sections.push_str(&render_pets_section(&document.pets));
    }

    let date = chrono::Local::now().format("%B %d, %Y").to_string();

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Legacy Document - {creator}</title>
    <style>
        body {{
            font-family: Georgia, 'Times New Roman', serif;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 40px 20px;
            color: #333;
        }}
        h1 {{
            text-align: center;
            border-bottom: 3px double #333;
            padding-bottom: 15px;
            margin-bottom: 10px;
        }}
        .subtitle {{
            text-align: center;
            color: #666;
            margin-bottom: 40px;
        }}
        h2 {{
            color: #333;
            border-bottom: 1px solid #999;
            padding-bottom: 8px;
            margin-top: 40px;
            page-break-after: avoid;
        }}
        h3 {{
            color: #555;
            margin-top: 25px;
            margin-bottom: 10px;
        }}
        .item {{
            background: #f9f9f9;
            padding: 15px;
            margin: 15px 0;
            border-left: 4px solid #666;
            page-break-inside: avoid;
        }}
        .item-title {{
            font-weight: bold;
            font-size: 1.1em;
            margin-bottom: 8px;
        }}
        .item-detail {{
            margin: 4px 0;
        }}
        .item-detail strong {{
            display: inline-block;
            min-width: 120px;
        }}
        .notes {{
            background: #fffbe6;
            padding: 15px;
            margin: 15px 0;
            font-style: italic;
            border-left: 4px solid #c9a227;
        }}
        .notes-label {{
            font-weight: bold;
            font-style: normal;
        }}
        .contact-info {{
            background: #f0f7ff;
            padding: 10px 15px;
            margin: 5px 0;
            border-radius: 4px;
        }}
        .warning {{
            background: #fff0f0;
            border-left: 4px solid #c00;
            padding: 15px;
            margin: 15px 0;
        }}
        @media print {{
            body {{
                max-width: none;
                padding: 20px;
            }}
            h2 {{
                page-break-after: avoid;
            }}
            .item {{
                page-break-inside: avoid;
            }}
        }}
    </style>
</head>
<body>
    <h1>Legacy Document</h1>
    <p class="subtitle">Prepared by {creator} &bull; {date}</p>
    <div class="warning">
        <strong>Important:</strong> This document contains sensitive personal information.
        Keep it in a secure location and share only with trusted individuals.
    </div>
    {sections}
</body>
</html>"#,
        creator = escape_html(&document.meta.creator_name),
        date = date,
        sections = sections
    )
}

fn render_contact(contact: &crate::models::Contact) -> String {
    let mut html = String::new();
    if !contact.name.is_empty() {
        html.push_str(&format!("<div class=\"item-detail\"><strong>Name:</strong> {}</div>", escape_html(&contact.name)));
    }
    if !contact.relationship.is_empty() {
        html.push_str(&format!("<div class=\"item-detail\"><strong>Relationship:</strong> {}</div>", escape_html(&contact.relationship)));
    }
    if !contact.phone.is_empty() {
        html.push_str(&format!("<div class=\"item-detail\"><strong>Phone:</strong> {}</div>", escape_html(&contact.phone)));
    }
    if !contact.email.is_empty() {
        html.push_str(&format!("<div class=\"item-detail\"><strong>Email:</strong> {}</div>", escape_html(&contact.email)));
    }
    if !contact.notes.is_empty() {
        html.push_str(&format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&contact.notes)));
    }
    html
}

fn render_medication(med: &crate::models::Medication) -> String {
    format!(
        "<div class=\"item-detail\">{} - {} ({})</div>",
        escape_html(&med.name),
        escape_html(&med.dosage),
        escape_html(&med.frequency)
    )
}

fn render_financial_section(financial: &crate::models::FinancialSection) -> String {
    let mut html = String::from("<h2>Financial Information</h2>");

    if !financial.bank_accounts.is_empty() {
        html.push_str("<h3>Bank Accounts</h3>");
        for account in &financial.bank_accounts {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Institution:</strong> {}</div>
                    <div class="item-detail"><strong>Type:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&account.name),
                escape_html(&account.institution),
                escape_html(&account.account_type),
                if !account.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&account.notes))
                } else { String::new() }
            ));
        }
    }

    if !financial.credit_cards.is_empty() {
        html.push_str("<h3>Credit Cards</h3>");
        for card in &financial.credit_cards {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Issuer:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&card.name),
                escape_html(&card.issuer),
                if !card.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&card.notes))
                } else { String::new() }
            ));
        }
    }

    if !financial.investments.is_empty() {
        html.push_str("<h3>Investments</h3>");
        for inv in &financial.investments {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Type:</strong> {}</div>
                    <div class="item-detail"><strong>Institution:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&inv.name),
                escape_html(&inv.account_type),
                escape_html(&inv.institution),
                if !inv.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&inv.notes))
                } else { String::new() }
            ));
        }
    }

    if !financial.debts.is_empty() {
        html.push_str("<h3>Debts & Loans</h3>");
        for debt in &financial.debts {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Lender:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&debt.name),
                escape_html(&debt.lender),
                if !debt.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&debt.notes))
                } else { String::new() }
            ));
        }
    }

    if !financial.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&financial.notes)
        ));
    }

    html
}

fn render_insurance_section(insurance: &crate::models::InsuranceSection) -> String {
    let mut html = String::from("<h2>Insurance</h2>");

    for policy in &insurance.policies {
        html.push_str(&format!(
            r#"<div class="item">
                <div class="item-title">{}</div>
                <div class="item-detail"><strong>Provider:</strong> {}</div>
                <div class="item-detail"><strong>Policy #:</strong> {}</div>
                <div class="item-detail"><strong>Contact:</strong> {}</div>
                {}
            </div>"#,
            escape_html(&policy.policy_type),
            escape_html(&policy.provider),
            escape_html(&policy.policy_number),
            escape_html(&policy.contact),
            if !policy.notes.is_empty() {
                format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&policy.notes))
            } else { String::new() }
        ));
    }

    if !insurance.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&insurance.notes)
        ));
    }

    html
}

fn render_bills_section(bills: &crate::models::BillsSection) -> String {
    let mut html = String::from("<h2>Bills</h2>");

    for bill in &bills.bills {
        html.push_str(&format!(
            r#"<div class="item">
                <div class="item-title">{}</div>
                <div class="item-detail"><strong>Provider:</strong> {}</div>
                <div class="item-detail"><strong>Amount:</strong> {}</div>
                <div class="item-detail"><strong>Due Day:</strong> {}</div>
                <div class="item-detail"><strong>Auto-pay:</strong> {}</div>
                {}
            </div>"#,
            escape_html(&bill.name),
            escape_html(&bill.provider),
            escape_html(&bill.amount),
            escape_html(&bill.due_day),
            if bill.autopay { "Yes" } else { "No" },
            if !bill.notes.is_empty() {
                format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&bill.notes))
            } else { String::new() }
        ));
    }

    if !bills.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&bills.notes)
        ));
    }

    html
}

fn render_property_section(property: &crate::models::PropertySection) -> String {
    let mut html = String::from("<h2>Property</h2>");

    if !property.properties.is_empty() {
        html.push_str("<h3>Properties</h3>");
        for prop in &property.properties {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Address:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&prop.name),
                escape_html(&prop.address),
                if !prop.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&prop.notes))
                } else { String::new() }
            ));
        }
    }

    if !property.vehicles.is_empty() {
        html.push_str("<h3>Vehicles</h3>");
        for vehicle in &property.vehicles {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Details:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&vehicle.name),
                escape_html(&vehicle.details),
                if !vehicle.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&vehicle.notes))
                } else { String::new() }
            ));
        }
    }

    if !property.valuables.is_empty() {
        html.push_str("<h3>Valuables</h3>");
        for item in &property.valuables {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Location:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&item.name),
                escape_html(&item.location),
                if !item.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&item.notes))
                } else { String::new() }
            ));
        }
    }

    if !property.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&property.notes)
        ));
    }

    html
}

fn render_legal_section(legal: &crate::models::LegalSection) -> String {
    let mut html = String::from("<h2>Legal Documents</h2>");

    html.push_str("<div class=\"item\">");

    if !legal.will_location.is_empty() {
        html.push_str(&format!("<div class=\"item-detail\"><strong>Will Location:</strong> {}</div>", escape_html(&legal.will_location)));
    }

    if !legal.power_of_attorney.is_empty() {
        html.push_str(&format!("<div class=\"item-detail\"><strong>Power of Attorney:</strong> {}</div>", escape_html(&legal.power_of_attorney)));
    }

    if !legal.attorney.name.is_empty() {
        html.push_str("<div class=\"item-detail\"><strong>Attorney:</strong></div>");
        html.push_str(&format!("<div class=\"contact-info\">{}</div>", render_contact(&legal.attorney)));
    }

    html.push_str("</div>");

    // Trusts
    if !legal.trusts.is_empty() {
        html.push_str("<h3>Trusts</h3>");
        for trust in &legal.trusts {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Trustee:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&trust.name),
                escape_html(&trust.trustee),
                if !trust.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&trust.notes))
                } else { String::new() }
            ));
        }
    }

    if !legal.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&legal.notes)
        ));
    }

    html
}

fn render_digital_section(digital: &crate::models::DigitalSection) -> String {
    let mut html = String::from("<h2>Digital Life</h2>");

    if !digital.password_manager.name.is_empty() {
        html.push_str("<h3>Password Manager</h3>");
        html.push_str(&format!(
            r#"<div class="item">
                <div class="item-title">{}</div>
                <div class="item-detail"><strong>Master Password Hint:</strong> {}</div>
                <div class="item-detail"><strong>Recovery Method:</strong> {}</div>
                {}
            </div>"#,
            escape_html(&digital.password_manager.name),
            escape_html(&digital.password_manager.master_password_hint),
            escape_html(&digital.password_manager.recovery_method),
            if !digital.password_manager.notes.is_empty() {
                format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&digital.password_manager.notes))
            } else { String::new() }
        ));
    }

    if !digital.email_accounts.is_empty() {
        html.push_str("<h3>Email Accounts</h3>");
        for email in &digital.email_accounts {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Username:</strong> {}</div>
                    <div class="item-detail"><strong>Recovery Hint:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&email.name),
                escape_html(&email.username),
                escape_html(&email.recovery_hint),
                if !email.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&email.notes))
                } else { String::new() }
            ));
        }
    }

    if !digital.social_media.is_empty() {
        html.push_str("<h3>Social Media</h3>");
        for social in &digital.social_media {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Username:</strong> {}</div>
                    <div class="item-detail"><strong>Recovery Hint:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&social.name),
                escape_html(&social.username),
                escape_html(&social.recovery_hint),
                if !social.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&social.notes))
                } else { String::new() }
            ));
        }
    }

    if !digital.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&digital.notes)
        ));
    }

    html
}

fn render_household_section(household: &crate::models::HouseholdSection) -> String {
    let mut html = String::from("<h2>Household</h2>");

    if !household.maintenance_items.is_empty() {
        html.push_str("<h3>Maintenance</h3>");
        for item in &household.maintenance_items {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail"><strong>Frequency:</strong> {}</div>
                    <div class="item-detail"><strong>Last Done:</strong> {}</div>
                    {}
                </div>"#,
                escape_html(&item.name),
                escape_html(&item.frequency),
                escape_html(&item.last_done),
                if !item.notes.is_empty() {
                    format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&item.notes))
                } else { String::new() }
            ));
        }
    }

    if !household.contractors.is_empty() {
        html.push_str("<h3>Contractors</h3>");
        for contractor in &household.contractors {
            html.push_str(&format!(
                r#"<div class="item"><div class="contact-info">{}</div></div>"#,
                render_contact(contractor)
            ));
        }
    }

    if !household.how_things_work.is_empty() {
        html.push_str("<h3>How Things Work</h3>");
        for howto in &household.how_things_work {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">{}</div>
                    <div class="item-detail">{}</div>
                </div>"#,
                escape_html(&howto.name),
                escape_html(&howto.instructions)
            ));
        }
    }

    if !household.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&household.notes)
        ));
    }

    html
}

fn render_personal_section(personal: &crate::models::PersonalSection) -> String {
    let mut html = String::from("<h2>Personal Wishes</h2>");

    if !personal.funeral_preferences.is_empty() {
        html.push_str("<h3>Funeral Preferences</h3>");
        html.push_str(&format!(
            r#"<div class="item"><div class="item-detail">{}</div></div>"#,
            escape_html(&personal.funeral_preferences)
        ));
    }

    if !personal.obituary_notes.is_empty() {
        html.push_str("<h3>Obituary Notes</h3>");
        html.push_str(&format!(
            r#"<div class="item"><div class="item-detail">{}</div></div>"#,
            escape_html(&personal.obituary_notes)
        ));
    }

    // Messages
    if !personal.messages.is_empty() {
        html.push_str("<h3>Personal Messages</h3>");
        for msg in &personal.messages {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-title">To: {}</div>
                    <div class="item-detail">{}</div>
                </div>"#,
                escape_html(&msg.recipient),
                escape_html(&msg.message)
            ));
        }
    }

    if !personal.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&personal.notes)
        ));
    }

    html
}

fn render_contacts_section(contacts: &crate::models::ContactsSection) -> String {
    let mut html = String::from("<h2>Important Contacts</h2>");

    if !contacts.emergency_contacts.is_empty() {
        html.push_str("<h3>Emergency Contacts</h3>");
        for contact in &contacts.emergency_contacts {
            html.push_str(&format!(
                r#"<div class="item"><div class="contact-info">{}</div></div>"#,
                render_contact(contact)
            ));
        }
    }

    if !contacts.family.is_empty() {
        html.push_str("<h3>Family</h3>");
        for contact in &contacts.family {
            html.push_str(&format!(
                r#"<div class="item"><div class="contact-info">{}</div></div>"#,
                render_contact(contact)
            ));
        }
    }

    if !contacts.professionals.is_empty() {
        html.push_str("<h3>Professional Contacts</h3>");
        for contact in &contacts.professionals {
            html.push_str(&format!(
                r#"<div class="item"><div class="contact-info">{}</div></div>"#,
                render_contact(contact)
            ));
        }
    }

    if !contacts.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&contacts.notes)
        ));
    }

    html
}

fn render_medical_section(medical: &crate::models::MedicalSection) -> String {
    let mut html = String::from("<h2>Medical Information</h2>");

    for member in &medical.family_members {
        html.push_str(&format!(
            r#"<div class="item">
                <div class="item-title">{}</div>"#,
            escape_html(&member.name)
        ));

        if !member.conditions.is_empty() {
            html.push_str(&format!("<div class=\"item-detail\"><strong>Conditions:</strong> {}</div>",
                member.conditions.iter().map(|c| escape_html(c)).collect::<Vec<_>>().join(", ")));
        }
        if !member.allergies.is_empty() {
            html.push_str(&format!("<div class=\"item-detail\"><strong>Allergies:</strong> {}</div>",
                member.allergies.iter().map(|a| escape_html(a)).collect::<Vec<_>>().join(", ")));
        }
        if !member.medications.is_empty() {
            html.push_str("<div class=\"item-detail\"><strong>Medications:</strong></div>");
            for med in &member.medications {
                html.push_str(&render_medication(med));
            }
        }
        if !member.pharmacy.name.is_empty() {
            html.push_str("<div class=\"item-detail\"><strong>Pharmacy:</strong></div>");
            html.push_str(&format!("<div class=\"contact-info\">{}</div>", render_contact(&member.pharmacy)));
        }
        if !member.notes.is_empty() {
            html.push_str(&format!("<div class=\"item-detail\"><strong>Notes:</strong> {}</div>", escape_html(&member.notes)));
        }

        html.push_str("</div>");
    }

    if !medical.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&medical.notes)
        ));
    }

    html
}

fn render_pets_section(pets: &crate::models::PetsSection) -> String {
    let mut html = String::from("<h2>Pets</h2>");

    for pet in &pets.pets {
        html.push_str(&format!(
            r#"<div class="item">
                <div class="item-title">{}</div>
                <div class="item-detail"><strong>Species:</strong> {}</div>
                <div class="item-detail"><strong>Breed:</strong> {}</div>"#,
            escape_html(&pet.name),
            escape_html(&pet.species),
            escape_html(&pet.breed)
        ));

        if !pet.vet.name.is_empty() {
            html.push_str("<div class=\"item-detail\"><strong>Veterinarian:</strong></div>");
            html.push_str(&format!("<div class=\"contact-info\">{}</div>", render_contact(&pet.vet)));
        }

        if !pet.medications.is_empty() {
            html.push_str("<div class=\"item-detail\"><strong>Medications:</strong></div>");
            for med in &pet.medications {
                html.push_str(&render_medication(med));
            }
        }

        if !pet.feeding.is_empty() {
            html.push_str(&format!("<div class=\"item-detail\"><strong>Feeding:</strong> {}</div>", escape_html(&pet.feeding)));
        }
        if !pet.care_notes.is_empty() {
            html.push_str(&format!("<div class=\"item-detail\"><strong>Care Notes:</strong> {}</div>", escape_html(&pet.care_notes)));
        }

        html.push_str("</div>");
    }

    if !pets.notes.is_empty() {
        html.push_str(&format!(
            r#"<div class="notes"><span class="notes-label">Notes:</span> {}</div>"#,
            escape_html(&pets.notes)
        ));
    }

    html
}

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

        let html = generate_encrypted_html(&original, passphrase)
            .expect("export should succeed");

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("ENCRYPTED_DATA"));
        assert!(html.contains("Test User"));

        let imported = import_from_html(&html, passphrase)
            .expect("import should succeed");

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

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test User"));
        assert!(html.contains("Checking Account"));
        assert!(html.contains("Test Bank"));
        assert!(!html.contains("ENCRYPTED_DATA"));
    }
}
