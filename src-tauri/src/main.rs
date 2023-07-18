// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rfd::FileDialog;
use std::process::Command;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

/// This file path is for storing the game paths chosen by the user.
/// 
/// constant represents file path for txt file called "game_paths.txt".
/// The file is expected to be made in the same directory along side the app.
/// The contents of this file used to store the paths related to the exe chosen by the user.
const GAME_PATH_STORAGE: &str = "./game_paths.txt";
/// This file path is for the disclaimer created due to user click.
/// 
/// Constant represents file path for the txt file called "settings.txt".
/// The file is expected to be created in the same directory along side the app.
/// The contents of this file will be empty. it is only created once a user clicks "dont show again" on the app.
/// If this file is there the disclaimer wont show in app. if this file is absent the disclaimer will show.
const DISCLAIMER_SETTINGS: &str = "./settings.txt";




/// Function that Creates the disclaimer settings file.
/// 
/// This function is invoked as a Tauri command using #[tauri::command].
/// It creates a settings file named "settings.txt" in the current directory.
/// 
/// # Panics
/// 
/// This function will panic if it fails to create the file. With the following error Failed to create settings file
#[tauri::command]
fn disclaimer() {
 OpenOptions::new()
 .write(true)
 .create(true)
 .open(DISCLAIMER_SETTINGS)
 .expect("Failed to create settings file");
}

/// Function that "checks" to see if the settings file is in the root directory returns string.
/// 
/// This function is invoked as a Tauri command using #[tauri::command].
/// It creates a settings file named [settings.txt](constant@crate::DISCLAIMER_SETTINGS) in the current directory, which will be used to store disclaimer settings.
/// 
/// # Panics
/// 
/// This function will panic if the file is not in the root directory with the following error "Failed to read from file: "error"."
#[tauri::command]
fn get_disclaimer() -> Result<String, String> {
    let contents = fs::read_to_string(DISCLAIMER_SETTINGS)
    .map_err(|e| format!("Failed to read from file: {}", e))?;
    Ok(contents)
}


/// Function that reads the game paths from the text file and returns them as a string.
/// 
/// This function is invoked as a Tauri command using #[tauri::command].
/// It reads the contents of [game_paths.txt](constant@crate::GAME_PATH_STORAGE) in the current directory, which will be returned as string.
///
/// # Panics
/// 
/// This function will panic if it cant read the file with the following error "Failed to read from file: "error""
#[tauri::command]
fn get_games() -> Result<String, String> {
    let contents = fs::read_to_string(GAME_PATH_STORAGE)
        .map_err(|e| format!("Failed to read from file: {}", e))?;
    Ok(contents)
}

/// A function that opens a file dialog for the user to select a game executable returns a string.
/// 
/// /// This function is invoked as a Tauri command using #[tauri::command].
/// This function filters .exe files only.
/// Dialog will show for the user to pick a exe.
/// Picked file path will be converted to a string.
/// file path will be appended to the [game_paths.txt](constant@crate::GAME_PATH_STORAGE) txt file.
/// if no file is selected it returns none.
/// 
/// # Panics
/// 
/// This function will panic if it fails to convert path to string, fails to open the file to append the path, fails to write to the game_paths file.
#[tauri::command]
fn get_file_path() -> Result<String, String> {
    let dialog = FileDialog::new().add_filter("Exe Files", &["exe"]);

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

/// A function that removes a game from the game paths file and returns a Result<String, String>.
/// 
/// This function is invoked as a Tauri command using #[tauri::command].
/// This function will attempt to read the contents of [game_paths.txt](constant@crate::GAME_PATH_STORAGE).
/// This function will filter the lines that matches the math and remove. 
/// The paths not removed will write back to the file. 
/// 
/// # Panics
/// 
/// This function will panic if it fails to read from file, or fails to write the paths back.
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

/// A function that attempts to execute the file at the given path
/// 
/// This function is invoked as a Tauri command using #[tauri::command].
/// This function is used to execute the content/paths from [game_paths.txt](constant@crate::GAME_PATH_STORAGE).
/// 
/// # Panics
/// 
/// This function will panic if it fails to execute the path files!
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


/// Main.
///
/// Will create the file [game_paths.txt](constant@crate::GAME_PATH_STORAGE) to main directory if it doesnt exist already,
/// 
/// # Panics
///
/// This function will panic if it fails to create the file specified by the constant [game_paths.txt](constant@crate::GAME_PATH_STORAGE).

fn main() {
    // Create the file if it does not exist on main launch
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(GAME_PATH_STORAGE)
        .expect("Failed to create file");
   
    tauri::Builder::default()
        // commands
        .invoke_handler(tauri::generate_handler![get_games, disclaimer, get_disclaimer, remove_game, execute_file,get_file_path])
        // run application.
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}