📋 Projet : Convertisseur YouTube → MP3
🎯 Objectif
Application de bureau locale permettant à une personne en situation de handicap mental de convertir, en autonomie, des vidéos YouTube en fichiers audio MP3 destinés à être transférés sur clé USB pour écoute en voiture et en chambre.
🧱 La stack
Côté interface (Front-end)

HTML5 / CSS3 — base du web
JavaScript (ES6+) — langage de la logique côté interface
React 19 — bibliothèque pour construire l'interface en composants
Vite 7 — outil de développement rapide (serveur dev, hot reload, build optimisé)
Lucide React — bibliothèque de pictogrammes (essentielle pour notre utilisatrice)

Côté natif (Back-end local)

Rust 1.95 — langage compilé, performant, sûr en mémoire
Cargo — gestionnaire de paquets + outil de build de Rust
Tauri 2 — framework qui combine WebView native + code Rust pour faire une app desktop

Outils externes embarqués dans l'app

yt-dlp — programme qui sait télécharger les flux YouTube
ffmpeg — programme qui sait convertir/transcoder l'audio (extraction et conversion MP3)

Outillage de développement

Node.js 24 + npm 11 — pour faire tourner Vite, gérer les paquets JS
Git + GitHub — versionnement du code
VS Code — éditeur de code
Extensions VS Code : rust-analyzer, Tauri

Format de sortie choisi

MP3 à 192 kbps — choix dicté par la compatibilité universelle (autoradios, vieux appareils USB)

❌ Ce qu'on a explicitement écarté et pourquoi

Electron → trop lourd en RAM (embarque Chromium entier).
PostgreSQL / Supabase / Vercel → aucune donnée à persister, aucun back-end web nécessaire.
Cloud / serveur distant → l'app doit tourner 100 % en local.
TypeScript (pour l'instant) → un seul nouveau langage à la fois (Rust suffit).
Version mobile (Android/iOS) → cible desktop Windows uniquement.

🗺️ Plan de route
✅ Étape 0 — Préparation de l'environnement (fait)
Installer Node, Rust (rustup), Build Tools C++. Vérifier WebView2. Concept appris : langages interprétés vs compilés, gestionnaires de versions.
✅ Étape 1 — Squelette Tauri + React (fait)
Créer le projet, explorer la structure, lancer l'app pour la première fois, commit initial. Concepts appris : structure d'un projet Tauri, séparation front/back, conventions sur conventions, builder pattern, premier appel JS → Rust avec la fonction greet.
🔜 Étape 2 — Première commande Rust personnalisée (à venir)
Remplacer greet par une fonction qui valide une URL YouTube. Concepts à apprendre : type Result<T, E> en Rust, gestion d'erreurs, expressions régulières (regex), invoke() côté React, async/await, communication d'erreurs entre Rust et JavaScript.
Étape 3 — Embarquer yt-dlp comme « sidecar »
Faire en sorte que l'app contienne yt-dlp.exe et puisse l'appeler. Concepts : binaires embarqués (sidecar binaries), permissions Tauri (capabilities), processus système.
Étape 4 — Téléchargement audio fonctionnel
Une URL → un fichier MP3 dans le dossier Téléchargements/MesAudios. Embarquer également ffmpeg pour la conversion MP3. Concepts : streaming de la progression, processus asynchrones, choix entre extraction et réencodage.
Étape 5 — Interface adaptée à l'utilisatrice
Refondre l'UI : grand champ pour coller l'URL, gros bouton avec pictogramme, retour visuel clair (✓/✗ géants, couleurs franches), pas de texte décoratif inutile. Concepts : conception centrée utilisateur, accessibilité cognitive, contraste visuel.
Étape 6 — Empaquetage et installation
Générer un installateur .msi pour Windows, installer l'app sur le PC de l'utilisatrice. Concepts : build de production, signature (optionnelle), distribution.
Étape 7 (bonus) — Durcissement

Ajout d'une CSP stricte dans tauri.conf.json
Mise en place d'un .gitattributes propre
Icône personnalisée pour l'app
Éventuellement : migration vers TypeScript

🧠 Concepts transverses déjà acquis
À garder en tête en permanence, ils dépassent ce projet :

Convention over invention — respecter les standards de chaque écosystème.
Incrémentalisme — un changement à la fois, toujours en état fonctionnel.
Fail fast — planter tôt et clairement plutôt que continuer en silence.
Principe du moindre privilège — n'accorder que les permissions nécessaires.
On versionne les recettes, pas les résultats — Git ignore node_modules/, target/, etc.
Vérifier avant de supposer — toujours tester, jamais supposer qu'une chose est installée.
Charge cognitive — un seul nouveau truc à apprendre à la fois.
Composants conteneur vs codec — distinction fondamentale en audio/vidéo.
Extraction > réencodage quand c'est possible (mais MP3 force le réencodage ici).

🔧 Commandes utiles à mémoriser
powershell# Lancer l'app en développement
npm run tauri dev

# Construire l'app pour distribution (plus tard)
npm run tauri build

# Vérifier l'état Git
git status

# Sauvegarder un état
git add .
git commit -m "Description claire à l'impératif présent"

# Mettre à jour Rust (à faire de temps en temps)
rustup update
📁 Structure du projet à retenir
convertisseur-yt/
├── src/             ← Front React (ton terrain connu)
├── src-tauri/       ← Back Rust (le nouveau monde)
│   ├── src/
│   │   ├── main.rs  ← Point d'entrée desktop (à ne pas toucher)
│   │   └── lib.rs   ← LE fichier où tu vas écrire tes commandes
│   ├── Cargo.toml   ← Dépendances Rust
│   └── tauri.conf.json  ← Config de l'app (taille fenêtre, etc.)
├── package.json     ← Dépendances JS
└── index.html       ← Point d'entrée du front