import { invoke } from "@tauri-apps/api/core";
import { Download, ThumbsUp, Home, QrCode, X } from "lucide-react";
import { listen } from "@tauri-apps/api/event";
import { useState, useEffect } from "react";
import "./App.css";

// Regex pour extraire l'ID d'une URL YouTube (les 11 caractères de l'ID)
const REGEX_ID = /(?:youtube\.com\/(?:watch\?v=|shorts\/)|youtu\.be\/)([a-zA-Z0-9_-]{11})/;

function App() {
  const [etat, setEtat] = useState("repos");
  const [url, setUrl] = useState("");
  const [titre, setTitre] = useState("");
  const [idVideo, setIdVideo] = useState("");
  const [progression, setProgression] = useState(0);
  const [adresseServeur, setAdresseServeur] = useState(null);
  const [qrCode, setQrCode] = useState(null);
  const [chargementQr, setChargementQr] = useState(false);
  const [modalOuvert, setModalOuvert] = useState(false);


useEffect(() => {
  const unlisten = listen("progression", (event) => {
    setProgression(event.payload);
  });

  return () => {
    unlisten.then((fn) => fn());
  };
}, []);
  // Appelée quand l'utilisateur colle dans le champ URL
  async function gererCollage(e) {
    e.preventDefault();
    const texteColle = e.clipboardData.getData("text");
    setUrl(texteColle);

    try {
      await invoke("valider_url_youtube", { url: texteColle });

      const match = texteColle.match(REGEX_ID);
      if (match) {
        setIdVideo(match[1]);
      }

      try {
        const titreRecu = await invoke("obtenir_titre_video", { url: texteColle });
        setTitre(titreRecu);
        setEtat("repos");
      } catch {
        setEtat("invalide");
        setTitre("");
        setIdVideo("");
      }
    } catch {
      setEtat("invalide");
      setTitre("");
      setIdVideo("");
    }
  }
async function lancerTelechargement() {
  setProgression(0);
  setEtat("telechargement");
  try {
    await invoke("telecharger_audio", { url });
    setEtat("succes");
  } catch {
    setEtat("echec");
  }
}
// Démarre le serveur (idempotent côté Rust) puis génère son QR — les deux ne sont
// relancés qu'une fois, un réaffichage du modal réutilise l'adresse déjà obtenue.
async function ouvrirTransfert() {
  setModalOuvert(true);
  if (qrCode) return;

  setChargementQr(true);
  try {
    const adresse = await invoke("demarrer_serveur_transfert");
    setAdresseServeur(adresse);
    const image = await invoke("generer_qr_code", { texte: adresse });
    setQrCode(image);
  } catch (erreur) {
    console.error("Erreur transfert :", erreur);
  } finally {
    setChargementQr(false);
  }
}

function retourRepos() {
  setEtat("repos");
  setUrl("");
  setTitre("");
  setIdVideo("");
  setProgression(0);
}

  return (
    <main className="conteneur">
      <button
        type="button"
        className="bouton-qr"
        onClick={ouvrirTransfert}
        title="Transfert de musique"
      >
        <QrCode size={22} />
      </button>

      {modalOuvert && (
        <div className="superposition-qr" onClick={() => setModalOuvert(false)}>
          <div className="carte-qr" onClick={(e) => e.stopPropagation()}>
            {chargementQr && <p>Démarrage du serveur...</p>}
            {qrCode && (
              <img
                src={`data:image/png;base64,${qrCode}`}
                alt="QR code de connexion"
                className="image-qr"
              />
            )}
            {adresseServeur && <div className="adresse-qr">{adresseServeur}</div>}
            <button type="button" className="bouton-fermer-qr" onClick={() => setModalOuvert(false)}>
              <X size={18} /> Fermer
            </button>
          </div>
        </div>
      )}

      <input
        type="text"
        className={`champ-url ${etat === "invalide" ? "champ-url--invalide" : ""}`}
        value={url}
        onChange={(e) => setUrl(e.currentTarget.value)}
        onPaste={gererCollage}
        placeholder="Coller ici..."
      />

      {/* Cadre titre + miniature, visible seulement si on a un titre */}
      {titre && idVideo && (
        <div className="cadre-info">
          <img
            src={`https://img.youtube.com/vi/${idVideo}/mqdefault.jpg`}
            alt=""
            className="miniature"
          />
          <div className="titre">{titre}</div>
        </div>
      )}

<div className="zone-action">
{etat === "succes" && (
  <button className="medaillon-cliquable medaillon-cliquable--succes" onClick={retourRepos}>
    <Home size={60} color="#FFFFFF" />
  </button>
)}
{etat === "telechargement" && (
    <div className="barre-conteneur">
      <div
        className="barre-remplissage"
        style={{ width: `${progression}%` }}
      ></div>
    </div>
  )}
{etat === "echec" && (
  <button className="medaillon-cliquable medaillon-cliquable--echec" onClick={retourRepos}>
    <Home size={60} color="#FFFFFF" />
  </button>
)}
  {etat !== "telechargement" && etat !== "succes" && etat !== "echec" && (
    <button
      type="button"
      className={`bouton-principal ${etat === "invalide" ? "bouton-principal--invalide" : ""}`}
      disabled={etat === "invalide"}
      onClick={lancerTelechargement}
    >
      <Download size={44} />
      {etat === "invalide" && (
        <svg className="barre-invalide" viewBox="0 0 120 120">
          <line x1="20" y1="20" x2="100" y2="100" stroke="#C8312E" strokeWidth="8" strokeLinecap="round" />
        </svg>
      )}
    </button>
  )}
</div>
    </main>
  );
}

export default App;