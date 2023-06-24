// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rfd::FileDialog;
use tauri::CustomMenuItem;
use tauri::Manager;
use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_file_path() -> Result<String, String> {
    let dialog = FileDialog::new().add_filter("Exe Files", &["exe"]);

    match dialog.pick_file(){
        Some(file_path) => Ok(file_path.to_string_lossy().into_owned()),
        None => Err("No file selected".to_string()),
    }
}

#[tauri::command]
fn execute_file(file_path: String) -> Result<String, String> {
    println!("Trying to execute file: {}", &file_path); // log file path
    match Command::new(file_path.clone())
        .output() {
        Ok(output) => {
            println!("Output: {:?}", output); // log command output
            if !output.status.success() {
                Err(format!("Failed to execute file: {:?}", output)) // provide command output in error
            } else {
                Ok(format!("Successfully executed file: {:?}", output)) // provide command output in success
            }
        },
        Err(err) => Err(err.to_string())
    }
}
// fn execute_file(file_path: String) -> Result<(), String> {
//     let output = Command::new(file_path)
//         .output()
//         .map_err(|err| err.to_string())?;

//     if !output.status.success() {
//         return Err("Failed to execute file".to_string());
//     }

//     Ok(())
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, execute_file,get_file_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
