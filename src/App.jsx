import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [file, setFile] = useState("");
  const [err, setErr] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function openFileDialog() {
    try {
      const filePath = await invoke("get_file_path");
      setFile(filePath);
    } catch (err) {
      console.error(err);
    }
  }

  async function executeFile() {
    setErr("here");
    try {
      await invoke("execute_file", { filePath: file });
    } catch (err) {
      console.error(err);
      setErr(err);
    }
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
      <button onClick={() => openFileDialog()}>Open Tfauri Dialog</button>
      <p>{greetMsg}</p>
      <p>{file}</p> <button onClick={() => executeFile()}>Execute file</button>
      <p>{err}</p>
    </div>
  );
}

export default App;
