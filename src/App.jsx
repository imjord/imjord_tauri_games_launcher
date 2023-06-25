import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Footer from "./component/Footer";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [file, setFile] = useState("");
  const [err, setErr] = useState("");
  const [files, setFiles] = useState([]);
  const [disclaimer, setDisclaimer] = useState(true);

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

  async function createDisclaimer() {
    try {
      await invoke("disclaimer");
      setDisclaimer(false);
    } catch (err) {
      console.error(err);
    }
  }

  async function checkDisclaimer() {
    try {
      await invoke("get_disclaimer");
      setDisclaimer(false);
    } catch (err) {
      console.error(err);
    }
  }

  useEffect(() => {
    fetchGames();
    checkDisclaimer();
  }, []);

  return (
    <div className="container">
      {disclaimer ? (
        <div className="disclaimer">
          <div className="inner-disclaimer">
            <div className="x">
              <span onClick={() => createDisclaimer()}>[dont show again]</span>
            </div>
            <div className="disclaimer-p">
              <p>
                Please be aware multiplayers games will launch but they wont
                connect to their respective launchers services.
              </p>
            </div>
          </div>
        </div>
      ) : null}

      <div className="title">
        <div>
          <h1>Favorite Games Launcher</h1>
        </div>
        <div>
          <button onClick={openFileDialog}>Add Game</button>
        </div>
      </div>
      <div className="game-list">
        <div></div>
        <div className="list">
          {files.map((game, index) => {
            const gamePathParts = game.split("\\");
            const gameName =
              gamePathParts[gamePathParts.length - 1].split(".exe")[0];

            return (
              <div key={index} className="list-content">
                <p>{gameName}</p>
                <div className="list-btns">
                  <button onClick={() => executeFile(game)}>Play</button>
                  <button onClick={() => removeGame(game)}>Remove</button>
                </div>
              </div>
            );
          })}
        </div>
      </div>
      <Footer />
    </div>
  );
}

export default App;
