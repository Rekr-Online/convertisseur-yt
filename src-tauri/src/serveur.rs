use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::chapitres::{self, Chapitre};

#[derive(Clone)]
pub struct EtatServeur {
    pub dossier_audios: PathBuf,
    pub ffmpeg_path: PathBuf,
}

#[derive(Serialize)]
pub struct FichierInfo {
    nom: String,
    taille_octets: u64,
    duree_sec: f64,
    chapitres: Vec<Chapitre>,
}

// /files donne la liste + métadonnées ; /download sert les mêmes fichiers en streaming,
// avec support des requêtes Range (ServeDir le gère nativement, indispensable pour
// qu'ExoPlayer côté app puisse chercher dans le fichier sans tout retélécharger).
pub fn construire_routeur(etat: EtatServeur) -> Router {
    let dossier_audios = etat.dossier_audios.clone();

    Router::new()
        .route("/files", get(lister_fichiers))
        .nest_service("/download", ServeDir::new(dossier_audios))
        .with_state(Arc::new(etat))
}

async fn lister_fichiers(State(etat): State<Arc<EtatServeur>>) -> Json<Vec<FichierInfo>> {
    let mut fichiers = Vec::new();

    let mut entrees = match tokio::fs::read_dir(&etat.dossier_audios).await {
        Ok(lecteur) => lecteur,
        Err(_) => return Json(fichiers),
    };

    while let Ok(Some(entree)) = entrees.next_entry().await {
        let chemin = entree.path();
        if chemin.extension().and_then(|e| e.to_str()) != Some("mp3") {
            continue;
        }

        let taille_octets = entree.metadata().await.map(|m| m.len()).unwrap_or(0);
        let duree_sec = obtenir_duree(&etat.ffmpeg_path, &chemin).await.unwrap_or(0.0);
        let chapitres = chapitres::lire_chapitres(&chemin);

        fichiers.push(FichierInfo {
            nom: entree.file_name().to_string_lossy().to_string(),
            taille_octets,
            duree_sec,
            chapitres,
        });
    }

    Json(fichiers)
}

// ffmpeg -i <fichier> échoue toujours (aucune sortie demandée) mais affiche la durée sur
// stderr : la façon standard d'obtenir une durée sans binaire ffprobe séparé, en réutilisant
// le ffmpeg déjà embarqué en sidecar pour la conversion.
async fn obtenir_duree(ffmpeg_path: &std::path::Path, fichier: &std::path::Path) -> Option<f64> {
    let sortie = tokio::process::Command::new(ffmpeg_path)
        .arg("-i")
        .arg(fichier)
        .output()
        .await
        .ok()?;

    let texte_erreur = String::from_utf8_lossy(&sortie.stderr);
    let regex = regex::Regex::new(r"Duration: (\d+):(\d+):(\d+\.\d+)").ok()?;
    let captures = regex.captures(&texte_erreur)?;

    let heures: f64 = captures.get(1)?.as_str().parse().ok()?;
    let minutes: f64 = captures.get(2)?.as_str().parse().ok()?;
    let secondes: f64 = captures.get(3)?.as_str().parse().ok()?;

    Some(heures * 3600.0 + minutes * 60.0 + secondes)
}
