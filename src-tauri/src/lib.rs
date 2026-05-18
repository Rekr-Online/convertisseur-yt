use regex::Regex;

// Garde la fonction greet pour test
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Nouvelle fonction : valide une URL YouTube
#[tauri::command]
fn valider_url_youtube(url: &str) -> Result<String, String> {
    let motif = r"^(https?://)?(www\.|m\.|music\.)?(youtube\.com/(watch\?v=|shorts/)|youtu\.be/)([a-zA-Z0-9_-]{11})";

    let regex = Regex::new(motif).map_err(|e| format!("Regex invalide : {}", e))?;

    if regex.is_match(url) {
        Ok(format!("URL YouTube valide : {}", url))
    } else {
        Err(format!("URL YouTube invalide : {}", url))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, valider_url_youtube])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}