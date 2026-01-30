mod encryption;
mod export;
mod models;
mod storage;

use models::LegacyDocument;
use std::sync::Mutex;
use tauri::{Manager, State};

#[cfg(target_os = "android")]
use tauri::plugin::PluginHandle;

struct AppState {
    document: Mutex<LegacyDocument>,
}

// Input validation constants
const MAX_PASSPHRASE_LENGTH: usize = 1024;
const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 256;
const MAX_HTML_CONTENT_LENGTH: usize = 50 * 1024 * 1024; // 50MB

/// Validates passphrase input
fn validate_passphrase(passphrase: &str) -> Result<(), String> {
    if passphrase.is_empty() {
        return Err("Passphrase cannot be empty".to_string());
    }
    if passphrase.len() > MAX_PASSPHRASE_LENGTH {
        return Err("Passphrase is too long".to_string());
    }
    Ok(())
}

/// Validates password input
fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(format!("Password must be at least {} characters", MIN_PASSWORD_LENGTH));
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err("Password is too long".to_string());
    }
    Ok(())
}

/// Validates HTML content for import
fn validate_html_content(html: &str) -> Result<(), String> {
    if html.is_empty() {
        return Err("File content cannot be empty".to_string());
    }
    if html.len() > MAX_HTML_CONTENT_LENGTH {
        return Err("File is too large".to_string());
    }
    Ok(())
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
fn export_html(state: State<AppState>, passphrase: String, include_welcome_screen: Option<bool>) -> Result<String, String> {
    validate_passphrase(&passphrase)?;
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    export::generate_encrypted_html(&doc, &passphrase, include_welcome_screen.unwrap_or(false)).map_err(|e: export::ExportError| e.to_string())
}

#[tauri::command]
fn export_html_with_questions(state: State<AppState>, passphrase: String, include_welcome_screen: bool) -> Result<String, String> {
    validate_passphrase(&passphrase)?;
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    export::generate_encrypted_html_with_questions(&doc, &passphrase, include_welcome_screen).map_err(|e: export::ExportError| e.to_string())
}

#[tauri::command]
fn save_html_to_downloads(html: String, file_name: String) -> Result<String, String> {
    let dir = get_download_dir()?;

    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    let path = dir.join(&file_name);
    std::fs::write(&path, html).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg(target_os = "android")]
fn get_download_dir() -> Result<std::path::PathBuf, String> {
    // On Android, write to the app's cache directory. The file is temporary —
    // the share intent (ACTION_SEND) is what delivers it to the user's chosen destination.
    if let Ok(tmpdir) = std::env::var("TMPDIR") {
        let export_dir = std::path::PathBuf::from(tmpdir).join("exports");
        return Ok(export_dir);
    }

    // Fall back to app-private storage
    storage::get_data_dir()
        .map(|d| d.join("exports"))
        .map_err(|e| format!("Cannot find a writable directory: {}", e))
}

#[cfg(not(target_os = "android"))]
fn get_download_dir() -> Result<std::path::PathBuf, String> {
    directories::UserDirs::new()
        .and_then(|u| u.download_dir().map(|d| d.to_path_buf()))
        .or_else(|| std::env::var("HOME").ok().map(std::path::PathBuf::from))
        .ok_or_else(|| "Cannot find a writable directory".to_string())
}

#[tauri::command]
fn save_export(state: State<AppState>, passphrase: String, file_path: String, include_welcome_screen: Option<bool>) -> Result<(), String> {
    validate_passphrase(&passphrase)?;
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    let html = export::generate_encrypted_html(&doc, &passphrase, include_welcome_screen.unwrap_or(false)).map_err(|e: export::ExportError| e.to_string())?;
    std::fs::write(&file_path, html).map_err(|_| "Failed to save file".to_string())
}

#[tauri::command]
async fn save_export_with_dialog(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    passphrase: String,
    include_welcome_screen: bool,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    validate_passphrase(&passphrase)?;
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    let html = export::generate_encrypted_html(&doc, &passphrase, include_welcome_screen).map_err(|e: export::ExportError| e.to_string())?;
    drop(doc);

    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let default_name = format!("honey-did-{}.html", date);

    let file_path = app.dialog()
        .file()
        .set_file_name(&default_name)
        .add_filter("HTML Files", &["html", "htm"])
        .blocking_save_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            std::fs::write(&path_str, &html)
                .map_err(|e| format!("Failed to save file: {}", e))?;
            Ok(Some(path_str))
        }
        None => Ok(None),
    }
}

