use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapitre {
    pub title: Option<String>,
    pub start_time: f64,
    pub end_time: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct InfoJson {
    #[serde(default)]
    chapters: Vec<ChapitreBrut>,
}

#[derive(Debug, Deserialize)]
struct ChapitreBrut {
    title: Option<String>,
    start_time: f64,
    end_time: Option<f64>,
}

// yt-dlp écrit <titre>.info.json à côté de <titre>.mp3 (--write-info-json, ajouté à
// telecharger_audio). Absent ou sans chapters[] -> liste vide, pas une erreur : c'est le
// cas normal pour un single/podcast sans chapitres.
pub fn lire_chapitres(chemin_mp3: &Path) -> Vec<Chapitre> {
    let chemin_json = chemin_mp3.with_extension("info.json");

    let contenu = match std::fs::read_to_string(&chemin_json) {
        Ok(texte) => texte,
        Err(_) => return Vec::new(),
    };

    let info: InfoJson = match serde_json::from_str(&contenu) {
        Ok(valeur) => valeur,
        Err(_) => return Vec::new(),
    };

    info.chapters
        .into_iter()
        .map(|c| Chapitre {
            title: c.title,
            start_time: c.start_time,
            end_time: c.end_time,
        })
        .collect()
}
