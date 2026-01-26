use crate::encryption::{decrypt_from_browser, encrypt_for_browser, EncryptedPayload, EncryptionError};
use crate::models::LegacyDocument;
use regex::Regex;

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
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; background: #F0EFEB; color: #283618; }}
        .lock-screen {{ display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; text-align: center; background: linear-gradient(145deg, #F0EFEB 0%, #D4D4D4 100%); }}
        .lock-icon {{ font-size: 4rem; margin-bottom: 1rem; }}
        .lock-title {{ font-size: 1.75rem; font-weight: 600; color: #283618; margin-bottom: 0.5rem; }}
        .lock-subtitle {{ color: #606C38; margin-bottom: 2rem; font-size: 0.95rem; }}
        .password-form {{ display: flex; flex-direction: column; gap: 1rem; width: 100%; max-width: 320px; }}
        .password-input {{ padding: 14px 16px; font-size: 1rem; border: 2px solid #D4D4D4; border-radius: 10px; text-align: center; background: white; transition: border-color 0.2s, box-shadow 0.2s; }}
        .password-input:focus {{ outline: none; border-color: #283618; box-shadow: 0 0 0 3px rgba(40, 54, 24, 0.1); }}
        .unlock-btn {{ padding: 14px 28px; font-size: 1rem; font-weight: 500; background: #283618; color: #F0EFEB; border: none; border-radius: 10px; cursor: pointer; transition: background 0.2s, transform 0.1s; }}
        .unlock-btn:hover {{ background: #1a2410; }}
        .unlock-btn:active {{ transform: scale(0.98); }}
        .error {{ color: #9B2C2C; margin-top: 1rem; font-size: 0.9rem; }}
        .content {{ display: none; width: 100%; }}
        .content.visible {{ display: block; width: 100%; }}
        .layout {{ display: flex; min-height: 100vh; width: 100%; }}
        .container {{ width: 100%; }}
        .sidebar {{ width: 280px; min-width: 280px; background: #FFFFFF; border-right: 1px solid #D4D4D4; height: 100vh; position: fixed; left: 0; top: 0; overflow-y: auto; display: flex; flex-direction: column; z-index: 100; }}
        .sidebar-header {{ padding: 24px 20px; border-bottom: 1px solid #D4D4D4; background: #283618; }}
        .sidebar-title {{ font-size: 1.25rem; font-weight: 600; color: #F0EFEB; margin-bottom: 0.25rem; }}
        .sidebar-subtitle {{ font-size: 0.8rem; color: #B7B7A4; }}
        .sidebar-search {{ padding: 16px; border-bottom: 1px solid #D4D4D4; }}
        .search-wrapper {{ position: relative; display: flex; align-items: center; }}
        .search-input {{ width: 100%; padding: 10px 36px 10px 14px; border: 1px solid #D4D4D4; border-radius: 8px; font-size: 0.9rem; background: #F0EFEB; transition: border-color 0.2s, box-shadow 0.2s; }}
        .search-input:focus {{ outline: none; border-color: #283618; box-shadow: 0 0 0 3px rgba(40, 54, 24, 0.1); background: white; }}
        .search-clear {{ position: absolute; right: 10px; background: none; border: none; cursor: pointer; color: #B7B7A4; font-size: 1.1rem; padding: 0 4px; line-height: 1; transition: color 0.2s; }}
        .search-clear:hover {{ color: #283618; }}
        .search-clear.hidden {{ display: none; }}
        .search-controls {{ padding: 12px 16px; border-bottom: 1px solid #D4D4D4; display: none; background: #F0EFEB; }}
        .search-controls.visible {{ display: block; }}
        .search-nav {{ display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.75rem; }}
        .search-nav button {{ padding: 6px 12px; border: 1px solid #D4D4D4; background: white; border-radius: 6px; cursor: pointer; font-size: 0.9rem; transition: all 0.2s; }}
        .search-nav button:hover:not(:disabled) {{ background: #F0EFEB; border-color: #B7B7A4; }}
        .search-nav button:disabled {{ opacity: 0.4; cursor: not-allowed; }}
        .search-counter {{ color: #606C38; font-size: 0.85rem; font-weight: 500; }}
        .search-filters {{ display: flex; gap: 6px; flex-wrap: wrap; }}
        .search-filter {{ padding: 4px 8px; border: 1px solid #D4D4D4; background: white; border-radius: 6px; font-size: 0.75rem; font-weight: 500; cursor: pointer; user-select: none; transition: all 0.2s; }}
        .search-filter.active {{ background: #283618; color: #F0EFEB; border-color: #283618; }}
        .search-filter.disabled {{ opacity: 0.4; cursor: not-allowed; }}
        .sidebar-nav {{ flex: 1; overflow-y: auto; padding: 16px; }}
        .nav-title {{ font-weight: 600; font-size: 0.7rem; text-transform: uppercase; color: #B7B7A4; margin-bottom: 0.75rem; letter-spacing: 0.05em; }}
        .nav-list {{ list-style: none; }}
        .nav-list li {{ margin: 2px 0; }}
        .nav-list a {{ color: #283618; text-decoration: none; display: block; padding: 8px 12px; border-radius: 6px; font-size: 0.9rem; font-weight: 500; transition: all 0.2s; }}
        .nav-list a:hover {{ background: #F0EFEB; color: #606C38; }}
        .sidebar-footer {{ padding: 16px; border-top: 1px solid #D4D4D4; }}
        .print-btn {{ width: 100%; padding: 12px 16px; background: #B7B7A4; color: #283618; border: none; border-radius: 8px; cursor: pointer; font-size: 0.9rem; font-weight: 500; transition: background 0.2s; }}
        .print-btn:hover {{ background: #a3a392; }}
        .main-content {{ flex: 1; margin-left: 280px; padding: 24px 40px; }}
        .section {{ background: white; padding: 24px; border-radius: 12px; margin-bottom: 20px; box-shadow: 0 1px 3px rgba(40,54,24,0.08), 0 1px 2px rgba(40,54,24,0.04); border: 1px solid #D4D4D4; }}
        .section-title {{ font-size: 1.15rem; font-weight: 600; color: #283618; border-bottom: 2px solid #283618; padding-bottom: 0.75rem; margin-bottom: 1.25rem; }}
        .item {{ background: #F0EFEB; padding: 16px; border-radius: 8px; margin-bottom: 12px; border: 1px solid #D4D4D4; }}
        .item-title {{ font-weight: 600; color: #283618; margin-bottom: 0.5rem; }}
        .item-detail {{ color: #606C38; font-size: 0.9rem; }}
        .notes {{ background: #F0EFEB; padding: 12px 14px; border-radius: 8px; margin-top: 1rem; font-style: italic; color: #283618; border-left: 3px solid #B7B7A4; }}
        .match-badge {{ font-size: 0.65rem; font-weight: 500; color: #606C38; background: #D4D4D4; padding: 2px 6px; border-radius: 4px; margin-left: 4px; vertical-align: middle; text-transform: lowercase; }}
        .highlight {{ background: #DDE5B6; padding: 1px 2px; border-radius: 2px; }}
        .highlight.current {{ background: #ADC178; outline: 2px solid #283618; }}
        .menu-toggle {{ display: none; position: fixed; top: 12px; left: 12px; z-index: 200; background: #283618; color: #F0EFEB; border: none; border-radius: 8px; padding: 10px 14px; cursor: pointer; font-weight: 500; font-size: 0.9rem; box-shadow: 0 2px 8px rgba(40,54,24,0.2); }}
        @media (max-width: 768px) {{
            .menu-toggle {{ display: block; }}
            .sidebar {{ transform: translateX(-100%); transition: transform 0.3s ease; }}
            .sidebar.open {{ transform: translateX(0); box-shadow: 4px 0 20px rgba(40,54,24,0.15); }}
            .main-content {{ margin-left: 0; padding: 70px 16px 20px 16px; }}
        }}
        @media print {{ .sidebar, .menu-toggle {{ display: none; }} .main-content {{ margin-left: 0; }} .section {{ break-inside: avoid; box-shadow: none; border: 1px solid #D4D4D4; }} }}
        /* Welcome Screen Styles */
        .welcome-screen {{ position: fixed; inset: 0; background: linear-gradient(145deg, #283618 0%, #1a2410 100%); display: flex; flex-direction: column; align-items: center; justify-content: center; z-index: 2000; opacity: 1; transition: opacity 0.5s ease; }}
        .welcome-screen.hidden {{ opacity: 0; pointer-events: none; }}
        .welcome-slide {{ max-width: 600px; padding: 40px; text-align: center; opacity: 0; transform: translateY(20px); transition: opacity 0.5s ease, transform 0.5s ease; }}
        .welcome-slide.visible {{ opacity: 1; transform: translateY(0); }}
        .welcome-slide-text {{ font-size: 1.5rem; line-height: 1.8; color: #F0EFEB; font-weight: 400; white-space: pre-wrap; }}
        .welcome-continue {{ margin-top: 40px; padding: 14px 32px; background: rgba(240, 239, 235, 0.15); color: #F0EFEB; border: 2px solid rgba(240, 239, 235, 0.3); border-radius: 10px; cursor: pointer; font-size: 1rem; font-weight: 500; transition: all 0.2s; }}
        .welcome-continue:hover {{ background: rgba(240, 239, 235, 0.25); border-color: rgba(240, 239, 235, 0.5); }}
        .welcome-progress {{ position: absolute; bottom: 40px; display: flex; gap: 8px; }}
        .welcome-dot {{ width: 8px; height: 8px; border-radius: 50%; background: rgba(240, 239, 235, 0.3); transition: background 0.3s; }}
        .welcome-dot.active {{ background: #F0EFEB; }}
        .welcome-timer {{ position: absolute; bottom: 20px; width: 200px; height: 3px; background: rgba(240, 239, 235, 0.2); border-radius: 2px; overflow: hidden; }}
        .welcome-timer-bar {{ height: 100%; background: #F0EFEB; width: 0%; transition: width linear; }}
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
        const PBKDF2_ITERATIONS = 600000;
        const WELCOME_SLIDES = {welcome_slides_json};

        // Welcome Screen State
        let welcomeCurrentSlide = 0;
        let welcomeTimerInterval = null;

        function initWelcomeScreen() {{
            if (WELCOME_SLIDES.length === 0) {{
                showLockScreen();
                return;
            }}

            const welcomeScreen = document.getElementById('welcomeScreen');
            const progressContainer = document.getElementById('welcomeProgress');

            // Create progress dots
            progressContainer.innerHTML = '';
            for (let i = 0; i < WELCOME_SLIDES.length; i++) {{
                const dot = document.createElement('div');
                dot.className = 'welcome-dot' + (i === 0 ? ' active' : '');
                progressContainer.appendChild(dot);
            }}

            welcomeScreen.classList.remove('hidden');
            document.getElementById('lockScreen').style.display = 'none';
            showWelcomeSlide(0);
        }}

        function showWelcomeSlide(index) {{
            if (index >= WELCOME_SLIDES.length) {{
                endWelcomeScreen();
                return;
            }}

            welcomeCurrentSlide = index;
            const slide = WELCOME_SLIDES[index];
            const slideEl = document.getElementById('welcomeSlide');
            const textEl = document.getElementById('welcomeText');
            const continueBtn = document.getElementById('welcomeContinue');
            const timerEl = document.getElementById('welcomeTimer');
            const timerBar = document.getElementById('welcomeTimerBar');

            // Update progress dots
            const dots = document.querySelectorAll('.welcome-dot');
            dots.forEach((dot, i) => {{
                dot.classList.toggle('active', i === index);
            }});

            // Clear any existing timer
            if (welcomeTimerInterval) {{
                clearInterval(welcomeTimerInterval);
                welcomeTimerInterval = null;
            }}

            // Hide slide for transition
            slideEl.classList.remove('visible');

            setTimeout(() => {{
                textEl.textContent = slide.text;

                // Handle transition type
                if (slide.transition.type === 'click') {{
                    continueBtn.style.display = 'block';
                    timerEl.style.display = 'none';
                }} else if (slide.transition.type === 'auto') {{
                    continueBtn.style.display = 'none';
                    timerEl.style.display = 'block';
                    timerBar.style.width = '0%';
                    timerBar.style.transition = 'none';

                    // Force reflow
                    timerBar.offsetHeight;

                    const duration = slide.transition.seconds * 1000;
                    timerBar.style.transition = 'width ' + duration + 'ms linear';
                    timerBar.style.width = '100%';

                    welcomeTimerInterval = setTimeout(() => {{
                        nextWelcomeSlide();
                    }}, duration);
                }}

                // Show slide with transition
                slideEl.classList.add('visible');
            }}, 100);
        }}

        function nextWelcomeSlide() {{
            if (welcomeTimerInterval) {{
                clearTimeout(welcomeTimerInterval);
                welcomeTimerInterval = null;
            }}
            showWelcomeSlide(welcomeCurrentSlide + 1);
        }}

        function endWelcomeScreen() {{
            const welcomeScreen = document.getElementById('welcomeScreen');
            welcomeScreen.classList.add('hidden');

            setTimeout(() => {{
                welcomeScreen.style.display = 'none';
                showLockScreen();
            }}, 500);
        }}

        function showLockScreen() {{
            document.getElementById('lockScreen').style.display = 'flex';
            document.getElementById('passphrase').focus();
        }}

        // Initialize on page load
        document.addEventListener('DOMContentLoaded', initWelcomeScreen);

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

            // Mobile menu toggle button
            html += '<button class="menu-toggle" onclick="toggleSidebar()">☰ Menu</button>';

            // Layout wrapper
            html += '<div class="layout">';

            // Sidebar
            html += '<div class="sidebar" id="sidebar">';

            // Sidebar header
            html += '<div class="sidebar-header">';
            html += '<div class="sidebar-title">Honey Did</div>';
            if (data.meta && data.meta.creator_name) {{
                html += '<div class="sidebar-subtitle">By ' + escapeHtml(data.meta.creator_name) + '</div>';
            }}
            html += '</div>';

            // Search section
            html += '<div class="sidebar-search">';
            html += '<div class="search-wrapper">';
            html += '<input type="text" id="searchInput" class="search-input" placeholder="Search..." oninput="debounceSearch(this.value)" onkeydown="if(event.key===\'Escape\')clearSearch()">';
            html += '<button class="search-clear hidden" id="searchClear" onclick="clearSearch()" title="Clear search (Esc)">✕</button>';
            html += '</div>';
            html += '</div>';

            // Search controls
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

            // Navigation
            html += '<div class="sidebar-nav">';
            html += '<div class="nav-title">Contents</div>';
            html += '<ul class="nav-list">';
            const sections = ['financial', 'insurance', 'bills', 'property', 'legal', 'digital', 'household', 'personal', 'contacts', 'medical', 'pets'];
            const sectionLabels = {{'financial': '💰 Financial', 'insurance': '🛡️ Insurance', 'bills': '📄 Bills', 'property': '🏠 Property', 'legal': '⚖️ Legal', 'digital': '💻 Digital Life', 'household': '🔧 Household', 'personal': '👤 Personal', 'contacts': '📇 Contacts', 'medical': '🏥 Medical', 'pets': '🐾 Pets'}};
            sections.forEach(s => {{
                html += '<li><a href="#' + s + '" onclick="closeSidebarOnMobile()">' + sectionLabels[s] + '</a></li>';
            }});
            html += '</ul>';
            html += '</div>';

            // Sidebar footer with print button
            html += '<div class="sidebar-footer">';
            html += '<button class="print-btn" onclick="window.print()">Print Document</button>';
            html += '</div>';

            html += '</div>'; // End sidebar

            // Main content area
            html += '<div class="main-content" id="mainContent">';

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
                html += renderSection('💰 Financial Information', 'financial', content);
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
                html += renderSection('🛡️ Insurance', 'insurance', content);
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
                html += renderSection('📄 Bills', 'bills', content);
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
                html += renderSection('🏠 Property', 'property', content);
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
                html += renderSection('⚖️ Legal Documents', 'legal', content);
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
                html += renderSection('💻 Digital Life', 'digital', content);
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
                html += renderSection('🔧 Household', 'household', content);
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
                html += renderSection('👤 Personal Wishes', 'personal', content);
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
                html += renderSection('📇 Important Contacts', 'contacts', content);
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
                html += renderSection('🏥 Medical Information', 'medical', content);
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
                html += renderSection('🐾 Pets', 'pets', content);
            }}

            html += '</div>'; // End main-content
            html += '</div>'; // End layout

            container.innerHTML = html;
            document.getElementById('lockScreen').style.display = 'none';
            document.getElementById('content').classList.add('visible');
            buildSearchIndex();
        }}

        function toggleSidebar() {{
            document.getElementById('sidebar').classList.toggle('open');
        }}

        function closeSidebarOnMobile() {{
            if (window.innerWidth <= 768) {{
                document.getElementById('sidebar').classList.remove('open');
            }}
        }}

        function levenshtein(a, b) {{
            if (a.length === 0) return b.length;
            if (b.length === 0) return a.length;
            const matrix = [];
            for (let i = 0; i <= b.length; i++) matrix[i] = [i];
            for (let j = 0; j <= a.length; j++) matrix[0][j] = j;
            for (let i = 1; i <= b.length; i++) {{
                for (let j = 1; j <= a.length; j++) {{
                    if (b.charAt(i-1) === a.charAt(j-1)) {{
                        matrix[i][j] = matrix[i-1][j-1];
                    }} else {{
                        matrix[i][j] = Math.min(
                            matrix[i-1][j-1] + 1,
                            matrix[i][j-1] + 1,
                            matrix[i-1][j] + 1
                        );
                    }}
                }}
            }}
            return matrix[b.length][a.length];
        }}

        function metaphone(word) {{
            if (!word || word.length < 2) return '';
            word = word.toUpperCase().replace(/[^A-Z]/g, '');
            if (!word) return '';

            const start = word.slice(0, 2);
            if (['KN', 'GN', 'PN', 'AE', 'WR'].includes(start)) word = word.slice(1);
            if (word[0] === 'X') word = 'S' + word.slice(1);
            if (start === 'WH') word = 'W' + word.slice(2);

            let result = '';
            let i = 0;
            const len = word.length;

            while (i < len && result.length < 6) {{
                const c = word[i];
                const next = word[i + 1] || '';
                const prev = word[i - 1] || '';

                if ('AEIOU'.includes(c)) {{
                    if (i === 0) result += c;
                }} else if (c === 'B') {{
                    if (!(i === len - 1 && prev === 'M')) result += 'B';
                }} else if (c === 'C') {{
                    if (next === 'H') {{ result += 'X'; i++; }}
                    else if ('IEY'.includes(next)) result += 'S';
                    else result += 'K';
                }} else if (c === 'D') {{
                    if (next === 'G' && 'IEY'.includes(word[i + 2] || '')) {{ result += 'J'; i++; }}
                    else result += 'T';
                }} else if (c === 'G') {{
                    if (next === 'H') {{ if (!'AEIOU'.includes(word[i + 2] || '')) i++; }}
                    else if (next === 'N' && word[i + 2] === 'E' && word[i + 3] === 'D') {{}}
                    else if ('IEY'.includes(next)) result += 'J';
                    else result += 'K';
                }} else if (c === 'H') {{
                    if ('AEIOU'.includes(next) && !'CSPTG'.includes(prev)) result += 'H';
                }} else if (c === 'K') {{
                    if (prev !== 'C') result += 'K';
                }} else if (c === 'P') {{
                    result += (next === 'H') ? (i++, 'F') : 'P';
                }} else if (c === 'Q') {{
                    result += 'K';
                }} else if (c === 'S') {{
                    if (next === 'H') {{ result += 'X'; i++; }}
                    else if (next === 'I' && 'OA'.includes(word[i + 2] || '')) {{ result += 'X'; i++; }}
                    else result += 'S';
                }} else if (c === 'T') {{
                    if (next === 'H') {{ result += '0'; i++; }}
                    else if (next === 'I' && 'OA'.includes(word[i + 2] || '')) {{ result += 'X'; i++; }}
                    else result += 'T';
                }} else if (c === 'V') {{
                    result += 'F';
                }} else if (c === 'W' || c === 'Y') {{
                    if ('AEIOU'.includes(next)) result += c;
                }} else if (c === 'X') {{
                    result += 'KS';
                }} else if (c === 'Z') {{
                    result += 'S';
                }} else if ('FJLMNR'.includes(c)) {{
                    result += c;
                }}
                i++;
            }}
            return result;
        }}

        let searchIndex = [];
        let searchState = {{
            term: '',
            matches: [],
            filters: {{ exact: true, contains: true, spelling: true, phonetic: true }},
            currentIndex: -1
        }};

        function buildSearchIndex() {{
            searchIndex = [];
            const content = document.getElementById('mainContent');
            if (!content) return;
            const walker = document.createTreeWalker(content, NodeFilter.SHOW_TEXT, {{
                acceptNode: (node) => {{
                    return node.textContent.trim() ? NodeFilter.FILTER_ACCEPT : NodeFilter.FILTER_REJECT;
                }}
            }});

            let node;
            while (node = walker.nextNode()) {{
                const text = node.textContent;
                const words = text.match(/\b[\w']+\b/g) || [];
                words.forEach(word => {{
                    if (word.length >= 2) {{
                        searchIndex.push({{
                            text: word,
                            lowerText: word.toLowerCase(),
                            metaphone: metaphone(word),
                            node: node,
                            fullText: text
                        }});
                    }}
                }});
            }}
        }}

        let searchTimeout;
        function debounceSearch(term) {{
            clearTimeout(searchTimeout);
            const clearBtn = document.getElementById('searchClear');
            if (term) {{
                clearBtn.classList.remove('hidden');
            }} else {{
                clearBtn.classList.add('hidden');
            }}
            searchTimeout = setTimeout(() => performSearch(term), 300);
        }}

        function clearSearch() {{
            const input = document.getElementById('searchInput');
            input.value = '';
            document.getElementById('searchClear').classList.add('hidden');
            clearTimeout(searchTimeout);
            clearHighlights();
            document.getElementById('searchControls').classList.remove('visible');
            searchState.term = '';
            searchState.matches = [];
            searchState.currentIndex = -1;
        }}

        function performSearch(term) {{
            clearHighlights();
            buildSearchIndex();
            searchState.term = term.toLowerCase();
            searchState.matches = [];
            searchState.currentIndex = -1;
            searchState.filters = {{ exact: true, contains: true, spelling: true, phonetic: true }};

            if (!term || term.length < 2) {{
                document.getElementById('searchControls').classList.remove('visible');
                updateSearchUI();
                return;
            }}

            document.getElementById('searchControls').classList.add('visible');
            const termMeta = metaphone(term);
            const matchMap = new Map();

            searchIndex.forEach(entry => {{
                const key = entry.node.textContent + '|' + entry.text;
                if (matchMap.has(key)) return;

                const wordLower = entry.lowerText;
                const termLower = term.toLowerCase();

                if (wordLower === termLower) {{
                    matchMap.set(key, {{ ...entry, type: 'exact' }});
                    return;
                }}

                if (term.length >= 3 && wordLower.includes(termLower)) {{
                    matchMap.set(key, {{ ...entry, type: 'contains' }});
                    return;
                }}

                const maxDist = term.length >= 8 ? 3 : 2;
                const dist = levenshtein(wordLower, termLower);
                if (dist > 0 && dist <= maxDist) {{
                    matchMap.set(key, {{ ...entry, type: 'spelling', distance: dist }});
                    return;
                }}

                if (term.length >= 3 && termMeta && entry.metaphone === termMeta) {{
                    matchMap.set(key, {{ ...entry, type: 'phonetic' }});
                }}
            }});

            const typeOrder = {{ exact: 0, contains: 1, spelling: 2, phonetic: 3 }};
            searchState.matches = Array.from(matchMap.values())
                .sort((a, b) => typeOrder[a.type] - typeOrder[b.type]);

            highlightMatches();
            updateSearchUI();

            if (getVisibleMatches().length > 0) {{
                searchState.currentIndex = 0;
                scrollToCurrentMatch();
            }}
        }}

        function clearHighlights() {{
            document.querySelectorAll('mark.highlight').forEach(mark => {{
                const parent = mark.parentNode;
                const badge = mark.nextSibling;
                if (badge && badge.classList && badge.classList.contains('match-badge')) {{
                    badge.remove();
                }}
                parent.replaceChild(document.createTextNode(mark.textContent), mark);
                parent.normalize();
            }});
        }}

        function highlightMatches() {{
            const visible = getVisibleMatches();

            // Group matches by their text node to handle multiple matches in same node
            const nodeGroups = new Map();
            visible.forEach((match, idx) => {{
                if (!nodeGroups.has(match.node)) {{
                    nodeGroups.set(match.node, []);
                }}
                nodeGroups.get(match.node).push({{ ...match, visibleIdx: idx }});
            }});

            // Process each node once
            for (const [node, matches] of nodeGroups) {{
                if (!node.parentNode) continue;

                const text = node.textContent;

                // Find positions of all matches in this node
                const highlights = [];
                for (const m of matches) {{
                    // Use word boundaries to match whole words only
                    const regex = new RegExp('\\b' + m.text.replace(/[.*+?^${{}}()|[\]\\]/g, '\\$&') + '\\b', 'i');
                    const result = regex.exec(text);
                    if (result) {{
                        highlights.push({{
                            start: result.index,
                            end: result.index + result[0].length,
                            matchedText: result[0],
                            type: m.type,
                            idx: m.visibleIdx
                        }});
                    }}
                }}

                if (highlights.length === 0) continue;

                // Sort by start position
                highlights.sort((a, b) => a.start - b.start);

                // Remove overlapping highlights (keep earlier one)
                const nonOverlapping = [];
                let lastEnd = 0;
                for (const h of highlights) {{
                    if (h.start >= lastEnd) {{
                        nonOverlapping.push(h);
                        lastEnd = h.end;
                    }}
                }}

                // Build fragment with all highlights
                const fragment = document.createDocumentFragment();
                let pos = 0;

                for (const h of nonOverlapping) {{
                    if (h.start > pos) {{
                        fragment.appendChild(document.createTextNode(text.slice(pos, h.start)));
                    }}

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
                }}

                if (pos < text.length) {{
                    fragment.appendChild(document.createTextNode(text.slice(pos)));
                }}

                node.parentNode.replaceChild(fragment, node);
            }}
        }}

        function getVisibleMatches() {{
            return searchState.matches.filter(m => searchState.filters[m.type]);
        }}

function updateSearchUI() {{
    const counts = {{ exact: 0, contains: 0, spelling: 0, phonetic: 0 }};
    searchState.matches.forEach(m => counts[m.type]++);

    document.getElementById('exactCount').textContent = counts.exact;
    document.getElementById('containsCount').textContent = counts.contains;
    document.getElementById('spellingCount').textContent = counts.spelling;
    document.getElementById('phoneticCount').textContent = counts.phonetic;

    ['exact', 'contains', 'spelling', 'phonetic'].forEach(type => {{
        const el = document.querySelector(`.search-filter[data-type="${{type}}"]`);
        if (counts[type] === 0) {{
            el.classList.add('disabled');
            el.classList.remove('active');
        }} else {{
            el.classList.remove('disabled');
            el.classList.toggle('active', searchState.filters[type]);
        }}
    }});

    const visible = getVisibleMatches();
    const counter = document.getElementById('searchCounter');
    const prevBtn = document.getElementById('prevBtn');
    const nextBtn = document.getElementById('nextBtn');

    if (visible.length === 0) {{
        counter.textContent = 'No matches';
        prevBtn.disabled = true;
        nextBtn.disabled = true;
    }} else {{
        const current = searchState.currentIndex + 1;
        counter.textContent = `Match ${{current}} of ${{visible.length}}`;
        prevBtn.disabled = visible.length <= 1;
        nextBtn.disabled = visible.length <= 1;
    }}

    updateCurrentHighlight();
}}

function updateCurrentHighlight() {{
    document.querySelectorAll('mark.highlight.current').forEach(el => el.classList.remove('current'));
    const visible = getVisibleMatches();
    if (searchState.currentIndex >= 0 && searchState.currentIndex < visible.length) {{
        const mark = document.querySelector(`mark.highlight[data-match-index="${{searchState.currentIndex}}"]`);
        if (mark) mark.classList.add('current');
    }}
}}

function scrollToCurrentMatch() {{
    updateCurrentHighlight();
    const mark = document.querySelector('mark.highlight.current');
    if (mark) {{
        mark.scrollIntoView({{ behavior: 'smooth', block: 'center' }});
    }}
    updateSearchUI();
}}

function nextMatch() {{
    const visible = getVisibleMatches();
    if (visible.length === 0) return;
    searchState.currentIndex = (searchState.currentIndex + 1) % visible.length;
    scrollToCurrentMatch();
}}

function prevMatch() {{
    const visible = getVisibleMatches();
    if (visible.length === 0) return;
    searchState.currentIndex = (searchState.currentIndex - 1 + visible.length) % visible.length;
    scrollToCurrentMatch();
}}

function toggleFilter(el) {{
    const type = el.dataset.type;
    const counts = {{ exact: 0, contains: 0, spelling: 0, phonetic: 0 }};
    searchState.matches.forEach(m => counts[m.type]++);

    if (counts[type] === 0) return;

    searchState.filters[type] = !searchState.filters[type];
    el.classList.toggle('active', searchState.filters[type]);

    // Clear and rebuild index since nodes were replaced
    clearHighlights();
    buildSearchIndex();

    // Re-run matching with current term
    const term = searchState.term;
    const termMeta = metaphone(term);
    const matchMap = new Map();

    searchIndex.forEach(entry => {{
        const key = entry.node.textContent + '|' + entry.text;
        if (matchMap.has(key)) return;

        const wordLower = entry.lowerText;
        const termLower = term.toLowerCase();

        if (wordLower === termLower) {{
            matchMap.set(key, {{ ...entry, type: 'exact' }});
            return;
        }}

        if (term.length >= 3 && wordLower.includes(termLower)) {{
            matchMap.set(key, {{ ...entry, type: 'contains' }});
            return;
        }}

        const maxDist = term.length >= 8 ? 3 : 2;
        const dist = levenshtein(wordLower, termLower);
        if (dist > 0 && dist <= maxDist) {{
            matchMap.set(key, {{ ...entry, type: 'spelling', distance: dist }});
            return;
        }}

        if (term.length >= 3 && termMeta && entry.metaphone === termMeta) {{
            matchMap.set(key, {{ ...entry, type: 'phonetic' }});
        }}
    }});

    const typeOrder = {{ exact: 0, contains: 1, spelling: 2, phonetic: 3 }};
    searchState.matches = Array.from(matchMap.values())
        .sort((a, b) => typeOrder[a.type] - typeOrder[b.type]);

    highlightMatches();

    const visible = getVisibleMatches();
    if (visible.length > 0) {{
        searchState.currentIndex = Math.min(searchState.currentIndex, visible.length - 1);
        if (searchState.currentIndex < 0) searchState.currentIndex = 0;
    }} else {{
        searchState.currentIndex = -1;
    }}

    updateSearchUI();
    if (searchState.currentIndex >= 0) scrollToCurrentMatch();
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
            welcome_screen: None,
        }
    }

    #[test]
    fn test_export_import_roundtrip() {
        let original = create_test_document();
        let passphrase = "test-passphrase-123";

        let html = generate_encrypted_html(&original, passphrase, false)
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
        let html = generate_encrypted_html(&doc, "correct-passphrase", false)
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
