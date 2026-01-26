#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod encryption;
mod export;
mod models;
mod storage;

use models::LegacyDocument;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    document: Mutex<LegacyDocument>,
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
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    export::generate_encrypted_html(&doc, &passphrase, include_welcome_screen.unwrap_or(false)).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_export(state: State<AppState>, passphrase: String, file_path: String, include_welcome_screen: Option<bool>) -> Result<(), String> {
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    let html = export::generate_encrypted_html(&doc, &passphrase, include_welcome_screen.unwrap_or(false)).map_err(|e| e.to_string())?;
    std::fs::write(&file_path, html).map_err(|e| format!("Failed to save file: {}", e))
}

#[tauri::command]
async fn save_export_with_dialog(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    passphrase: String,
    include_welcome_screen: bool,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let doc = state.document.lock().map_err(|e| e.to_string())?;
    let html = export::generate_encrypted_html(&doc, &passphrase, include_welcome_screen).map_err(|e| e.to_string())?;
    drop(doc); // Release lock before dialog

    // Generate filename with current date
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
            std::fs::write(&path_str, html)
                .map_err(|e| format!("Failed to save file: {}", e))?;
            Ok(Some(path_str))
        }
        None => Ok(None), // User cancelled
    }
}

#[tauri::command]
fn get_print_html(state: State<AppState>) -> Result<String, String> {
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    Ok(export::generate_print_html(&doc))
}

#[tauri::command]
fn import_file(encrypted_html: String, passphrase: String) -> Result<LegacyDocument, String> {
    export::import_from_html(&encrypted_html, &passphrase).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_file(file_path: String) -> Result<String, String> {
    std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))
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

    let words = vec![
        "apple", "banana", "cherry", "dragon", "eagle", "forest", "garden", "harbor",
        "island", "jungle", "kitchen", "lemon", "mountain", "nectar", "ocean", "piano",
        "quartz", "river", "sunset", "thunder", "umbrella", "violet", "window", "yellow",
        "zebra", "anchor", "beacon", "castle", "diamond", "ember", "falcon", "glacier",
    ];

    let mut rng = rand::thread_rng();
    let selected: Vec<&str> = words.choose_multiple(&mut rng, 4).cloned().collect();
    selected.join("-")
}

#[tauri::command]
fn set_app_password(password: String) -> Result<(), String> {
    let hash = storage::hash_password(&password).map_err(|e| e.to_string())?;
    storage::save_password_hash(&hash).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn verify_app_password(password: String) -> Result<bool, String> {
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

fn main() {
    // Try to load existing document, or create new one
    let document = storage::load_document()
        .ok()
        .flatten()
        .unwrap_or_default();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            document: Mutex::new(document),
        })
        .invoke_handler(tauri::generate_handler![
            get_document,
            update_document,
            export_html,
            save_export,
            save_export_with_dialog,
            get_print_html,
            import_file,
            read_file,
            merge_document,
            generate_passphrase,
            set_app_password,
            verify_app_password,
            has_app_password,
            change_app_password,
            clear_all_data,
            get_clear_on_exit,
            set_clear_on_exit,
            clear_data_on_exit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
