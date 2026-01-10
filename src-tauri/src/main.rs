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
fn export_html(state: State<AppState>, passphrase: String) -> Result<String, String> {
    let doc = state.document.lock().map_err(|e| e.to_string())?;
    export::generate_encrypted_html(&doc, &passphrase).map_err(|e| e.to_string())
}

#[tauri::command]
fn import_file(_encrypted_html: String, _passphrase: String) -> Result<LegacyDocument, String> {
    // Extract encrypted data from HTML and decrypt
    // This is a simplified version - full implementation would parse the HTML
    Err("Import not yet implemented".to_string())
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

fn main() {
    // Try to load existing document, or create new one
    let document = storage::load_document()
        .ok()
        .flatten()
        .unwrap_or_default();

    tauri::Builder::default()
        .manage(AppState {
            document: Mutex::new(document),
        })
        .invoke_handler(tauri::generate_handler![
            get_document,
            update_document,
            export_html,
            import_file,
            generate_passphrase,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
