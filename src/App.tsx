import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Greet from "./models/greet";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [encryotedGreetMsg, setEncryptedGreetMsg] = useState("");
  const [decryptedGreetMsg, setDecryptedGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let greet = await invoke("greet", { name }) as Greet
    setGreetMsg(greet.greetMsg);
    setEncryptedGreetMsg(greet.encryptedGreetMsg)
    setDecryptedGreetMsg(greet.decryptedGreetMsg)
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
      <p>{encryotedGreetMsg}</p>
      <p>{decryptedGreetMsg}</p>
    </div>
  );
}

export default App;
