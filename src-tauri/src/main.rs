// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn expand_scope(app_handle: tauri::AppHandle, folder_path: std::path::PathBuf) -> Result<(), String> {
  app_handle.fs_scope().allow_directory(&folder_path, true)
    .map_err(|err| err.to_string())?;
  Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![expand_scope])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
