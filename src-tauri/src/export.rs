use crate::encryption::{decrypt_from_browser, decrypt_key_with_passphrase, decrypt_with_raw_key, encrypt_for_browser, encrypt_key_with_passphrase, encrypt_with_raw_key, generate_document_key, EncryptedPayload, EncryptionError};
use serde::Deserialize;
use crate::models::{FieldType, LegacyDocument, SlideType};
use serde::Serialize;

#[derive(Debug)]
pub enum ExportError {
    EncryptionError(EncryptionError),
    SerializationError(String),
    ParseError(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::EncryptionError(e) => write!(f, "Encryption error: {}", e),
            ExportError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
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
    include_welcome_screen: bool,
) -> Result<String, ExportError> {
    // Serialize document to JSON
    let json = serde_json::to_string(document)
        .map_err(|e| ExportError::SerializationError(e.to_string()))?;

    // Encrypt the JSON
    let encrypted = encrypt_for_browser(&json, passphrase)?;

    // Serialize encrypted payload
    let encrypted_json = serde_json::to_string(&encrypted)
        .map_err(|e| ExportError::SerializationError(e.to_string()))?;

    // Prepare welcome screen data if enabled
    let welcome_screen_json = if include_welcome_screen {
        if let Some(ref welcome) = document.welcome_screen {
            if welcome.enabled && !welcome.slides.is_empty() {
                serde_json::to_string(&welcome.slides)
                    .map_err(|e| ExportError::SerializationError(e.to_string()))?
            } else {
                "[]".to_string()
            }
        } else {
            "[]".to_string()
        }
    } else {
        "[]".to_string()
    };

    // Generate the HTML
    let html = generate_html_template(&encrypted_json, &document.meta.creator_name, &welcome_screen_json);

    Ok(html)
}

/// Structure for dual-key encrypted data (question-based unlock)
#[derive(Serialize)]
struct DualKeyEncryptedData {
    question_key: EncryptedPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    passphrase_key: Option<EncryptedPayload>,
    document: DocumentPayload,
}

#[derive(Serialize)]
struct DocumentPayload {
    nonce: String,
    ciphertext: String,
}

/// Slide data for export (without answers)
#[derive(Serialize)]
struct ExportSlide {
    id: String,
    #[serde(rename = "type")]
    slide_type: String,
    text: String,
    transition: serde_json::Value,
}

/// Generates encrypted HTML with question-based unlock
pub fn generate_encrypted_html_with_questions(
    document: &LegacyDocument,
    include_welcome_screen: bool,
) -> Result<String, ExportError> {
    let welcome = document.welcome_screen.as_ref()
        .ok_or_else(|| ExportError::SerializationError("Welcome screen not configured".into()))?;

    if !welcome.enabled {
        return Err(ExportError::SerializationError("Welcome screen not enabled".into()));
    }

    // Extract question slides and build the key from answers
    let question_slides: Vec<_> = welcome.slides.iter()
        .filter(|s| s.slide_type == SlideType::Question)
        .collect();

    if question_slides.len() < 2 {
        return Err(ExportError::SerializationError("At least 2 questions required".into()));
    }

    if question_slides.len() > 5 {
        return Err(ExportError::SerializationError("Maximum 5 questions allowed".into()));
    }

    // Concatenate answers (normalized to lowercase) to form the question key passphrase
    let question_passphrase: String = question_slides.iter()
        .filter_map(|s| s.answer.as_ref())
        .map(|a| a.to_lowercase().trim().to_string())
        .collect::<Vec<_>>()
        .join("");

    if question_passphrase.is_empty() {
        return Err(ExportError::SerializationError("All questions must have answers".into()));
    }

    // Serialize document to JSON
    let json = serde_json::to_string(document)
        .map_err(|e| ExportError::SerializationError(e.to_string()))?;

    // Generate random document key
    let doc_key = generate_document_key();

    // Encrypt document with document key
    let doc_encrypted = encrypt_with_raw_key(json.as_bytes(), &doc_key)?;

    // Encrypt document key with question-derived key
    let question_key_encrypted = encrypt_key_with_passphrase(&doc_key, &question_passphrase)?;

    // Optionally encrypt document key with fallback passphrase
    let passphrase_key_encrypted = if let Some(ref passphrase) = welcome.fallback_passphrase {
        if !passphrase.is_empty() {
            Some(encrypt_key_with_passphrase(&doc_key, passphrase)?)
        } else {
            None
        }
    } else {
        None
    };

    // Build the encrypted data structure
    let encrypted_data = DualKeyEncryptedData {
        question_key: question_key_encrypted,
        passphrase_key: passphrase_key_encrypted,
        document: DocumentPayload {
            nonce: doc_encrypted.nonce,
            ciphertext: doc_encrypted.ciphertext,
        },
    };

    let encrypted_json = serde_json::to_string(&encrypted_data)
        .map_err(|e| ExportError::SerializationError(e.to_string()))?;

    // Prepare slides for export (strip answers from question slides)
    let export_slides: Vec<ExportSlide> = if include_welcome_screen {
        welcome.slides.iter().map(|s| ExportSlide {
            id: s.id.clone(),
            slide_type: match s.slide_type {
                SlideType::Message => "message".to_string(),
                SlideType::Question => "question".to_string(),
            },
            text: s.text.clone(),
            transition: serde_json::to_value(&s.transition).unwrap_or(serde_json::json!({"type": "click"})),
        }).collect()
    } else {
        vec![]
    };

    let slides_json = serde_json::to_string(&export_slides)
        .map_err(|e| ExportError::SerializationError(e.to_string()))?;

    let has_passphrase_fallback = welcome.fallback_passphrase.as_ref().map(|p| !p.is_empty()).unwrap_or(false);

    // Generate the HTML with question-based unlock
    let html = generate_question_html_template(
        &encrypted_json,
        &slides_json,
        has_passphrase_fallback,
    );

    Ok(html)
}

/// Structure for question-based encrypted data (used for import detection)
#[derive(Deserialize)]
struct QuestionBasedEncryptedData {
    question_key: EncryptedPayload,
    passphrase_key: Option<EncryptedPayload>,
    document: DocumentPayloadImport,
}

#[derive(Deserialize)]
struct DocumentPayloadImport {
    nonce: String,
    ciphertext: String,
}

/// Extracts JSON object from HTML starting at the given marker
fn extract_json_from_html(html: &str, marker: &str) -> Result<String, ExportError> {
    let start_pos = html.find(marker)
        .ok_or_else(|| ExportError::ParseError(format!("Could not find '{}' in HTML file", marker)))?;

    let json_start = start_pos + marker.len();

    // Find the matching closing brace by counting braces
    let mut brace_count = 0;
    let mut json_end = json_start;
    let mut in_string = false;
    let mut escape_next = false;

    for (i, c) in html[json_start..].char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match c {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => brace_count += 1,
            '}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    json_end = json_start + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }

    if brace_count != 0 {
        return Err(ExportError::ParseError("Malformed JSON in HTML file".into()));
    }

    Ok(html[json_start..json_end].to_string())
}

/// Imports a legacy document from an encrypted HTML file
pub fn import_from_html(html: &str, passphrase: &str) -> Result<LegacyDocument, ExportError> {
    let encrypted_json = extract_json_from_html(html, "const ENCRYPTED_DATA = ")?;

    // Try to detect if this is a question-based export by checking for question_key field
    if encrypted_json.contains("\"question_key\"") {
        // This is a question-based export
        let data: QuestionBasedEncryptedData = serde_json::from_str(&encrypted_json)
            .map_err(|e| ExportError::ParseError(format!("Invalid question-based encrypted data: {}", e)))?;

        // Check if passphrase_key is available
        let passphrase_key = data.passphrase_key.ok_or_else(|| {
            ExportError::ParseError(
                "This file was exported with question-based unlock and no fallback passphrase. \
                 It can only be opened by answering the original questions in a browser.".into()
            )
        })?;

        // Decrypt the document key using the passphrase
        let doc_key = decrypt_key_with_passphrase(&passphrase_key, passphrase)?;

        // Decrypt the document using the document key
        let decrypted_json = decrypt_with_raw_key(&data.document.nonce, &data.document.ciphertext, &doc_key)?;

        // Parse the decrypted JSON into a LegacyDocument
        let document: LegacyDocument = serde_json::from_str(&decrypted_json)
            .map_err(|e| ExportError::SerializationError(format!("Invalid document format: {}", e)))?;

        Ok(document)
    } else {
        // This is a passphrase-based export (original format)
        let payload: EncryptedPayload = serde_json::from_str(&encrypted_json)
            .map_err(|e| ExportError::ParseError(format!("Invalid encrypted data format: {}", e)))?;

        // Decrypt the payload
        let decrypted_json = decrypt_from_browser(&payload, passphrase)?;

        // Parse the decrypted JSON into a LegacyDocument
        let document: LegacyDocument = serde_json::from_str(&decrypted_json)
            .map_err(|e| ExportError::SerializationError(format!("Invalid document format: {}", e)))?;

        Ok(document)
    }
}

// ============================================================================
// SHARED TEMPLATE COMPONENTS
// ============================================================================

/// Shared CSS styles used by both templates
const SHARED_CSS: &str = r##"
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body { font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; background: #F0EFEB; color: #283618; }
        .content { display: none; width: 100%; }
        .content.visible { display: block; width: 100%; }
        .layout { display: flex; min-height: 100vh; width: 100%; }
        .container { width: 100%; }
        .sidebar { width: 280px; min-width: 280px; background: #FFFFFF; border-right: 1px solid #D4D4D4; height: 100vh; position: fixed; left: 0; top: 0; overflow-y: auto; display: flex; flex-direction: column; z-index: 100; }
        .sidebar-header { padding: 16px 20px; border-bottom: 1px solid #D4D4D4; background: #283618; display: flex; align-items: center; gap: 12px; }
        .logo-icon { width: 40px; height: 40px; flex-shrink: 0; }
        .logo-text { flex: 1; }
        .sidebar-title { font-size: 1.25rem; font-weight: 600; color: #F0EFEB; margin-bottom: 0.25rem; }
        .sidebar-subtitle { font-size: 0.8rem; color: #B7B7A4; }
        .sidebar-search { padding: 16px; border-bottom: 1px solid #D4D4D4; }
        .search-wrapper { position: relative; display: flex; align-items: center; }
        .search-input { width: 100%; padding: 10px 36px 10px 14px; border: 1px solid #D4D4D4; border-radius: 8px; font-size: 0.9rem; background: #F0EFEB; transition: border-color 0.2s, box-shadow 0.2s; }
        .search-input:focus { outline: none; border-color: #283618; box-shadow: 0 0 0 3px rgba(40, 54, 24, 0.1); background: white; }
        .search-clear { position: absolute; right: 10px; background: none; border: none; cursor: pointer; color: #B7B7A4; font-size: 1.1rem; padding: 0 4px; line-height: 1; transition: color 0.2s; }
        .search-clear:hover { color: #283618; }
        .search-clear.hidden { display: none; }
        .search-controls { padding: 12px 16px; border-bottom: 1px solid #D4D4D4; display: none; background: #F0EFEB; }
        .search-controls.visible { display: block; }
        .search-nav { display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.75rem; }
        .search-nav button { padding: 6px 12px; border: 1px solid #D4D4D4; background: white; border-radius: 6px; cursor: pointer; font-size: 0.9rem; transition: all 0.2s; }
        .search-nav button:hover:not(:disabled) { background: #F0EFEB; border-color: #B7B7A4; }
        .search-nav button:disabled { opacity: 0.4; cursor: not-allowed; }
        .search-counter { color: #606C38; font-size: 0.85rem; font-weight: 500; }
        .search-filters { display: flex; gap: 6px; flex-wrap: wrap; }
        .search-filter { padding: 4px 8px; border: 1px solid #D4D4D4; background: white; border-radius: 6px; font-size: 0.75rem; font-weight: 500; cursor: pointer; user-select: none; transition: all 0.2s; }
        .search-filter.active { background: #283618; color: #F0EFEB; border-color: #283618; }
        .search-filter.disabled { opacity: 0.4; cursor: not-allowed; }
        .sidebar-nav { flex: 1; overflow-y: auto; padding: 16px; }
        .nav-title { font-weight: 600; font-size: 0.7rem; text-transform: uppercase; color: #B7B7A4; margin-bottom: 0.75rem; letter-spacing: 0.05em; }
        .nav-list { list-style: none; }
        .nav-list li { margin: 2px 0; }
        .nav-list a { color: #283618; text-decoration: none; display: block; padding: 8px 12px; border-radius: 6px; font-size: 0.9rem; font-weight: 500; transition: all 0.2s; }
        .nav-list a:hover { background: #F0EFEB; color: #606C38; }
        .sidebar-footer { padding: 16px; border-top: 1px solid #D4D4D4; }
        .print-btn { width: 100%; padding: 12px 16px; background: #B7B7A4; color: #283618; border: none; border-radius: 8px; cursor: pointer; font-size: 0.9rem; font-weight: 500; transition: background 0.2s; }
        .print-btn:hover { background: #a3a392; }
        .main-content { flex: 1; margin-left: 280px; padding: 24px 40px; }
        .section { background: white; padding: 24px; border-radius: 12px; margin-bottom: 20px; box-shadow: 0 1px 3px rgba(40,54,24,0.08), 0 1px 2px rgba(40,54,24,0.04); border: 1px solid #D4D4D4; }
        .section-title { font-size: 1.15rem; font-weight: 600; color: #283618; border-bottom: 2px solid #283618; padding-bottom: 0.75rem; margin-bottom: 1.25rem; }
        .item { background: #F0EFEB; padding: 16px; border-radius: 8px; margin-bottom: 12px; border: 1px solid #D4D4D4; }
        .item-title { font-weight: 600; color: #283618; margin-bottom: 0.5rem; }
        .item-detail { color: #606C38; font-size: 0.9rem; }
        .notes { background: #F0EFEB; padding: 12px 14px; border-radius: 8px; margin-top: 1rem; font-style: italic; color: #283618; border-left: 3px solid #B7B7A4; }
        .match-badge { font-size: 0.65rem; font-weight: 500; color: #606C38; background: #D4D4D4; padding: 2px 6px; border-radius: 4px; margin-left: 4px; vertical-align: middle; text-transform: lowercase; }
        .highlight { background: #DDE5B6; padding: 1px 2px; border-radius: 2px; }
        .highlight.current { background: #ADC178; outline: 2px solid #283618; }
        .menu-toggle { display: none; position: fixed; top: 12px; left: 12px; z-index: 200; background: #283618; color: #F0EFEB; border: none; border-radius: 8px; padding: 10px 14px; cursor: pointer; font-weight: 500; font-size: 0.9rem; box-shadow: 0 2px 8px rgba(40,54,24,0.2); }
        @media (max-width: 768px) {
            .menu-toggle { display: block; }
            .sidebar { transform: translateX(-100%); transition: transform 0.3s ease; }
            .sidebar.open { transform: translateX(0); box-shadow: 4px 0 20px rgba(40,54,24,0.15); }
            .main-content { margin-left: 0; padding: 70px 16px 20px 16px; }
        }
        @media print { .sidebar, .menu-toggle { display: none; } .main-content { margin-left: 0; } .section { break-inside: avoid; box-shadow: none; border: 1px solid #D4D4D4; } }
"##;

/// Shared JavaScript utility functions
const SHARED_JS_UTILS: &str = r##"
        function escapeHtml(text) {
            if (!text) return '';
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }

        // Escape for use in HTML attributes (href, id, etc.)
        // Only allows alphanumeric, dash, underscore for safety
        function escapeAttr(text) {
            if (!text) return '';
            return String(text).replace(/[^a-zA-Z0-9_-]/g, '');
        }

        function renderContact(contact) {
            if (!contact || !contact.name) return '';
            let html = '<div class="contact-info">';
            if (contact.name) html += '<div><strong>' + escapeHtml(contact.name) + '</strong></div>';
            if (contact.relationship) html += '<div>' + escapeHtml(contact.relationship) + '</div>';
            if (contact.phone) html += '<div>Phone: ' + escapeHtml(contact.phone) + '</div>';
            if (contact.email) html += '<div>Email: ' + escapeHtml(contact.email) + '</div>';
            if (contact.notes) html += '<div class="notes">' + escapeHtml(contact.notes) + '</div>';
            html += '</div>';
            return html;
        }

        function renderSection(title, id, content) {
            if (!content) return '';
            return '<div class="section" id="' + escapeAttr(id) + '"><h2 class="section-title">' + escapeHtml(title) + '</h2>' + content + '</div>';
        }

        function renderCustomSubsection(subsection) {
            if (!subsection.items || !subsection.items.length) return '';
            let html = '<h3>' + escapeHtml(subsection.name) + '</h3>';
            subsection.items.forEach(item => {
                html += '<div class="item">';
                const textField = (subsection.field_definitions || []).find(fd => fd.field_type === 'text');
                const title = textField && item.values && item.values[textField.id] ? item.values[textField.id] : 'Item';
                html += '<div class="item-title">' + escapeHtml(title) + '</div>';
                (subsection.field_definitions || []).forEach(fd => {
                    const value = item.values && item.values[fd.id];
                    if (value) {
                        let displayValue = value;
                        if (fd.field_type === 'boolean') displayValue = value === 'true' ? 'Yes' : 'No';
                        html += '<div class="item-detail"><strong>' + escapeHtml(fd.name) + ':</strong> ' + escapeHtml(displayValue) + '</div>';
                    }
                });
                html += '</div>';
            });
            return html;
        }

        function toggleSidebar() {
            document.getElementById('sidebar').classList.toggle('open');
        }

        function closeSidebarOnMobile() {
            if (window.innerWidth <= 768) {
                document.getElementById('sidebar').classList.remove('open');
            }
        }
"##;

/// Shared JavaScript for search functionality
const SHARED_JS_SEARCH: &str = r##"
        function levenshtein(a, b) {
            if (a.length === 0) return b.length;
            if (b.length === 0) return a.length;
            const matrix = [];
            for (let i = 0; i <= b.length; i++) matrix[i] = [i];
            for (let j = 0; j <= a.length; j++) matrix[0][j] = j;
            for (let i = 1; i <= b.length; i++) {
                for (let j = 1; j <= a.length; j++) {
                    if (b.charAt(i - 1) === a.charAt(j - 1)) {
                        matrix[i][j] = matrix[i - 1][j - 1];
                    } else {
                        matrix[i][j] = Math.min(
                            matrix[i - 1][j - 1] + 1,
                            matrix[i][j - 1] + 1,
                            matrix[i - 1][j] + 1
                        );
                    }
                }
            }
            return matrix[b.length][a.length];
        }

        function metaphone(word) {
            if (!word) return '';
            word = word.toUpperCase();
            let result = '';
            const vowels = 'AEIOU';
            let i = 0;

            if (word.length >= 2) {
                const first2 = word.substring(0, 2);
                if (['KN', 'GN', 'PN', 'AE', 'WR'].includes(first2)) {
                    word = word.substring(1);
                }
            }

            while (i < word.length && result.length < 6) {
                const c = word[i];
                const prev = i > 0 ? word[i - 1] : '';
                const next = i < word.length - 1 ? word[i + 1] : '';

                if (c === prev && c !== 'C') { i++; continue; }
                if (vowels.includes(c) && i === 0) { result += c; i++; continue; }
                if (vowels.includes(c)) { i++; continue; }

                switch (c) {
                    case 'B': if (prev !== 'M') result += 'P'; break;
                    case 'C':
                        if (next === 'H') { result += 'X'; i++; }
                        else if (['I', 'E', 'Y'].includes(next)) result += 'S';
                        else result += 'K';
                        break;
                    case 'D':
                        if (next === 'G' && ['I', 'E', 'Y'].includes(word[i + 2] || '')) { result += 'J'; i++; }
                        else result += 'T';
                        break;
                    case 'F': result += 'F'; break;
                    case 'G':
                        if (next === 'H') { if (!vowels.includes(word[i + 2] || '')) i++; }
                        else if (next === 'N' && word[i + 2] === undefined) { }
                        else if (['I', 'E', 'Y'].includes(next)) result += 'J';
                        else result += 'K';
                        break;
                    case 'H': if (vowels.includes(next) && !vowels.includes(prev)) result += 'H'; break;
                    case 'J': result += 'J'; break;
                    case 'K': if (prev !== 'C') result += 'K'; break;
                    case 'L': result += 'L'; break;
                    case 'M': result += 'M'; break;
                    case 'N': result += 'N'; break;
                    case 'P': if (next === 'H') { result += 'F'; i++; } else result += 'P'; break;
                    case 'Q': result += 'K'; break;
                    case 'R': result += 'R'; break;
                    case 'S':
                        if (next === 'H') { result += 'X'; i++; }
                        else if (next === 'I' && ['O', 'A'].includes(word[i + 2] || '')) { result += 'X'; i++; }
                        else result += 'S';
                        break;
                    case 'T':
                        if (next === 'H') { result += '0'; i++; }
                        else if (next === 'I' && ['O', 'A'].includes(word[i + 2] || '')) { result += 'X'; i++; }
                        else result += 'T';
                        break;
                    case 'V': result += 'F'; break;
                    case 'W': case 'Y': if (vowels.includes(next)) result += c; break;
                    case 'X': result += 'KS'; break;
                    case 'Z': result += 'S'; break;
                }
                i++;
            }
            return result;
        }

        let searchIndex = [];
        let searchState = {
            term: '',
            matches: [],
            filters: { exact: true, contains: true, spelling: true, phonetic: true },
            currentIndex: -1
        };

        function buildSearchIndex() {
            searchIndex = [];
            const content = document.getElementById('mainContent');
            if (!content) return;
            const walker = document.createTreeWalker(content, NodeFilter.SHOW_TEXT, {
                acceptNode: (node) => {
                    return node.textContent.trim() ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_REJECT;
                }
            });

            let node;
            while (node = walker.nextNode()) {
                const text = node.textContent;
                const words = text.match(/\b[\w']+\b/g) || [];
                words.forEach(word => {
                    if (word.length >= 2) {
                        searchIndex.push({
                            text: word,
                            lowerText: word.toLowerCase(),
                            metaphone: metaphone(word),
                            node: node,
                            fullText: text
                        });
                    }
                });
            }
        }

        let searchTimeout;
        function debounceSearch(term) {
            clearTimeout(searchTimeout);
            const clearBtn = document.getElementById('searchClear');
            if (term) {
                clearBtn.classList.remove('hidden');
            } else {
                clearBtn.classList.add('hidden');
            }
            searchTimeout = setTimeout(() => performSearch(term), 300);
        }

        function clearSearch() {
            const input = document.getElementById('searchInput');
            input.value = '';
            document.getElementById('searchClear').classList.add('hidden');
            clearTimeout(searchTimeout);
            clearHighlights();
            document.getElementById('searchControls').classList.remove('visible');
            searchState.term = '';
            searchState.matches = [];
            searchState.currentIndex = -1;
        }

        function performSearch(term) {
            clearHighlights();
            buildSearchIndex();
            searchState.term = term.toLowerCase();
            searchState.matches = [];
            searchState.currentIndex = -1;
            searchState.filters = { exact: true, contains: true, spelling: true, phonetic: true };

            if (!term || term.length < 2) {
                document.getElementById('searchControls').classList.remove('visible');
                updateSearchUI();
                return;
            }

            document.getElementById('searchControls').classList.add('visible');
            const termMeta = metaphone(term);
            const matchMap = new Map();

            searchIndex.forEach(entry => {
                const key = entry.node.textContent + '|' + entry.text;
                if (matchMap.has(key)) return;

                const wordLower = entry.lowerText;
                const termLower = term.toLowerCase();

                if (wordLower === termLower) {
                    matchMap.set(key, { ...entry, type: 'exact' });
                    return;
                }

                if (term.length >= 3 && wordLower.includes(termLower)) {
                    matchMap.set(key, { ...entry, type: 'contains' });
                    return;
                }

                // Spelling match: require first char match and limit edit distance
                if (wordLower[0] === termLower[0]) {
                    const maxDist = term.length >= 8 ? 3 : (term.length >= 5 ? 2 : 1);
                    const dist = levenshtein(wordLower, termLower);
                    if (dist > 0 && dist <= maxDist) {
                        matchMap.set(key, { ...entry, type: 'spelling', distance: dist });
                        return;
                    }
                }

                if (term.length >= 3 && termMeta && entry.metaphone === termMeta) {
                    matchMap.set(key, { ...entry, type: 'phonetic' });
                }
            });

            const typeOrder = { exact: 0, contains: 1, spelling: 2, phonetic: 3 };
            searchState.matches = Array.from(matchMap.values())
                .sort((a, b) => typeOrder[a.type] - typeOrder[b.type]);

            highlightMatches();
            updateSearchUI();

            if (getVisibleMatches().length > 0) {
                searchState.currentIndex = 0;
                scrollToCurrentMatch();
            }
        }

        function clearHighlights() {
            document.querySelectorAll('mark.highlight').forEach(mark => {
                const parent = mark.parentNode;
                const badge = mark.nextSibling;
                if (badge && badge.classList && badge.classList.contains('match-badge')) {
                    badge.remove();
                }
                parent.replaceChild(document.createTextNode(mark.textContent), mark);
                parent.normalize();
            });
        }

        function highlightMatches() {
            const visible = getVisibleMatches();

            // Group matches by their text node to handle multiple matches in same node
            const nodeGroups = new Map();
            visible.forEach((match, idx) => {
                if (!nodeGroups.has(match.node)) {
                    nodeGroups.set(match.node, []);
                }
                nodeGroups.get(match.node).push({ ...match, visibleIdx: idx });
            });

            // Process each node once
            for (const [node, matches] of nodeGroups) {
                if (!node.parentNode) continue;

                const text = node.textContent;

                // Find positions of all matches in this node
                const highlights = [];
                for (const m of matches) {
                    // Use word boundaries to match whole words only
                    const regex = new RegExp('\\b' + m.text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') + '\\b', 'i');
                    const result = regex.exec(text);
                    if (result) {
                        highlights.push({
                            start: result.index,
                            end: result.index + result[0].length,
                            matchedText: result[0],
                            type: m.type,
                            idx: m.visibleIdx
                        });
                    }
                }

                if (highlights.length === 0) continue;

                // Sort by start position
                highlights.sort((a, b) => a.start - b.start);

                // Remove overlapping highlights (keep earlier one)
                const nonOverlapping = [];
                let lastEnd = 0;
                for (const h of highlights) {
                    if (h.start >= lastEnd) {
                        nonOverlapping.push(h);
                        lastEnd = h.end;
                    }
                }

                // Build fragment with all highlights
                const fragment = document.createDocumentFragment();
                let pos = 0;

                for (const h of nonOverlapping) {
                    if (h.start > pos) {
                        fragment.appendChild(document.createTextNode(text.slice(pos, h.start)));
                    }

                    const mark = document.createElement('mark');
                    mark.className = 'highlight';
                    mark.dataset.matchIndex = h.idx;
                    mark.dataset.matchType = h.type;
                    mark.textContent = h.matchedText;
                    fragment.appendChild(mark);

                    const badge = document.createElement('span');
                    badge.className = 'match-badge';
                    badge.textContent = h.type === 'spelling' ? '~spelling' :
                                       h.type === 'phonetic' ? 'sounds like' : h.type;
                    fragment.appendChild(badge);

                    pos = h.end;
                }

                if (pos < text.length) {
                    fragment.appendChild(document.createTextNode(text.slice(pos)));
                }

                node.parentNode.replaceChild(fragment, node);
            }
        }

        function getVisibleMatches() {
            return searchState.matches.filter(m => searchState.filters[m.type]);
        }

        function updateSearchUI() {
            const counts = { exact: 0, contains: 0, spelling: 0, phonetic: 0 };
            searchState.matches.forEach(m => counts[m.type]++);

            document.getElementById('exactCount').textContent = counts.exact;
            document.getElementById('containsCount').textContent = counts.contains;
            document.getElementById('spellingCount').textContent = counts.spelling;
            document.getElementById('phoneticCount').textContent = counts.phonetic;

            ['exact', 'contains', 'spelling', 'phonetic'].forEach(type => {
                const el = document.querySelector(`.search-filter[data-type="${type}"]`);
                if (counts[type] === 0) {
                    el.classList.add('disabled');
                    el.classList.remove('active');
                } else {
                    el.classList.remove('disabled');
                    el.classList.toggle('active', searchState.filters[type]);
                }
            });

            const visible = getVisibleMatches();
            const counter = document.getElementById('searchCounter');
            const prevBtn = document.getElementById('prevBtn');
            const nextBtn = document.getElementById('nextBtn');

            if (visible.length === 0) {
                counter.textContent = 'No matches';
                prevBtn.disabled = true;
                nextBtn.disabled = true;
            } else {
                const current = searchState.currentIndex + 1;
                counter.textContent = `Match ${current} of ${visible.length}`;
                prevBtn.disabled = visible.length <= 1;
                nextBtn.disabled = visible.length <= 1;
            }

            updateCurrentHighlight();
        }

        function updateCurrentHighlight() {
            document.querySelectorAll('mark.highlight.current').forEach(el => el.classList.remove('current'));
            const visible = getVisibleMatches();
            if (searchState.currentIndex >= 0 && searchState.currentIndex < visible.length) {
                const mark = document.querySelector(`mark.highlight[data-match-index="${searchState.currentIndex}"]`);
                if (mark) mark.classList.add('current');
            }
        }

        function scrollToCurrentMatch() {
            updateCurrentHighlight();
            const mark = document.querySelector('mark.highlight.current');
            if (mark) {
                mark.scrollIntoView({ behavior: 'smooth', block: 'center' });
            }
            updateSearchUI();
        }

        function nextMatch() {
            const visible = getVisibleMatches();
            if (visible.length === 0) return;
            searchState.currentIndex = (searchState.currentIndex + 1) % visible.length;
            scrollToCurrentMatch();
        }

        function prevMatch() {
            const visible = getVisibleMatches();
            if (visible.length === 0) return;
            searchState.currentIndex = (searchState.currentIndex - 1 + visible.length) % visible.length;
            scrollToCurrentMatch();
        }

        function toggleFilter(el) {
            const type = el.dataset.type;
            const counts = { exact: 0, contains: 0, spelling: 0, phonetic: 0 };
            searchState.matches.forEach(m => counts[m.type]++);

            if (counts[type] === 0) return;

            searchState.filters[type] = !searchState.filters[type];
            el.classList.toggle('active', searchState.filters[type]);

            clearHighlights();
            highlightMatches();

            const visible = getVisibleMatches();
            if (visible.length > 0) {
                searchState.currentIndex = Math.min(searchState.currentIndex, visible.length - 1);
                if (searchState.currentIndex < 0) searchState.currentIndex = 0;
            } else {
                searchState.currentIndex = -1;
            }

            updateSearchUI();
            if (searchState.currentIndex >= 0) scrollToCurrentMatch();
        }

        // Clear search when clicking on main content area
        document.addEventListener('click', (e) => {
            const mainContent = document.getElementById('mainContent');
            if (mainContent && mainContent.contains(e.target) && searchState.matches.length > 0) {
                // Don't clear if clicking on a highlight (user might want to see it)
                if (!e.target.closest('mark.highlight')) {
                    clearSearch();
                }
            }
        });

        // Keyboard navigation for search results
        document.addEventListener('keydown', (e) => {
            // Escape clears search when active
            if (e.key === 'Escape' && (searchState.matches.length > 0 || searchState.term)) {
                e.preventDefault();
                clearSearch();
                return;
            }

            if (searchState.matches.length === 0) return;

            const searchInput = document.getElementById('searchInput');
            const isInSearchInput = e.target === searchInput;

            // In search input: Enter = next, Shift+Enter = prev
            if (isInSearchInput && e.key === 'Enter') {
                e.preventDefault();
                if (e.shiftKey) {
                    prevMatch();
                } else {
                    nextMatch();
                }
                return;
            }

            // Outside inputs: arrow keys navigate
            if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;

            if (e.key === 'ArrowDown' || e.key === 'ArrowRight') {
                e.preventDefault();
                nextMatch();
            } else if (e.key === 'ArrowUp' || e.key === 'ArrowLeft') {
                e.preventDefault();
                prevMatch();
            }
        });
"##;

/// Shared JavaScript for rendering the document content
const SHARED_JS_RENDER_DOCUMENT: &str = r##"
        function renderDocument(data) {
            const container = document.getElementById('documentContent');
            let html = '';

            html += '<button class="menu-toggle" onclick="toggleSidebar()">&#9776; Menu</button>';
            html += '<div class="layout">';
            html += '<div class="sidebar" id="sidebar">';
            html += '<div class="sidebar-header">';
            html += '<svg class="logo-icon" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg">';
            html += '<rect x="8" y="6" width="32" height="36" rx="2" fill="#F0EFEB" stroke="#DDE5B6" stroke-width="1.5"/>';
            html += '<ellipse cx="24" cy="6" rx="16" ry="3" fill="#DDE5B6"/>';
            html += '<ellipse cx="24" cy="6" rx="14" ry="2" fill="#F0EFEB"/>';
            html += '<ellipse cx="24" cy="42" rx="16" ry="3" fill="#DDE5B6"/>';
            html += '<ellipse cx="24" cy="42" rx="14" ry="2" fill="#F0EFEB"/>';
            html += '<text x="24" y="28" text-anchor="middle" font-family="Georgia, serif" font-style="italic" font-size="16" font-weight="600" fill="#283618">HD</text>';
            html += '<line x1="14" y1="34" x2="34" y2="34" stroke="#B7B7A4" stroke-width="1" stroke-linecap="round"/>';
            html += '<line x1="16" y1="37" x2="32" y2="37" stroke="#B7B7A4" stroke-width="0.75" stroke-linecap="round"/>';
            html += '</svg>';
            html += '<div class="logo-text"><div class="sidebar-title">Honey Did</div>';
            if (data.meta && data.meta.creator_name) {
                html += '<div class="sidebar-subtitle">By ' + escapeHtml(data.meta.creator_name) + '</div>';
            }
            html += '</div></div>';
            html += '<div class="sidebar-search">';
            html += '<div class="search-wrapper">';
            html += '<input type="text" id="searchInput" class="search-input" placeholder="Search..." oninput="debounceSearch(this.value)" onkeydown="if(event.key===\'Escape\')clearSearch()">';
            html += '<button class="search-clear hidden" id="searchClear" onclick="clearSearch()" title="Clear search (Esc)">✕</button>';
            html += '</div>';
            html += '</div>';
            html += '<div class="search-controls" id="searchControls">';
            html += '<div class="search-nav">';
            html += '<button onclick="prevMatch()" id="prevBtn" disabled>◀</button>';
            html += '<button onclick="nextMatch()" id="nextBtn" disabled>▶</button>';
            html += '<span class="search-counter" id="searchCounter"></span>';
            html += '</div>';
            html += '<div class="search-filters">';
            html += '<span class="search-filter active" data-type="exact" onclick="toggleFilter(this)">Exact (<span id="exactCount">0</span>)</span>';
            html += '<span class="search-filter active" data-type="contains" onclick="toggleFilter(this)">Contains (<span id="containsCount">0</span>)</span>';
            html += '<span class="search-filter active" data-type="spelling" onclick="toggleFilter(this)">Spelling (<span id="spellingCount">0</span>)</span>';
            html += '<span class="search-filter active" data-type="phonetic" onclick="toggleFilter(this)">Sounds-like (<span id="phoneticCount">0</span>)</span>';
            html += '</div>';
            html += '</div>';
            html += '<div class="sidebar-nav"><div class="nav-title">Contents</div><ul class="nav-list">';

            // Check which sections have content
            const sectionLabels = {'financial': '💰 Financial', 'insurance': '🛡️ Insurance', 'bills': '📄 Bills', 'property': '🏠 Property', 'legal': '⚖️ Legal', 'digital': '💻 Digital Life', 'household': '🔧 Household', 'personal': '👤 Personal', 'contacts': '📇 Contacts', 'medical': '🏥 Medical', 'pets': '🐾 Pets'};

            function hasContent(section) {
                if (!data[section]) return false;
                const d = data[section];
                switch(section) {
                    case 'financial': return (d.bank_accounts?.length || d.credit_cards?.length || d.investments?.length || d.debts?.length || d.notes);
                    case 'insurance': return (d.policies?.length || d.notes);
                    case 'bills': return (d.bills?.length || d.notes);
                    case 'property': return (d.properties?.length || d.vehicles?.length || d.valuables?.length || d.notes);
                    case 'legal': return (d.will_location || d.power_of_attorney || d.trusts?.length || d.attorney?.name || d.notes);
                    case 'digital': return (d.email_accounts?.length || d.social_media?.length || d.password_manager?.name || d.notes);
                    case 'household': return (d.maintenance_items?.length || d.contractors?.length || d.how_things_work?.length || d.notes);
                    case 'personal': return (d.funeral_preferences || d.obituary_notes || d.messages?.length || d.notes);
                    case 'contacts': return (d.emergency_contacts?.length || d.family?.length || d.professionals?.length || d.notes);
                    case 'medical': return (d.family_members?.length || d.notes);
                    case 'pets': return (d.pets?.length || d.notes);
                    default: return false;
                }
            }

            const sections = ['financial', 'insurance', 'bills', 'property', 'legal', 'digital', 'household', 'personal', 'contacts', 'medical', 'pets'];
            sections.forEach(s => {
                if (hasContent(s)) {
                    html += '<li><a href="#' + s + '" onclick="closeSidebarOnMobile()">' + sectionLabels[s] + '</a></li>';
                }
            });

            // Add custom sections to nav
            if (data.custom_sections && data.custom_sections.length) {
                const topLevel = data.custom_sections.filter(s => !s.parent);
                topLevel.forEach(section => {
                    if (section.subsections && section.subsections.some(sub => sub.items && sub.items.length)) {
                        html += '<li><a href="#custom-' + escapeAttr(section.id) + '" onclick="closeSidebarOnMobile()">📋 ' + escapeHtml(section.name) + '</a></li>';
                    }
                });
            }
            html += '</ul></div>';
            html += '<div class="sidebar-footer"><button class="print-btn" onclick="window.print()">Print Document</button></div>';
            html += '</div>';
            html += '<div class="main-content" id="mainContent">';

            // Financial Section
            if (data.financial) {
                let content = '';
                if (data.financial.bank_accounts && data.financial.bank_accounts.length) {
                    content += '<h3>Bank Accounts</h3>';
                    data.financial.bank_accounts.forEach(a => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(a.name) + '</div>';
                        content += '<div class="item-detail">Institution: ' + escapeHtml(a.institution) + '</div>';
                        content += '<div class="item-detail">Type: ' + escapeHtml(a.account_type) + '</div>';
                        if (a.notes) content += '<div class="notes">' + escapeHtml(a.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.financial.credit_cards && data.financial.credit_cards.length) {
                    content += '<h3>Credit Cards</h3>';
                    data.financial.credit_cards.forEach(c => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(c.name) + '</div>';
                        content += '<div class="item-detail">Issuer: ' + escapeHtml(c.issuer) + '</div>';
                        if (c.notes) content += '<div class="notes">' + escapeHtml(c.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.financial.investments && data.financial.investments.length) {
                    content += '<h3>Investments</h3>';
                    data.financial.investments.forEach(inv => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(inv.name) + '</div>';
                        content += '<div class="item-detail">Institution: ' + escapeHtml(inv.institution) + '</div>';
                        content += '<div class="item-detail">Type: ' + escapeHtml(inv.account_type) + '</div>';
                        if (inv.notes) content += '<div class="notes">' + escapeHtml(inv.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.financial.debts && data.financial.debts.length) {
                    content += '<h3>Debts & Loans</h3>';
                    data.financial.debts.forEach(d => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(d.name) + '</div>';
                        content += '<div class="item-detail">Lender: ' + escapeHtml(d.lender) + '</div>';
                        if (d.notes) content += '<div class="notes">' + escapeHtml(d.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.financial.notes) content += '<div class="notes">' + escapeHtml(data.financial.notes) + '</div>';
                html += renderSection('💰 Financial Information', 'financial', content);
            }

            // Insurance Section
            if (data.insurance && ((data.insurance.policies && data.insurance.policies.length) || data.insurance.notes)) {
                let content = '';
                if (data.insurance.policies && data.insurance.policies.length) {
                    data.insurance.policies.forEach(p => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(p.policy_type) + '</div>';
                        content += '<div class="item-detail">Provider: ' + escapeHtml(p.provider) + '</div>';
                        content += '<div class="item-detail">Policy #: ' + escapeHtml(p.policy_number) + '</div>';
                        if (p.contact) content += '<div class="item-detail">Contact: ' + escapeHtml(p.contact) + '</div>';
                        if (p.notes) content += '<div class="notes">' + escapeHtml(p.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.insurance.notes) content += '<div class="notes">' + escapeHtml(data.insurance.notes) + '</div>';
                html += renderSection('🛡️ Insurance', 'insurance', content);
            }

            // Bills Section
            if (data.bills && ((data.bills.bills && data.bills.bills.length) || data.bills.notes)) {
                let content = '';
                if (data.bills.bills && data.bills.bills.length) {
                    data.bills.bills.forEach(b => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(b.name) + '</div>';
                        content += '<div class="item-detail">Provider: ' + escapeHtml(b.provider) + '</div>';
                        content += '<div class="item-detail">Amount: ' + escapeHtml(b.amount) + '</div>';
                        content += '<div class="item-detail">Due Day: ' + escapeHtml(b.due_day) + '</div>';
                        content += '<div class="item-detail">Auto-pay: ' + (b.autopay ? 'Yes' : 'No') + '</div>';
                        if (b.notes) content += '<div class="notes">' + escapeHtml(b.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.bills.notes) content += '<div class="notes">' + escapeHtml(data.bills.notes) + '</div>';
                html += renderSection('📄 Bills', 'bills', content);
            }

            // Property Section
            if (data.property) {
                let content = '';
                if (data.property.properties && data.property.properties.length) {
                    content += '<h3>Properties</h3>';
                    data.property.properties.forEach(p => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(p.name) + '</div>';
                        content += '<div class="item-detail">Address: ' + escapeHtml(p.address) + '</div>';
                        if (p.notes) content += '<div class="notes">' + escapeHtml(p.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.property.vehicles && data.property.vehicles.length) {
                    content += '<h3>Vehicles</h3>';
                    data.property.vehicles.forEach(v => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(v.name) + '</div>';
                        content += '<div class="item-detail">' + escapeHtml(v.details) + '</div>';
                        if (v.notes) content += '<div class="notes">' + escapeHtml(v.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.property.valuables && data.property.valuables.length) {
                    content += '<h3>Valuables</h3>';
                    data.property.valuables.forEach(v => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(v.name) + '</div>';
                        content += '<div class="item-detail">Location: ' + escapeHtml(v.location) + '</div>';
                        if (v.notes) content += '<div class="notes">' + escapeHtml(v.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.property.notes) content += '<div class="notes">' + escapeHtml(data.property.notes) + '</div>';
                html += renderSection('🏠 Property', 'property', content);
            }

            // Legal Section
            if (data.legal) {
                let content = '';
                if (data.legal.will_location) content += '<div class="item-detail"><strong>Will Location:</strong> ' + escapeHtml(data.legal.will_location) + '</div>';
                if (data.legal.power_of_attorney) content += '<div class="item-detail"><strong>Power of Attorney:</strong> ' + escapeHtml(data.legal.power_of_attorney) + '</div>';
                if (data.legal.attorney && data.legal.attorney.name) {
                    content += '<h3>Attorney</h3>' + renderContact(data.legal.attorney);
                }
                if (data.legal.trusts && data.legal.trusts.length) {
                    content += '<h3>Trusts</h3>';
                    data.legal.trusts.forEach(t => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(t.name) + '</div>';
                        content += '<div class="item-detail">Trustee: ' + escapeHtml(t.trustee) + '</div>';
                        if (t.notes) content += '<div class="notes">' + escapeHtml(t.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.legal.notes) content += '<div class="notes">' + escapeHtml(data.legal.notes) + '</div>';
                html += renderSection('⚖️ Legal Documents', 'legal', content);
            }

            // Digital Section
            if (data.digital) {
                let content = '';
                if (data.digital.password_manager && data.digital.password_manager.name) {
                    content += '<h3>Password Manager</h3>';
                    content += '<div class="item"><div class="item-title">' + escapeHtml(data.digital.password_manager.name) + '</div>';
                    if (data.digital.password_manager.master_password_hint) content += '<div class="item-detail">Hint: ' + escapeHtml(data.digital.password_manager.master_password_hint) + '</div>';
                    if (data.digital.password_manager.recovery_method) content += '<div class="item-detail">Recovery: ' + escapeHtml(data.digital.password_manager.recovery_method) + '</div>';
                    content += '</div>';
                }
                if (data.digital.email_accounts && data.digital.email_accounts.length) {
                    content += '<h3>Email Accounts</h3>';
                    data.digital.email_accounts.forEach(e => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(e.name) + '</div>';
                        content += '<div class="item-detail">Username: ' + escapeHtml(e.username) + '</div>';
                        if (e.recovery_hint) content += '<div class="item-detail">Recovery: ' + escapeHtml(e.recovery_hint) + '</div>';
                        if (e.notes) content += '<div class="notes">' + escapeHtml(e.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.digital.social_media && data.digital.social_media.length) {
                    content += '<h3>Social Media</h3>';
                    data.digital.social_media.forEach(s => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(s.name) + '</div>';
                        content += '<div class="item-detail">Username: ' + escapeHtml(s.username) + '</div>';
                        if (s.notes) content += '<div class="notes">' + escapeHtml(s.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.digital.notes) content += '<div class="notes">' + escapeHtml(data.digital.notes) + '</div>';
                html += renderSection('💻 Digital Life', 'digital', content);
            }

            // Household Section
            if (data.household) {
                let content = '';
                if (data.household.maintenance_items && data.household.maintenance_items.length) {
                    content += '<h3>Maintenance</h3>';
                    data.household.maintenance_items.forEach(m => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(m.name) + '</div>';
                        content += '<div class="item-detail">Frequency: ' + escapeHtml(m.frequency) + '</div>';
                        content += '<div class="item-detail">Last Done: ' + escapeHtml(m.last_done) + '</div>';
                        if (m.notes) content += '<div class="notes">' + escapeHtml(m.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.household.contractors && data.household.contractors.length) {
                    content += '<h3>Contractors</h3>';
                    data.household.contractors.forEach(c => {
                        content += '<div class="item">' + renderContact(c) + '</div>';
                    });
                }
                if (data.household.how_things_work && data.household.how_things_work.length) {
                    content += '<h3>How Things Work</h3>';
                    data.household.how_things_work.forEach(h => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(h.name) + '</div>';
                        content += '<div class="item-detail">' + escapeHtml(h.instructions) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.household.notes) content += '<div class="notes">' + escapeHtml(data.household.notes) + '</div>';
                html += renderSection('🔧 Household', 'household', content);
            }

            // Personal Section
            if (data.personal) {
                let content = '';
                if (data.personal.funeral_preferences) {
                    content += '<h3>Funeral Preferences</h3><div class="item">' + escapeHtml(data.personal.funeral_preferences) + '</div>';
                }
                if (data.personal.obituary_notes) {
                    content += '<h3>Obituary Notes</h3><div class="item">' + escapeHtml(data.personal.obituary_notes) + '</div>';
                }
                if (data.personal.messages && data.personal.messages.length) {
                    content += '<h3>Personal Messages</h3>';
                    data.personal.messages.forEach(m => {
                        content += '<div class="item"><div class="item-title">To: ' + escapeHtml(m.recipient) + '</div>';
                        content += '<div class="item-detail">' + escapeHtml(m.message) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.personal.notes) content += '<div class="notes">' + escapeHtml(data.personal.notes) + '</div>';
                html += renderSection('👤 Personal Wishes', 'personal', content);
            }

            // Contacts Section
            if (data.contacts) {
                let content = '';
                if (data.contacts.emergency_contacts && data.contacts.emergency_contacts.length) {
                    content += '<h3>Emergency Contacts</h3>';
                    data.contacts.emergency_contacts.forEach(c => {
                        content += '<div class="item">' + renderContact(c) + '</div>';
                    });
                }
                if (data.contacts.family && data.contacts.family.length) {
                    content += '<h3>Family</h3>';
                    data.contacts.family.forEach(c => {
                        content += '<div class="item">' + renderContact(c) + '</div>';
                    });
                }
                if (data.contacts.professionals && data.contacts.professionals.length) {
                    content += '<h3>Professional Contacts</h3>';
                    data.contacts.professionals.forEach(c => {
                        content += '<div class="item">' + renderContact(c) + '</div>';
                    });
                }
                if (data.contacts.notes) content += '<div class="notes">' + escapeHtml(data.contacts.notes) + '</div>';
                html += renderSection('📇 Important Contacts', 'contacts', content);
            }

            // Medical Section
            if (data.medical && ((data.medical.family_members && data.medical.family_members.length) || data.medical.notes)) {
                let content = '';
                if (data.medical.family_members && data.medical.family_members.length) {
                    data.medical.family_members.forEach(m => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(m.name) + '</div>';
                        if (m.conditions && m.conditions.length) content += '<div class="item-detail"><strong>Conditions:</strong> ' + m.conditions.map(c => escapeHtml(c)).join(', ') + '</div>';
                        if (m.allergies && m.allergies.length) content += '<div class="item-detail"><strong>Allergies:</strong> ' + m.allergies.map(a => escapeHtml(a)).join(', ') + '</div>';
                        if (m.doctors && m.doctors.length) {
                            content += '<div class="item-detail"><strong>Doctors:</strong></div>';
                            m.doctors.forEach(doc => {
                                let docInfo = escapeHtml(doc.name);
                                if (doc.specialty) docInfo += ' (' + escapeHtml(doc.specialty) + ')';
                                if (doc.phone) docInfo += ' - ' + escapeHtml(doc.phone);
                                content += '<div class="item-detail">&nbsp;&nbsp;' + docInfo + '</div>';
                                if (doc.notes) content += '<div class="item-detail">&nbsp;&nbsp;&nbsp;&nbsp;<em>' + escapeHtml(doc.notes) + '</em></div>';
                            });
                        }
                        if (m.medications && m.medications.length) {
                            content += '<div class="item-detail"><strong>Medications:</strong></div>';
                            m.medications.forEach(med => {
                                content += '<div class="item-detail">&nbsp;&nbsp;' + escapeHtml(med.name) + ' - ' + escapeHtml(med.dosage) + ' (' + escapeHtml(med.frequency) + ')</div>';
                            });
                        }
                        if (m.pharmacy && m.pharmacy.name) content += '<div class="item-detail"><strong>Pharmacy:</strong> ' + escapeHtml(m.pharmacy.name) + ' ' + escapeHtml(m.pharmacy.phone || '') + '</div>';
                        if (m.notes) content += '<div class="notes">' + escapeHtml(m.notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.medical.notes) content += '<div class="notes">' + escapeHtml(data.medical.notes) + '</div>';
                html += renderSection('🏥 Medical Information', 'medical', content);
            }

            // Pets Section
            if (data.pets && ((data.pets.pets && data.pets.pets.length) || data.pets.notes)) {
                let content = '';
                if (data.pets.pets && data.pets.pets.length) {
                    data.pets.pets.forEach(p => {
                        content += '<div class="item"><div class="item-title">' + escapeHtml(p.name) + '</div>';
                        content += '<div class="item-detail">Species: ' + escapeHtml(p.species) + '</div>';
                        content += '<div class="item-detail">Breed: ' + escapeHtml(p.breed) + '</div>';
                        if (p.vet && p.vet.name) content += '<div class="item-detail"><strong>Vet:</strong> ' + escapeHtml(p.vet.name) + ' ' + escapeHtml(p.vet.phone || '') + '</div>';
                        if (p.medications && p.medications.length) {
                            content += '<div class="item-detail"><strong>Medications:</strong></div>';
                            p.medications.forEach(med => {
                                content += '<div class="item-detail">&nbsp;&nbsp;' + escapeHtml(med.name) + ' - ' + escapeHtml(med.dosage) + '</div>';
                            });
                        }
                        if (p.feeding) content += '<div class="item-detail"><strong>Feeding:</strong> ' + escapeHtml(p.feeding) + '</div>';
                        if (p.care_notes) content += '<div class="notes">' + escapeHtml(p.care_notes) + '</div>';
                        content += '</div>';
                    });
                }
                if (data.pets.notes) content += '<div class="notes">' + escapeHtml(data.pets.notes) + '</div>';
                html += renderSection('🐾 Pets', 'pets', content);
            }

            // Custom Sections
            if (data.custom_sections && data.custom_sections.length) {
                // Separate top-level from preset subsections
                const topLevel = data.custom_sections.filter(s => !s.parent);
                const presetSubs = data.custom_sections.filter(s => s.parent);

                // Render top-level custom sections
                topLevel.forEach(section => {
                    if (!section.subsections || !section.subsections.length) return;
                    let content = '';
                    section.subsections.forEach(sub => {
                        content += renderCustomSubsection(sub);
                    });
                    if (content) html += renderSection('📋 ' + escapeHtml(section.name), 'custom-' + section.id, content);
                });

                // Render preset subsections grouped by parent
                const byParent = {};
                presetSubs.forEach(section => {
                    if (!byParent[section.parent]) byParent[section.parent] = [];
                    byParent[section.parent].push(section);
                });

                const parentLabels = {
                    'financial': 'Financial (Custom)',
                    'insurance': 'Insurance (Custom)',
                    'bills': 'Bills (Custom)',
                    'property': 'Property (Custom)',
                    'legal': 'Legal (Custom)',
                    'digital': 'Digital Life (Custom)',
                    'household': 'Household (Custom)',
                    'personal': 'Personal Wishes (Custom)',
                    'contacts': 'Contacts (Custom)',
                    'medical': 'Medical (Custom)',
                    'pets': 'Pets (Custom)'
                };

                Object.keys(byParent).forEach(parentName => {
                    let content = '';
                    byParent[parentName].forEach(section => {
                        if (section.subsections) {
                            section.subsections.forEach(sub => {
                                content += renderCustomSubsection(sub);
                            });
                        }
                    });
                    if (content) {
                        const label = parentLabels[parentName] || parentName + ' (Custom)';
                        html += renderSection('📋 ' + escapeHtml(label), 'custom-' + parentName, content);
                    }
                });
            }

            html += '</div>'; // End main-content
            html += '</div>'; // End layout

            container.innerHTML = html;
            buildSearchIndex();
        }
"##;

// ============================================================================
// PASSPHRASE-SPECIFIC TEMPLATE COMPONENTS
// ============================================================================

/// CSS specific to passphrase-based unlock
const PASSPHRASE_CSS: &str = r##"
        .lock-screen { display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; text-align: center; background: linear-gradient(145deg, #F0EFEB 0%, #D4D4D4 100%); }
        .lock-icon { font-size: 4rem; margin-bottom: 1rem; }
        .lock-title { font-size: 1.75rem; font-weight: 600; color: #283618; margin-bottom: 0.5rem; }
        .lock-subtitle { color: #606C38; margin-bottom: 2rem; font-size: 0.95rem; }
        .password-form { display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 320px; }
        .password-input { padding: 14px 16px; font-size: 1rem; border: 2px solid #D4D4D4; border-radius: 10px; text-align: center; background: white; transition: border-color 0.2s, box-shadow 0.2s; }
        .password-input:focus { outline: none; border-color: #283618; box-shadow: 0 0 0 3px rgba(40, 54, 24, 0.1); }
        .unlock-btn { padding: 14px 28px; font-size: 1rem; font-weight: 500; background: #283618; color: #F0EFEB; border: none; border-radius: 10px; cursor: pointer; transition: background 0.2s, transform 0.1s; }
        .unlock-btn:hover { background: #1a2410; }
        .unlock-btn:active { transform: scale(0.98); }
        .error { color: #9B2C2C; font-size: 0.9rem; margin-top: 1rem; }
        /* Welcome Screen Styles */
        .welcome-screen { position: fixed; inset: 0; background: linear-gradient(145deg, #283618 0%, #1a2410 100%); display: flex; flex-direction: column; align-items: center; justify-content: center; z-index: 2000; opacity: 1; transition: opacity 0.5s ease; }
        .welcome-screen.hidden { opacity: 0; pointer-events: none; }
        .welcome-slide { max-width: 600px; padding: 40px; text-align: center; opacity: 0; transform: translateY(20px); transition: opacity 0.5s ease, transform 0.5s ease; }
        .welcome-slide.visible { opacity: 1; transform: translateY(0); }
        .welcome-slide-text { font-size: 1.5rem; line-height: 1.8; color: #F0EFEB; font-weight: 400; white-space: pre-wrap; }
        .welcome-continue { margin-top: 40px; padding: 14px 32px; background: rgba(240, 239, 235, 0.15); color: #F0EFEB; border: 2px solid rgba(240, 239, 235, 0.3); border-radius: 10px; cursor: pointer; font-size: 1rem; font-weight: 500; transition: all 0.2s; }
        .welcome-continue:hover { background: rgba(240, 239, 235, 0.25); border-color: rgba(240, 239, 235, 0.5); }
        .welcome-progress { position: absolute; bottom: 40px; display: flex; gap: 8px; }
        .welcome-dot { width: 8px; height: 8px; border-radius: 50%; background: rgba(240, 239, 235, 0.3); transition: background 0.3s; }
        .welcome-dot.active { background: #F0EFEB; }
        .welcome-timer { position: absolute; bottom: 20px; width: 200px; height: 3px; background: rgba(240, 239, 235, 0.2); border-radius: 2px; overflow: hidden; }
        .welcome-timer-bar { height: 100%; background: #F0EFEB; width: 0%; transition: width linear; }
"##;

/// JavaScript specific to passphrase-based decryption
const PASSPHRASE_JS_DECRYPT: &str = r##"
        const PBKDF2_ITERATIONS = 600000;

        async function unlock(event) {
            event.preventDefault();
            const passphrase = document.getElementById('passphrase').value;
            if (!passphrase) return false;

            try {
                const salt = Uint8Array.from(atob(ENCRYPTED_DATA.salt), c => c.charCodeAt(0));
                const nonce = Uint8Array.from(atob(ENCRYPTED_DATA.nonce), c => c.charCodeAt(0));
                const ciphertext = Uint8Array.from(atob(ENCRYPTED_DATA.ciphertext), c => c.charCodeAt(0));

                const encoder = new TextEncoder();
                const keyMaterial = await crypto.subtle.importKey(
                    'raw', encoder.encode(passphrase), 'PBKDF2', false, ['deriveKey']
                );

                const key = await crypto.subtle.deriveKey(
                    { name: 'PBKDF2', salt: salt, iterations: PBKDF2_ITERATIONS, hash: 'SHA-256' },
                    keyMaterial,
                    { name: 'AES-GCM', length: 256 },
                    false,
                    ['decrypt']
                );

                const decrypted = await crypto.subtle.decrypt(
                    { name: 'AES-GCM', iv: nonce },
                    key,
                    ciphertext
                );

                const decoder = new TextDecoder();
                const json = decoder.decode(decrypted);
                const data = JSON.parse(json);

                renderDocument(data);
                document.getElementById('lockScreen').style.display = 'none';
                document.getElementById('content').classList.add('visible');
            } catch (err) {
                document.getElementById('error').textContent = 'Incorrect passphrase. Please try again.';
                document.getElementById('error').style.display = 'block';
            }
            return false;
        }
"##;

/// JavaScript for welcome screen (passphrase template)
const PASSPHRASE_JS_WELCOME: &str = r##"
        let currentWelcomeSlide = 0;
        let welcomeTimer = null;

        function initWelcome() {
            if (!WELCOME_SLIDES || WELCOME_SLIDES.length === 0) {
                document.getElementById('welcomeScreen').classList.add('hidden');
                document.getElementById('lockScreen').style.display = 'flex';
                return;
            }

            document.getElementById('welcomeScreen').classList.remove('hidden');

            const progressContainer = document.getElementById('welcomeProgress');
            progressContainer.innerHTML = '';
            WELCOME_SLIDES.forEach((_, i) => {
                const dot = document.createElement('div');
                dot.className = 'welcome-dot' + (i === 0 ? ' active' : '');
                progressContainer.appendChild(dot);
            });

            showWelcomeSlide(0);
        }

        function showWelcomeSlide(index) {
            if (welcomeTimer) {
                clearTimeout(welcomeTimer);
                welcomeTimer = null;
            }

            currentWelcomeSlide = index;
            const slide = WELCOME_SLIDES[index];
            const textEl = document.getElementById('welcomeText');
            const slideEl = document.getElementById('welcomeSlide');
            const btnEl = document.getElementById('welcomeContinue');
            const timerEl = document.getElementById('welcomeTimer');
            const timerBar = document.getElementById('welcomeTimerBar');

            slideEl.classList.remove('visible');

            setTimeout(() => {
                textEl.textContent = slide.text;

                const isLastSlide = index === WELCOME_SLIDES.length - 1;
                btnEl.textContent = isLastSlide ? 'Continue to Document' : 'Continue';

                const dots = document.querySelectorAll('.welcome-dot');
                dots.forEach((dot, i) => dot.classList.toggle('active', i === index));

                slideEl.classList.add('visible');

                if (slide.transition && slide.transition.type === 'auto') {
                    const duration = slide.transition.seconds || 5;
                    timerEl.style.display = 'block';
                    timerBar.style.width = '0%';
                    timerBar.style.transition = 'none';

                    setTimeout(() => {
                        timerBar.style.transition = `width ${duration}s linear`;
                        timerBar.style.width = '100%';
                    }, 50);

                    welcomeTimer = setTimeout(() => nextWelcomeSlide(), duration * 1000);
                } else {
                    timerEl.style.display = 'none';
                }
            }, 300);
        }

        function nextWelcomeSlide() {
            if (welcomeTimer) {
                clearTimeout(welcomeTimer);
                welcomeTimer = null;
            }

            if (currentWelcomeSlide < WELCOME_SLIDES.length - 1) {
                showWelcomeSlide(currentWelcomeSlide + 1);
            } else {
                document.getElementById('welcomeScreen').classList.add('hidden');
                document.getElementById('lockScreen').style.display = 'flex';
                document.getElementById('passphrase').focus();
            }
        }

        document.addEventListener('DOMContentLoaded', initWelcome);

        // Allow Enter key to advance welcome slides
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const welcomeScreen = document.getElementById('welcomeScreen');
                if (welcomeScreen && !welcomeScreen.classList.contains('hidden')) {
                    e.preventDefault();
                    nextWelcomeSlide();
                }
            }
        });
"##;

// ============================================================================
// QUESTION-BASED TEMPLATE COMPONENTS
// ============================================================================

/// CSS specific to question-based unlock
const QUESTION_CSS: &str = r##"
        /* Slide Screen */
        .slide-screen { position: fixed; inset: 0; background: linear-gradient(145deg, #283618 0%, #1a2410 100%); display: flex; flex-direction: column; align-items: center; justify-content: center; z-index: 2000; }
        .slide-screen.hidden { display: none; }
        .slide-container { max-width: 600px; padding: 40px; text-align: center; }
        .slide-text { font-size: 1.5rem; line-height: 1.8; color: #F0EFEB; font-weight: 400; white-space: pre-wrap; margin-bottom: 24px; }
        .slide-input { width: 100%; max-width: 400px; padding: 14px 16px; font-size: 1.1rem; border: 2px solid rgba(240, 239, 235, 0.3); border-radius: 10px; text-align: center; background: rgba(255,255,255,0.1); color: #F0EFEB; margin-bottom: 16px; }
        .slide-input::placeholder { color: rgba(240, 239, 235, 0.5); }
        .slide-input:focus { outline: none; border-color: rgba(240, 239, 235, 0.6); background: rgba(255,255,255,0.15); }
        .slide-btn { padding: 14px 32px; background: rgba(240, 239, 235, 0.15); color: #F0EFEB; border: 2px solid rgba(240, 239, 235, 0.3); border-radius: 10px; cursor: pointer; font-size: 1rem; font-weight: 500; transition: all 0.2s; }
        .slide-btn:hover { background: rgba(240, 239, 235, 0.25); border-color: rgba(240, 239, 235, 0.5); }
        .slide-progress { position: absolute; bottom: 40px; display: flex; gap: 8px; }
        .slide-dot { width: 10px; height: 10px; border-radius: 50%; background: rgba(240, 239, 235, 0.3); transition: background 0.3s; }
        .slide-dot.active { background: #F0EFEB; }
        .slide-dot.question { border: 2px solid rgba(240, 239, 235, 0.5); }

        /* Retry Screen */
        .retry-screen { position: fixed; inset: 0; background: linear-gradient(145deg, #F0EFEB 0%, #D4D4D4 100%); display: flex; flex-direction: column; align-items: center; justify-content: center; z-index: 2000; padding: 20px; }
        .retry-screen.hidden { display: none; }
        .retry-container { max-width: 500px; width: 100%; text-align: center; }
        .retry-title { font-size: 1.25rem; color: #283618; margin-bottom: 8px; }
        .retry-subtitle { color: #606C38; margin-bottom: 24px; }
        .retry-questions { text-align: left; margin-bottom: 24px; }
        .retry-question { background: white; border: 1px solid #D4D4D4; border-radius: 8px; padding: 16px; margin-bottom: 12px; }
        .retry-question label { display: block; font-weight: 500; color: #283618; margin-bottom: 8px; }
        .retry-question input { width: 100%; padding: 10px 12px; border: 1px solid #D4D4D4; border-radius: 6px; font-size: 1rem; }
        .retry-question input:focus { outline: none; border-color: #283618; }
        .retry-btn { padding: 14px 28px; font-size: 1rem; font-weight: 500; background: #283618; color: #F0EFEB; border: none; border-radius: 10px; cursor: pointer; transition: background 0.2s; }
        .retry-btn:hover { background: #1a2410; }
        .fallback-link { background: none; border: none; color: #606C38; font-size: 0.9rem; margin-top: 16px; cursor: pointer; text-decoration: underline; }
        .fallback-link:hover { color: #283618; }
        .error-msg { color: #9B2C2C; margin-top: 16px; }

        /* Passphrase Screen */
        .passphrase-screen { position: fixed; inset: 0; background: linear-gradient(145deg, #F0EFEB 0%, #D4D4D4 100%); display: flex; flex-direction: column; align-items: center; justify-content: center; z-index: 2000; }
        .passphrase-screen.hidden { display: none; }
        .passphrase-container { max-width: 320px; text-align: center; }
        .passphrase-icon { font-size: 4rem; margin-bottom: 1rem; }
        .passphrase-title { font-size: 1.5rem; font-weight: 600; color: #283618; margin-bottom: 2rem; }
        .passphrase-input { width: 100%; padding: 14px 16px; font-size: 1rem; border: 2px solid #D4D4D4; border-radius: 10px; text-align: center; background: white; margin-bottom: 16px; }
        .passphrase-input:focus { outline: none; border-color: #283618; }
        .back-link { background: none; border: none; color: #606C38; font-size: 0.9rem; margin-top: 16px; cursor: pointer; }
        .back-link:hover { color: #283618; text-decoration: underline; }

        /* Unlocking screen */
        .unlocking-screen { position: fixed; inset: 0; background: linear-gradient(145deg, #F0EFEB 0%, #D4D4D4 100%); display: flex; flex-direction: column; align-items: center; justify-content: center; z-index: 2000; }
        .unlocking-screen.hidden { display: none; }
        .unlocking-icon { font-size: 4rem; margin-bottom: 1rem; animation: pulse 1.5s infinite; }
        @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
        .unlocking-text { color: #283618; font-size: 1.1rem; }
"##;

/// JavaScript specific to question-based decryption
const QUESTION_JS_DECRYPT: &str = r##"
        const PBKDF2_ITERATIONS = 600000;

        async function decryptWithQuestionKey(passphrase) {
            const qk = ENCRYPTED_DATA.question_key;
            const salt = Uint8Array.from(atob(qk.salt), c => c.charCodeAt(0));
            const nonce = Uint8Array.from(atob(qk.nonce), c => c.charCodeAt(0));
            const ciphertext = Uint8Array.from(atob(qk.ciphertext), c => c.charCodeAt(0));

            const key = await deriveKey(passphrase, salt);
            const docKeyBytes = await crypto.subtle.decrypt(
                { name: 'AES-GCM', iv: nonce },
                key,
                ciphertext
            );

            await decryptDocument(new Uint8Array(docKeyBytes));
        }

        async function decryptWithPassphraseKey(passphrase) {
            const pk = ENCRYPTED_DATA.passphrase_key;
            const salt = Uint8Array.from(atob(pk.salt), c => c.charCodeAt(0));
            const nonce = Uint8Array.from(atob(pk.nonce), c => c.charCodeAt(0));
            const ciphertext = Uint8Array.from(atob(pk.ciphertext), c => c.charCodeAt(0));

            const key = await deriveKey(passphrase, salt);
            const docKeyBytes = await crypto.subtle.decrypt(
                { name: 'AES-GCM', iv: nonce },
                key,
                ciphertext
            );

            await decryptDocument(new Uint8Array(docKeyBytes));
        }

        async function decryptDocument(docKey) {
            const doc = ENCRYPTED_DATA.document;
            const nonce = Uint8Array.from(atob(doc.nonce), c => c.charCodeAt(0));
            const ciphertext = Uint8Array.from(atob(doc.ciphertext), c => c.charCodeAt(0));

            const cryptoKey = await crypto.subtle.importKey(
                'raw', docKey, { name: 'AES-GCM' }, false, ['decrypt']
            );

            const decrypted = await crypto.subtle.decrypt(
                { name: 'AES-GCM', iv: nonce },
                cryptoKey,
                ciphertext
            );

            const decoder = new TextDecoder();
            const jsonString = decoder.decode(decrypted);
            const data = JSON.parse(jsonString);

            renderDocument(data);
            document.getElementById('unlockingScreen').classList.add('hidden');
            document.getElementById('content').classList.add('visible');
        }

        async function deriveKey(passphrase, salt) {
            const encoder = new TextEncoder();
            const keyMaterial = await crypto.subtle.importKey(
                'raw', encoder.encode(passphrase), 'PBKDF2', false, ['deriveKey']
            );
            return await crypto.subtle.deriveKey(
                { name: 'PBKDF2', salt: salt, iterations: PBKDF2_ITERATIONS, hash: 'SHA-256' },
                keyMaterial,
                { name: 'AES-GCM', length: 256 },
                false,
                ['decrypt']
            );
        }
"##;

/// JavaScript for slide navigation (question-based template)
const QUESTION_JS_SLIDES: &str = r##"
        let currentSlide = 0;
        let answers = {};
        let attempts = 0;
        const MAX_ATTEMPTS = 5;

        function initSlides() {
            if (SLIDES.length === 0) {
                showRetryScreen();
                return;
            }

            const progressContainer = document.getElementById('slideProgress');
            progressContainer.innerHTML = '';
            for (let i = 0; i < SLIDES.length; i++) {
                const dot = document.createElement('div');
                dot.className = 'slide-dot' + (SLIDES[i].type === 'question' ? ' question' : '') + (i === 0 ? ' active' : '');
                progressContainer.appendChild(dot);
            }

            showSlide(0);
        }

        function showSlide(index) {
            if (index >= SLIDES.length) {
                attemptUnlock();
                return;
            }

            currentSlide = index;
            const slide = SLIDES[index];
            const textEl = document.getElementById('slideText');
            const inputEl = document.getElementById('slideInput');
            const btnEl = document.getElementById('slideBtn');

            textEl.textContent = slide.text;

            if (slide.type === 'question') {
                inputEl.style.display = 'block';
                inputEl.value = answers[slide.id] || '';
                inputEl.focus();
                btnEl.textContent = 'Continue';
            } else {
                inputEl.style.display = 'none';
                btnEl.textContent = 'Continue';
            }

            // Update progress dots
            const dots = document.querySelectorAll('.slide-dot');
            dots.forEach((dot, i) => dot.classList.toggle('active', i === index));
        }

        function nextSlide() {
            const slide = SLIDES[currentSlide];
            if (slide.type === 'question') {
                const input = document.getElementById('slideInput');
                answers[slide.id] = input.value.toLowerCase().trim();
            }
            showSlide(currentSlide + 1);
        }

        async function attemptUnlock() {
            const questionSlides = SLIDES.filter(s => s.type === 'question');
            const passphrase = questionSlides.map(s => answers[s.id] || '').join('');

            document.getElementById('slideScreen').classList.add('hidden');
            document.getElementById('unlockingScreen').classList.remove('hidden');

            try {
                await decryptWithQuestionKey(passphrase);
            } catch (err) {
                attempts++;
                showRetryScreen();
            }
        }

        function showRetryScreen() {
            document.getElementById('slideScreen').classList.add('hidden');
            document.getElementById('unlockingScreen').classList.add('hidden');
            document.getElementById('passphraseScreen').classList.add('hidden');
            document.getElementById('retryScreen').classList.remove('hidden');

            const counter = document.getElementById('attemptCounter');
            if (attempts >= MAX_ATTEMPTS) {
                counter.textContent = 'Having trouble? You can keep trying, or use the passphrase if you have it.';
            } else {
                counter.textContent = 'Attempt ' + attempts + ' of ' + MAX_ATTEMPTS;
            }

            // Build retry questions form
            const container = document.getElementById('retryQuestions');
            container.innerHTML = '';
            const questionSlides = SLIDES.filter(s => s.type === 'question');
            questionSlides.forEach((slide, i) => {
                const div = document.createElement('div');
                div.className = 'retry-question';
                div.innerHTML = '<label>' + escapeHtml(slide.text) + '</label><input type="text" id="retry-' + slide.id + '" value="' + escapeHtml(answers[slide.id] || '') + '">';
                container.appendChild(div);
            });
        }

        async function retryUnlock() {
            const questionSlides = SLIDES.filter(s => s.type === 'question');
            questionSlides.forEach(slide => {
                const input = document.getElementById('retry-' + slide.id);
                if (input) answers[slide.id] = input.value.toLowerCase().trim();
            });

            const passphrase = questionSlides.map(s => answers[s.id] || '').join('');

            document.getElementById('retryScreen').classList.add('hidden');
            document.getElementById('unlockingScreen').classList.remove('hidden');

            try {
                await decryptWithQuestionKey(passphrase);
            } catch (err) {
                attempts++;
                document.getElementById('retryError').textContent = 'Some answers weren\'t quite right.';
                document.getElementById('retryError').style.display = 'block';
                showRetryScreen();
            }
        }

        function showPassphraseScreen() {
            document.getElementById('retryScreen').classList.add('hidden');
            document.getElementById('passphraseScreen').classList.remove('hidden');
            document.getElementById('passphraseInput').focus();
        }

        async function unlockWithPassphrase() {
            const passphrase = document.getElementById('passphraseInput').value;
            if (!passphrase) {
                document.getElementById('passphraseError').textContent = 'Please enter a passphrase.';
                document.getElementById('passphraseError').style.display = 'block';
                return;
            }

            document.getElementById('passphraseScreen').classList.add('hidden');
            document.getElementById('unlockingScreen').classList.remove('hidden');

            try {
                await decryptWithPassphraseKey(passphrase);
            } catch (err) {
                document.getElementById('passphraseError').textContent = 'Incorrect passphrase.';
                document.getElementById('passphraseError').style.display = 'block';
                document.getElementById('unlockingScreen').classList.add('hidden');
                document.getElementById('passphraseScreen').classList.remove('hidden');
            }
        }

        document.getElementById('passphraseInput').addEventListener('keydown', (e) => {
            if (e.key === 'Enter') unlockWithPassphrase();
        });

        // Allow Enter key to advance slides
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const slideScreen = document.getElementById('slideScreen');
                if (slideScreen && !slideScreen.classList.contains('hidden')) {
                    e.preventDefault();
                    nextSlide();
                }
            }
        });

        document.addEventListener('DOMContentLoaded', initSlides);
"##;

// ============================================================================
// TEMPLATE GENERATION FUNCTIONS
// ============================================================================

fn generate_html_template(encrypted_data: &str, creator_name: &str, welcome_slides_json: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Honey Did - Legacy Document</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap" rel="stylesheet">
    <style>
{SHARED_CSS}
{PASSPHRASE_CSS}
    </style>
</head>
<body>
    <div id="welcomeScreen" class="welcome-screen hidden">
        <div id="welcomeSlide" class="welcome-slide">
            <div id="welcomeText" class="welcome-slide-text"></div>
            <button id="welcomeContinue" class="welcome-continue" onclick="nextWelcomeSlide()">Continue</button>
        </div>
        <div id="welcomeProgress" class="welcome-progress"></div>
        <div id="welcomeTimer" class="welcome-timer"><div id="welcomeTimerBar" class="welcome-timer-bar"></div></div>
    </div>
    <div id="lockScreen" class="lock-screen" style="display: none;">
        <div class="lock-icon">🔐</div>
        <h1 class="lock-title">Honey Did</h1>
        <p class="lock-subtitle">This document was prepared by {creator_name}<br>to help you in their absence.</p>
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
        const WELCOME_SLIDES = {welcome_slides_json};
{SHARED_JS_UTILS}
{SHARED_JS_SEARCH}
{SHARED_JS_RENDER_DOCUMENT}
{PASSPHRASE_JS_DECRYPT}
{PASSPHRASE_JS_WELCOME}
    </script>
</body>
</html>"##,
        SHARED_CSS = SHARED_CSS,
        PASSPHRASE_CSS = PASSPHRASE_CSS,
        creator_name = creator_name,
        encrypted_data = encrypted_data,
        welcome_slides_json = welcome_slides_json,
        SHARED_JS_UTILS = SHARED_JS_UTILS,
        SHARED_JS_SEARCH = SHARED_JS_SEARCH,
        SHARED_JS_RENDER_DOCUMENT = SHARED_JS_RENDER_DOCUMENT,
        PASSPHRASE_JS_DECRYPT = PASSPHRASE_JS_DECRYPT,
        PASSPHRASE_JS_WELCOME = PASSPHRASE_JS_WELCOME,
    )
}

fn generate_question_html_template(encrypted_data: &str, slides_json: &str, has_passphrase_fallback: bool) -> String {
    let fallback_link = if has_passphrase_fallback {
        r#"<button class="fallback-link" onclick="showPassphraseScreen()">I have the passphrase instead</button>"#
    } else {
        ""
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Honey Did - Legacy Document</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap" rel="stylesheet">
    <style>
{SHARED_CSS}
{QUESTION_CSS}
    </style>
</head>
<body>
    <div id="slideScreen" class="slide-screen">
        <div class="slide-container">
            <div id="slideText" class="slide-text"></div>
            <input type="text" id="slideInput" class="slide-input" style="display: none;" placeholder="Type your answer...">
            <button id="slideBtn" class="slide-btn" onclick="nextSlide()">Continue</button>
        </div>
        <div id="slideProgress" class="slide-progress"></div>
    </div>

    <div id="unlockingScreen" class="unlocking-screen hidden">
        <div class="unlocking-icon">&#128275;</div>
        <div class="unlocking-text">Unlocking...</div>
    </div>

    <div id="retryScreen" class="retry-screen hidden">
        <div class="retry-container">
            <h2 class="retry-title">Some answers weren't quite right.</h2>
            <p class="retry-subtitle">Please try again. <span id="attemptCounter"></span></p>
            <div id="retryQuestions" class="retry-questions"></div>
            <button class="retry-btn" onclick="retryUnlock()">Try Again</button>
            <p id="retryError" class="error-msg" style="display: none;"></p>
            {fallback_link}
        </div>
    </div>

    <div id="passphraseScreen" class="passphrase-screen hidden">
        <div class="passphrase-container">
            <div class="passphrase-icon">&#128274;</div>
            <h2 class="passphrase-title">Enter passphrase</h2>
            <input type="password" id="passphraseInput" class="passphrase-input" placeholder="Enter passphrase">
            <button class="retry-btn" onclick="unlockWithPassphrase()">Unlock</button>
            <p id="passphraseError" class="error-msg" style="display: none;"></p>
            <button class="back-link" onclick="showRetryScreen()">&#8592; Back to questions</button>
        </div>
    </div>

    <div id="content" class="content">
        <div class="container" id="documentContent"></div>
    </div>

    <script>
        const ENCRYPTED_DATA = {encrypted_data};
        const SLIDES = {slides_json};
        const HAS_PASSPHRASE = {has_passphrase};
{SHARED_JS_UTILS}
{SHARED_JS_SEARCH}
{SHARED_JS_RENDER_DOCUMENT}
{QUESTION_JS_DECRYPT}
{QUESTION_JS_SLIDES}
    </script>
</body>
</html>"##,
        SHARED_CSS = SHARED_CSS,
        QUESTION_CSS = QUESTION_CSS,
        fallback_link = fallback_link,
        encrypted_data = encrypted_data,
        slides_json = slides_json,
        has_passphrase = has_passphrase_fallback,
        SHARED_JS_UTILS = SHARED_JS_UTILS,
        SHARED_JS_SEARCH = SHARED_JS_SEARCH,
        SHARED_JS_RENDER_DOCUMENT = SHARED_JS_RENDER_DOCUMENT,
        QUESTION_JS_DECRYPT = QUESTION_JS_DECRYPT,
        QUESTION_JS_SLIDES = QUESTION_JS_SLIDES,
    )
}

// ============================================================================
// PRINT HTML GENERATION (for non-encrypted output)
// ============================================================================

/// Generates a printable HTML version of the document (not encrypted)
pub fn generate_print_html(document: &LegacyDocument) -> String {
    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Honey Did - Legacy Document</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; background: #fff; color: #333; max-width: 800px; margin: 0 auto; padding: 40px 20px; }
        h1 { font-size: 1.75rem; margin-bottom: 0.5rem; color: #283618; }
        h2 { font-size: 1.25rem; margin: 2rem 0 1rem 0; padding-bottom: 0.5rem; border-bottom: 2px solid #283618; color: #283618; }
        h3 { font-size: 1rem; margin: 1.5rem 0 0.75rem 0; color: #606C38; }
        .subtitle { color: #606C38; margin-bottom: 2rem; }
        .item { background: #f5f5f5; padding: 16px; border-radius: 8px; margin-bottom: 12px; }
        .item-title { font-weight: 600; margin-bottom: 0.5rem; }
        .item-detail { color: #666; font-size: 0.9rem; }
        .notes { background: #f9f9f9; padding: 12px; border-radius: 8px; margin-top: 1rem; font-style: italic; border-left: 3px solid #ccc; }
        @media print { body { padding: 0; } }
    </style>
</head>
<body>
    <h1>Honey Did - Legacy Document</h1>
"#);

    html.push_str(&format!("<p class=\"subtitle\">Prepared by {}</p>\n", document.meta.creator_name));

    // Financial
    if !document.financial.bank_accounts.is_empty() || !document.financial.credit_cards.is_empty()
        || !document.financial.investments.is_empty() || !document.financial.debts.is_empty()
        || !document.financial.notes.is_empty() {
        html.push_str("<h2>💰 Financial Information</h2>\n");

        if !document.financial.bank_accounts.is_empty() {
            html.push_str("<h3>Bank Accounts</h3>\n");
            for account in &document.financial.bank_accounts {
                html.push_str(&format!(
                    "<div class=\"item\"><div class=\"item-title\">{}</div><div class=\"item-detail\">Institution: {}</div><div class=\"item-detail\">Type: {}</div></div>\n",
                    account.name, account.institution, account.account_type
                ));
            }
        }

        if !document.financial.notes.is_empty() {
            html.push_str(&format!("<div class=\"notes\">{}</div>\n", document.financial.notes));
        }
    }

    // Insurance
    if !document.insurance.policies.is_empty() || !document.insurance.notes.is_empty() {
        html.push_str("<h2>🛡️ Insurance</h2>\n");
        for policy in &document.insurance.policies {
            html.push_str(&format!(
                "<div class=\"item\"><div class=\"item-title\">{}</div><div class=\"item-detail\">Provider: {}</div><div class=\"item-detail\">Policy #: {}</div></div>\n",
                policy.policy_type, policy.provider, policy.policy_number
            ));
        }
        if !document.insurance.notes.is_empty() {
            html.push_str(&format!("<div class=\"notes\">{}</div>\n", document.insurance.notes));
        }
    }

    // Bills
    if !document.bills.bills.is_empty() || !document.bills.notes.is_empty() {
        html.push_str("<h2>📄 Bills</h2>\n");
        for bill in &document.bills.bills {
            html.push_str(&format!(
                "<div class=\"item\"><div class=\"item-title\">{}</div><div class=\"item-detail\">Provider: {}</div><div class=\"item-detail\">Amount: {}</div><div class=\"item-detail\">Due Day: {}</div></div>\n",
                bill.name, bill.provider, bill.amount, bill.due_day
            ));
        }
        if !document.bills.notes.is_empty() {
            html.push_str(&format!("<div class=\"notes\">{}</div>\n", document.bills.notes));
        }
    }

    // Pets
    if !document.pets.pets.is_empty() || !document.pets.notes.is_empty() {
        html.push_str("<h2>🐾 Pets</h2>\n");
        for pet in &document.pets.pets {
            html.push_str(&format!(
                "<div class=\"item\"><div class=\"item-title\">{}</div><div class=\"item-detail\">Species: {}</div><div class=\"item-detail\">Breed: {}</div></div>\n",
                pet.name, pet.species, pet.breed
            ));
        }
        if !document.pets.notes.is_empty() {
            html.push_str(&format!("<div class=\"notes\">{}</div>\n", document.pets.notes));
        }
    }

    // Custom Sections
    for section in &document.custom_sections {
        html.push_str(&format!("<h2>📋 {}</h2>\n", section.name));
        for subsection in &section.subsections {
            html.push_str(&format!("<h3>{}</h3>\n", subsection.name));
            for item in &subsection.items {
                html.push_str("<div class=\"item\">");
                for field_def in &subsection.field_definitions {
                    if let Some(value) = item.values.get(&field_def.id) {
                        let display_value: String = match field_def.field_type {
                            FieldType::Boolean => if value == "true" { "Yes".to_string() } else { "No".to_string() },
                            _ => value.clone(),
                        };
                        html.push_str(&format!("<div class=\"item-detail\"><strong>{}:</strong> {}</div>", field_def.name, display_value));
                    }
                }
                html.push_str("</div>\n");
            }
        }
    }

    html.push_str("</body>\n</html>");
    html
}
