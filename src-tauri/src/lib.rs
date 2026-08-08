use regex::Regex;
use std::sync::Mutex;
use tauri_plugin_shell::ShellExt;

mod chapitres;
mod serveur;

// Adresse du serveur de transfert une fois démarré, gardée pour ne pas en relancer un
// second si l'utilisateur reclique sur "Transfert de musique".
pub struct EtatServeurTransfert {
    adresse: Mutex<Option<String>>,
}

// Garde la fonction greet pour test
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

//valide une URL YouTube
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
// Obtient le titre d'une vidéo YouTube à partir de son URL
#[tauri::command]
async fn obtenir_titre_video(
    app: tauri::AppHandle,
    url: String,
) -> Result<String, String> {

    let sortie = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("Sidecar introuvable : {}", e))?
        .args([
            "--get-title",
            "--no-playlist",
            "--extractor-args", "youtube:player_client=android_vr",
            &url,
        ])
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
// télécharge la vidéo yt et la convertit en mp3
#[tauri::command]
async fn telecharger_audio(
    app: tauri::AppHandle,
    url: String,
) -> Result<String, String> {
    use tauri::path::BaseDirectory;
    use tauri::{Manager, Emitter};
    use tauri_plugin_shell::process::CommandEvent;

    let dossier = dirs::download_dir()
        .ok_or("Dossier Téléchargements introuvable")?
        .join("MesAudios");

    std::fs::create_dir_all(&dossier)
        .map_err(|e| format!("Impossible de créer le dossier : {}", e))?;

    let ffmpeg_path = app
        .path()
        .resolve("binaries/ffmpeg-x86_64-pc-windows-msvc.exe", BaseDirectory::Resource)
        .map_err(|e| format!("ffmpeg introuvable : {}", e))?;

    let modele_sortie = dossier.join("%(title)s.%(ext)s");

    // Lance yt-dlp en mode streaming (au lieu de .output())
    let (mut rx, _child) = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| format!("yt-dlp introuvable : {}", e))?
        .args([
            "-x",
            "--extractor-args", "youtube:player_client=android_vr",
            "--audio-format", "mp3",
            "--audio-quality", "0",
            "--no-playlist",
            "--newline",
            "--write-info-json",
            "--ffmpeg-location", ffmpeg_path.to_str().ok_or("Chemin ffmpeg invalide")?,
            "-o", modele_sortie.to_str().ok_or("Chemin invalide")?,
            &url,
        ])
        .spawn()
        .map_err(|e| format!("Erreur d'exécution : {}", e))?;

    // Regex pour extraire le pourcentage de progression
    let regex_progression = Regex::new(r"(\d+\.?\d*)%")
        .map_err(|e| format!("Regex invalide : {}", e))?;

    let mut succes = false;

    // Lit la sortie ligne par ligne au fur et à mesure
    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(ligne) = event {
            let texte = String::from_utf8_lossy(&ligne);  // ← ligne à ajouter
            if let Some(captures) = regex_progression.captures(&texte) {
                if let Some(pourcentage) = captures.get(1) {
                    let valeur: f32 = pourcentage.as_str().parse().unwrap_or(0.0);
                    app.emit("progression", valeur)
                        .map_err(|e| format!("Erreur d'émission : {}", e))?;
                }
            }
        } else if let CommandEvent::Terminated(payload) = event {
            succes = payload.code == Some(0);
        }
    }

    if succes {
        Ok(format!("Téléchargement réussi dans : {}", dossier.display()))
    } else {
        Err("Échec du téléchargement".to_string())
    }
}

// Démarre le serveur de transfert Wi-Fi (action explicite, pas au lancement de l'app —
// choisi avec Leo pour ne pas garder un port ouvert en permanence). Idempotent : si déjà
// démarré, renvoie l'adresse existante plutôt que d'ouvrir un deuxième serveur.
#[tauri::command]
async fn demarrer_serveur_transfert(
    app: tauri::AppHandle,
    etat: tauri::State<'_, EtatServeurTransfert>,
) -> Result<String, String> {
    use tauri::path::BaseDirectory;
    use tauri::Manager;

    {
        let adresse_existante = etat.adresse.lock().unwrap();
        if let Some(adresse) = adresse_existante.as_ref() {
            return Ok(adresse.clone());
        }
    }

    let dossier_audios = dirs::download_dir()
        .ok_or("Dossier Téléchargements introuvable")?
        .join("MesAudios");

    let ffmpeg_path = app
        .path()
        .resolve("binaries/ffmpeg-x86_64-pc-windows-msvc.exe", BaseDirectory::Resource)
        .map_err(|e| format!("ffmpeg introuvable : {}", e))?;

    let routeur = serveur::construire_routeur(serveur::EtatServeur {
        dossier_audios,
        ffmpeg_path,
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:0")
        .await
        .map_err(|e| format!("Impossible d'ouvrir le port : {}", e))?;
    let port = listener
        .local_addr()
        .map_err(|e| format!("Adresse locale introuvable : {}", e))?
        .port();

    tokio::spawn(async move {
        axum::serve(listener, routeur).await.ok();
    });

    let ip = local_ip_address::local_ip()
        .map_err(|e| format!("IP locale introuvable : {}", e))?;
    let adresse_complete = format!("http://{}:{}", ip, port);

    *etat.adresse.lock().unwrap() = Some(adresse_complete.clone());

    Ok(adresse_complete)
}

// Génère un QR code à partir de n'importe quel texte (ici l'adresse du serveur), encodé en
// PNG puis base64 — le plus simple à transmettre à React, qui l'affiche directement dans un
// <img src="data:image/png;base64,...">, sans fichier temporaire à gérer.
#[tauri::command]
fn generer_qr_code(texte: String) -> Result<String, String> {
    use base64::{engine::general_purpose, Engine as _};
    use image::Luma;
    use qrcode::QrCode;

    let code = QrCode::new(texte.as_bytes()).map_err(|e| format!("QR invalide : {}", e))?;
    let image = code.render::<Luma<u8>>().build();

    let mut tampon = std::io::Cursor::new(Vec::new());
    image
        .write_to(&mut tampon, image::ImageFormat::Png)
        .map_err(|e| format!("Encodage PNG échoué : {}", e))?;

    Ok(general_purpose::STANDARD.encode(tampon.into_inner()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(EtatServeurTransfert {
            adresse: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            valider_url_youtube,
            obtenir_titre_video,
            telecharger_audio,
            demarrer_serveur_transfert,
            generer_qr_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}