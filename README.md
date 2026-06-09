# convertisseur-yt

Application desktop Windows pour convertir des vidéos YouTube en fichiers MP3, conçue pour fonctionner entièrement en local et avec une interface accessible.

J'ai créé cette app pour une personne qui en avait besoin, aujourd'hui je la partage en espérant que ça aidera quelqu'un d'autre.

## Fonctionnalités

- Validation automatique d'URL YouTube au collage
- Affichage du titre et de la miniature de la vidéo
- Téléchargement et conversion en MP3 (192 kbps, qualité max)
- Suivi de progression en temps réel
- Interface sans texte instructionnel, basée sur des pictogrammes et codes couleur
- Fonctionnement 100% local, aucune donnée envoyée à un serveur tiers

## Stack technique

- **Front-end** : React 19, Vite, Lucide React, CSS
- **Back-end natif** : Rust, Tauri 2
- **Outils embarqués** : yt-dlp (téléchargement), ffmpeg (conversion)

## Installation (utilisateur)

Télécharger l'installateur depuis la section [Releases](https://github.com/Rekr-Online/convertisseur-yt/releases) et l'exécuter.

L'app s'installe dans `Program Files` et apparaît dans le menu Démarrer.

Les fichiers MP3 téléchargés se trouvent dans `Téléchargements/MesAudios`.

## Installation (développeur)

### Prérequis

- Node.js 20+
- Rust 1.75+ (via [rustup](https://rustup.rs/))
- Microsoft C++ Build Tools (proposés à l'installation de Rust)

### Mise en place

```bash
git clone https://github.com/Rekr-Online/convertisseur-yt.git
cd convertisseur-yt
npm install
```

### Binaires externes requis (non versionnés)

Télécharger et placer dans `src-tauri/binaries/` :

- **yt-dlp** : `yt-dlp.exe` depuis [github.com/yt-dlp/yt-dlp/releases/latest](https://github.com/yt-dlp/yt-dlp/releases/latest)
  → renommer en `yt-dlp-x86_64-pc-windows-msvc.exe`

- **ffmpeg** : `ffmpeg.exe` depuis [gyan.dev/ffmpeg/builds](https://www.gyan.dev/ffmpeg/builds/) (release-essentials)
  → renommer en `ffmpeg-x86_64-pc-windows-msvc.exe`

### Lancement en mode développement

```bash
npm run tauri dev
```

Première compilation : 3-10 minutes. Les suivantes : quelques secondes.

### Build de production

```bash
npm run tauri build
```

Les installateurs sont générés dans `src-tauri/target/release/bundle/`.

## Structure du projet

```
convertisseur-yt/
├── src/              # Front React (interface)
├── src-tauri/
│   ├── src/lib.rs    # Logique Rust (téléchargement, conversion)
│   ├── binaries/     # Sidecars yt-dlp et ffmpeg (non versionnés)
│   └── tauri.conf.json
└── package.json
```

## Licence

GPL v3 — voir le fichier [LICENSE](LICENSE) pour les détails complets.

---

*Un code qui n'existe pas est un code qui n'a pas de bug.*