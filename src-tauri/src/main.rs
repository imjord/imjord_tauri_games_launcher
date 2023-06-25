// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rfd::FileDialog;
// use tauri::CustomMenuItem;
use std::process::Command;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

// Define the name of the text file that will store the paths of the games
const GAME_PATH_STORAGE: &str = "../game_paths.txt";
// A function that reads the game paths from the text file and returns them as a string
#[tauri::command]
fn get_games() -> Result<String, String> {
    // Attempt to read the contents of the game paths file
    let contents = fs::read_to_string(GAME_PATH_STORAGE)
        .map_err(|e| format!("Failed to read from file: {}", e))?;
    // Return the contents of the file
    Ok(contents)
}

// A function that opens a file dialog for the user to select a game executable,
// then adds the path of that executable to the game paths file
#[tauri::command]
fn get_file_path() -> Result<String, String> {
    // Create a new file dialog that filters for .exe files only
    let dialog = FileDialog::new().add_filter("Exe Files", &["exe"]);

    // Wait for the user to pick a file
    match dialog.pick_file() {
        // If the user picked a file
        Some(file_path) => {
            // Convert the file path to a string
            let file_path_str = file_path
                .to_str()
                .ok_or("Failed to convert path to string")?;
            // Open the game paths file in append mode
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(GAME_PATH_STORAGE)
                .map_err(|e| format!("Failed to open file: {}", e))?;
            // write a \n line 
            write!(file, "\n").map_err(|e| format!("Failed to write to file: {}", e))?;
            // Write the path of the new game to the file
            writeln!(file, "{}", file_path_str)
                .map_err(|e| format!("Failed to write to file: {}", e))?;
            // Return the path of the new game.
            Ok(file_path_str.to_string())
        },
        // If the user didn't pick a file, return an error
        None => Err("No file selected".to_string()),
    }
}

// A function that removes a game from the game paths file.
#[tauri::command]
fn remove_game(game_name: String) -> Result<String, String> {
    // Attempt to read the contents of the game paths file
    let contents = fs::read_to_string(GAME_PATH_STORAGE)
        .map_err(|e| format!("Failed to read from file: {}", e))?;

    // Split the contents into lines
    let lines: Vec<String> = contents.lines().map(String::from).collect();

    // Filter out the line that matches the game to remove
    let remaining_games: Vec<String> = lines
        .into_iter()
        .filter(|line| !line.contains(&game_name))
        .collect();

    // Write the remaining game paths back to the file
    fs::write(GAME_PATH_STORAGE, remaining_games.join("\n"))
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    // Return a success message
    Ok(format!("Game {} removed", game_name))
}

// A function that attempts to execute the file at the given path
#[tauri::command]
fn execute_file(file_path: String) -> Result<String, String> {
    // Attempt to execute the file
    match Command::new(file_path.clone())
        .output() {
        // If the file executed successfully
        Ok(output) => {
            // If the program exited with a non zero status code return an error
            if !output.status.success() {
                Err(format!("Failed to execute file: {:?}", output)) 
            } else {
                // otherwise return a success message
                Ok(format!("Successfully executed file: {:?}", output)) 
            }
        },
        // If the file failed to execute  return an error
        Err(err) => Err(err.to_string())
    }
}


fn main() {
    // Create the file if it does not exist
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(GAME_PATH_STORAGE)
        .expect("Failed to create file");
    tauri::Builder::default()
        // commands
        .invoke_handler(tauri::generate_handler![get_games, remove_game, execute_file,get_file_path])
        // run application.
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}