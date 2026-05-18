import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [url, setUrl] = useState("");
  const [message, setMessage] = useState("");
  const [statut, setStatut] = useState("");

  async function verifierUrl() {
    try {
      const resultat = await invoke("valider_url_youtube", { url });
      setMessage(resultat);
      setStatut("succes");
    } catch (erreur) {
      setMessage(erreur);
      setStatut("erreur");
    }
  }

  return (
    <main className="container">
      <h1>Convertisseur YouTube</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          verifierUrl();
        }}
      >
        <input
          value={url}
          onChange={(e) => setUrl(e.currentTarget.value)}
          placeholder="Colle ici une URL YouTube..."
        />
        <button type="submit">Vérifier</button>
      </form>
      {message && <p className={statut}>{message}</p>}
    </main>
  );
}

export default App;