#[tauri::command]
async fn save_export_with_questions(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    passphrase: String,
    include_welcome_screen: bool,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    validate_passphrase(&passphrase)?;
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    let html = export::generate_encrypted_html_with_questions(&doc, &passphrase, include_welcome_screen).map_err(|e: export::ExportError| e.to_string())?;
    drop(doc);

    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let default_name = format!("honey-did-{}.html", date);

    let file_path = app.dialog()
        .file()
        .set_file_name(&default_name)
        .add_filter("HTML Files", &["html", "htm"])
        .blocking_save_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_string();
            std::fs::write(&path_str, &html)
                .map_err(|e| format!("Failed to save file: {}", e))?;
            Ok(Some(path_str))
        }
        None => Ok(None),
    }
}

#[tauri::command]
fn get_print_html(state: State<AppState>) -> Result<String, String> {
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    Ok(export::generate_print_html(&doc))
}

#[tauri::command]
fn import_file(encrypted_html: String, passphrase: String) -> Result<LegacyDocument, String> {
    validate_html_content(&encrypted_html)?;
    validate_passphrase(&passphrase)?;
    export::import_from_html(&encrypted_html, &passphrase).map_err(|e: export::ExportError| e.to_string())
}

#[tauri::command]
fn merge_document(state: State<AppState>, imported: LegacyDocument) -> Result<(), String> {
    let mut doc = state.document.lock().map_err(|e| e.to_string())?;
    *doc = imported;
    storage::save_document(&doc).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn generate_passphrase() -> String {
    use rand::seq::SliceRandom;
    use rand::rngs::OsRng;

    // Extended word list (256 words) for better entropy
    // 6 words from 256 = log2(256^6) = 48 bits of entropy
    let words: Vec<&str> = vec![
        "apple", "arrow", "anchor", "autumn", "azure", "baker", "banana", "beacon",
        "berry", "blade", "blanket", "blaze", "bloom", "bottle", "branch", "breeze",
        "bridge", "bronze", "brush", "bucket", "butter", "cabin", "cactus", "candle",
        "canvas", "canyon", "carpet", "carrot", "castle", "cedar", "chain", "chair",
        "cherry", "cider", "circle", "cliff", "cloud", "clover", "coast", "cobalt",
        "coffee", "comet", "coral", "cotton", "crayon", "creek", "cricket", "crown",
        "crystal", "curtain", "dagger", "daisy", "dancer", "delta", "desert", "diamond",
        "dolphin", "dragon", "dream", "drift", "drum", "eagle", "earth", "echo",
        "elder", "ember", "engine", "fabric", "falcon", "feather", "fern", "field",
        "fire", "flame", "flint", "flower", "forest", "forge", "fossil", "fountain",
        "frost", "galaxy", "garden", "garnet", "geyser", "glacier", "globe", "golden",
        "grain", "grape", "grass", "gravel", "grove", "hammer", "harbor", "harvest",
        "hawk", "hazel", "heart", "heather", "helmet", "honey", "horizon", "hunter",
        "ice", "indigo", "iris", "iron", "island", "ivory", "jade", "jasper",
        "jet", "journal", "jungle", "kelp", "kernel", "kettle", "kingdom", "kitchen",
        "kite", "lake", "lantern", "lapis", "lark", "lava", "leaf", "lemon",
        "library", "light", "lily", "linen", "lion", "lotus", "lunar", "magnet",
        "maple", "marble", "market", "meadow", "melon", "metal", "mirror", "mist",
        "moon", "moss", "mountain", "mushroom", "nectar", "needle", "night", "north",
        "oak", "oasis", "ocean", "olive", "onyx", "orange", "orchid", "osprey",
        "otter", "palm", "panther", "paper", "parrot", "path", "pebble", "pepper",
        "phoenix", "piano", "pillow", "pine", "planet", "plum", "pond", "poplar",
        "prism", "pumpkin", "quartz", "queen", "quiet", "rabbit", "radish", "rain",
        "rainbow", "raven", "reef", "ridge", "river", "robin", "rocket", "rose",
        "ruby", "sage", "salmon", "sand", "sapphire", "scarlet", "scroll", "shadow",
        "shell", "shore", "silver", "sky", "slate", "snow", "spark", "spirit",
        "spruce", "star", "stone", "storm", "stream", "summer", "summit", "sunset",
        "swift", "temple", "thistle", "thunder", "tiger", "timber", "torch", "trail",
        "treasure", "tree", "trout", "tulip", "turtle", "twilight", "umbrella", "valley",
        "velvet", "violet", "volcano", "wave", "wheat", "willow", "wind", "winter",
        "wolf", "wonder", "woods", "yarn", "yellow", "zebra", "zenith", "zephyr",
        "zinc", "amber", "aspen", "atlas", "basil", "birch", "brook", "canyon",
    ];

    let mut rng = OsRng;
    let selected: Vec<&str> = words.choose_multiple(&mut rng, 6).cloned().collect();
    selected.join("-")
}

#[tauri::command]
fn set_app_password(password: String) -> Result<(), String> {
    validate_password(&password)?;
    let hash = storage::hash_password(&password).map_err(|e| e.to_string())?;
    storage::save_password_hash(&hash).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn verify_app_password(password: String) -> Result<bool, String> {
    // Don't validate length on verification - user may have old password
    if password.is_empty() || password.len() > MAX_PASSWORD_LENGTH {
        return Err("Invalid password".to_string());
    }
    let hash = storage::load_password_hash()
        .map_err(|e| e.to_string())?
        .ok_or("No password set")?;
    storage::verify_password(&password, &hash).map_err(|e| e.to_string())
}

#[tauri::command]
fn has_app_password() -> Result<bool, String> {
    let hash = storage::load_password_hash().map_err(|e| e.to_string())?;
    Ok(hash.is_some())
}

#[tauri::command]
fn change_app_password(old_password: String, new_password: String) -> Result<(), String> {
    // Validate new password
    validate_password(&new_password)?;

    // Verify old password first
    let hash = storage::load_password_hash()
        .map_err(|e| e.to_string())?
        .ok_or("No password set")?;
    let valid = storage::verify_password(&old_password, &hash).map_err(|e| e.to_string())?;
    if !valid {
        return Err("Incorrect password".to_string());
    }
    // Set new password
    let new_hash = storage::hash_password(&new_password).map_err(|e| e.to_string())?;
    storage::save_password_hash(&new_hash).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn clear_all_data(state: State<AppState>, password: String) -> Result<(), String> {
    // Check if password exists
    let hash = storage::load_password_hash().map_err(|e| e.to_string())?;

    // If password exists, verify it
    if let Some(h) = hash {
        let valid = storage::verify_password(&password, &h).map_err(|e| e.to_string())?;
        if !valid {
            return Err("Incorrect password".to_string());
        }
    }
    // If no password set, allow clearing without verification

    // Clear everything
    storage::delete_document().map_err(|e| e.to_string())?;
    storage::delete_password_hash().map_err(|e| e.to_string())?;
    storage::delete_settings().map_err(|e| e.to_string())?;
    // Reset in-memory state
    let mut doc = state.document.lock().map_err(|e| e.to_string())?;
    *doc = LegacyDocument::default();
    Ok(())
}

#[tauri::command]
fn force_clear_all_data(state: State<AppState>, confirmation: String) -> Result<(), String> {
    // Require exact confirmation phrase (case-insensitive)
    if confirmation.to_uppercase() != "DELETE ALL DATA" {
        return Err("Please type DELETE ALL DATA to confirm".to_string());
    }

    // Clear everything without password verification
    storage::delete_document().map_err(|e| e.to_string())?;
    storage::delete_password_hash().map_err(|e| e.to_string())?;
    storage::delete_settings().map_err(|e| e.to_string())?;
    // Reset in-memory state
    let mut doc = state.document.lock().map_err(|e| e.to_string())?;
    *doc = LegacyDocument::default();
    Ok(())
}

#[tauri::command]
fn get_clear_on_exit() -> Result<bool, String> {
    storage::load_settings().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_clear_on_exit(enabled: bool) -> Result<(), String> {
    storage::save_settings(enabled).map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_data_on_exit(state: State<AppState>) -> Result<(), String> {
    storage::delete_document().map_err(|e| e.to_string())?;
    storage::delete_password_hash().map_err(|e| e.to_string())?;
    storage::delete_settings().map_err(|e| e.to_string())?;
    let mut doc = state.document.lock().map_err(|e| e.to_string())?;
    *doc = LegacyDocument::default();
    Ok(())
}

#[tauri::command]
fn open_external_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(&url, None::<&str>)
        .map_err(|e| format!("Failed to open URL: {}", e))
}

#[cfg(target_os = "android")]
fn init_share_plugin<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("share")
        .setup(|app, _api| {
            let handle = _api.register_android_plugin("com.honeydid.app", "SharePlugin")
                .map_err(|e| format!("Failed to register SharePlugin: {}", e))?;
            app.manage(handle);
            Ok(())
        })
        .build()
}

#[cfg(not(target_os = "android"))]
fn init_share_plugin<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("share").build()
}

#[tauri::command]
#[cfg(target_os = "android")]
fn share_file(app: tauri::AppHandle, file_path: String, mime_type: String) -> Result<(), String> {
    let handle: State<PluginHandle<tauri::Wry>> = app.state();
    let mut args = serde_json::Map::new();
    args.insert("filePath".into(), serde_json::Value::String(file_path));
    args.insert("mimeType".into(), serde_json::Value::String(mime_type));
    handle.run_mobile_plugin::<()>("shareFile", serde_json::Value::Object(args))
        .map_err(|e| format!("Share failed: {}", e))
}

#[tauri::command]
#[cfg(not(target_os = "android"))]
fn share_file(_file_path: String, _mime_type: String) -> Result<(), String> {
    Err("Share is only available on Android".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(init_share_plugin())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            document: Mutex::new(LegacyDocument::default()),
        })
        .setup(|app| {
            // Load document after Tauri runtime is initialized (needed for Android)
            let loaded = match storage::load_document() {
                Ok(Some(doc)) => doc,
                Ok(None) => LegacyDocument::default(),
                Err(_e) => {
                    // Storage not available yet or first run — use default
                    LegacyDocument::default()
                }
            };
            let state: State<AppState> = app.state();
            if let Ok(mut doc) = state.document.lock() {
                *doc = loaded;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_document,
            update_document,
            export_html,
            export_html_with_questions,
            save_html_to_downloads,
            share_file,
            save_export,
            save_export_with_dialog,
            save_export_with_questions,
            get_print_html,
            import_file,
            merge_document,
            generate_passphrase,
            set_app_password,
            verify_app_password,
            has_app_password,
            change_app_password,
            clear_all_data,
            force_clear_all_data,
            get_clear_on_exit,
            set_clear_on_exit,
            clear_data_on_exit,
            open_external_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
