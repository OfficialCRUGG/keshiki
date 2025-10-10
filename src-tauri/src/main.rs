// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod wallpaper;

use tauri::Manager;
use tauri::api::dialog;
use tauri::Window;

#[tauri::command]
fn expand_scope(app_handle: tauri::AppHandle, folder_path: std::path::PathBuf) -> Result<(), String> {
  app_handle.fs_scope().allow_directory(&folder_path, true)
    .map_err(|err| err.to_string())?;
  Ok(())
}

#[tauri::command]
fn set_wallpaper(window: Window, path: String) -> Result<(), String> {
  println!("Setting wallpaper to: {}", path);
  // Get operating system
  let os = std::env::consts::OS;
  if os == "macos" {
    wallpaper::macos::set(&path)?;
  } else if os == "windows" {
    wallpaper::windows::set(&path)?;
  } else if os == "linux" {
    dialog::message(Some(&window), "Linux support", "Keshiki currently only supports Windows and macOS. Linux support is however planned for the future at some point.")
  }
  Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![expand_scope, set_wallpaper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
