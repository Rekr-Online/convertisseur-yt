import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [url, setUrl] = useState("");
  const [message, setMessage] = useState("");
  const [statut, setStatut] = useState("");
  const [chargement, setChargement] = useState(false);

  async function obtenirTitre() {
    setChargement(true);
    setMessage("Recherche du titre...");
    setStatut("");
    try {
      const titre = await invoke("obtenir_titre_video", { url });
      setMessage(`Titre : ${titre}`);
      setStatut("succes");
    } catch (erreur) {
      setMessage(erreur);
      setStatut("erreur");
    } finally {
      setChargement(false);
    }
  }

  async function telecharger() {
    setChargement(true);
    setMessage("Téléchargement en cours...");
    setStatut("");
    try {
      const resultat = await invoke("telecharger_audio", { url });
      setMessage(resultat);
      setStatut("succes");
    } catch (erreur) {
      setMessage(erreur);
      setStatut("erreur");
    } finally {
      setChargement(false);
    }
  }

  return (
    <main className="container">
      <h1>Convertisseur YouTube</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          telecharger();
        }}
      >
        <input
          value={url}
          onChange={(e) => setUrl(e.currentTarget.value)}
          placeholder="Colle ici une URL YouTube..."
        />
        <button type="button" onClick={obtenirTitre} disabled={chargement}>
          Obtenir le titre
        </button>
        <button type="submit" disabled={chargement}>
          Télécharger en MP3
        </button>
      </form>
      {message && <p className={statut}>{message}</p>}
    </main>
  );
}

export default App;