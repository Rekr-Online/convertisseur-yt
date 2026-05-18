use regex::Regex;
use tauri_plugin_shell::ShellExt;

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
#[tauri::command]
async fn obtenir_titre_video(
    app: tauri::AppHandle,
    url: String,
) -> Result<String, String> {
    let sortie = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Sidecar introuvable : {}", e))?
        .args(["--get-title", &url])
        .output()
        .await
        .map_err(|e| format!("Erreur d'exécution : {}", e))?;

    if sortie.status.success() {
        let titre = String::from_utf8_lossy(&sortie.stdout).trim().to_string();
        Ok(titre)
    } else {
        let erreur = String::from_utf8_lossy(&sortie.stderr).to_string();
        Err(format!("yt-dlp a échoué : {}", erreur))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, valider_url_youtube, obtenir_titre_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}