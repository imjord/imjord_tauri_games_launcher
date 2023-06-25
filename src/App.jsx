import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [file, setFile] = useState("");
  const [err, setErr] = useState("");
  const [files, setFiles] = useState([]);

  // Fetches the games and sets the state
  const fetchGames = async () => {
    try {
      const gameData = await invoke("get_games");
      const fileLines = gameData.split("\n").filter(Boolean);
      setFiles(fileLines);
    } catch (err) {
      setErr(err);
    }
  };

  useEffect(() => {
    fetchGames();
  }, []);

  async function openFileDialog() {
    try {
      await invoke("get_file_path");
      fetchGames(); // Fetch games after adding
    } catch (err) {
      console.error(err);
    }
  }

  async function removeGame(gameName) {
    try {
      await invoke("remove_game", { gameName });
      fetchGames(); // Fetch games after removing
    } catch (err) {
      console.error(err);
    }
  }

  async function executeFile(gamePath) {
    try {
      await invoke("execute_file", { filePath: gamePath });
    } catch (err) {
      console.error(err);
      setErr(err);
    }
  }

  return (
    <div className="container">
      <div className="title">
        <div>
          <h1>Favorite Games Launcher(made with rust)</h1>
        </div>
        <div>
          <button onClick={openFileDialog}>Add Game</button>
        </div>
      </div>

      <h2>Game List</h2>
      {files.map((game, index) => {
        const gamePathParts = game.split("\\");
        const gameName =
          gamePathParts[gamePathParts.length - 1].split(".exe")[0];

        return (
          <div key={index}>
            <p>{gameName}</p>
            <button onClick={() => executeFile(game)}>Play</button>
            <button onClick={() => removeGame(game)}>Remove</button>
          </div>
        );
      })}
      {err}
    </div>
  );
}

export default App;
