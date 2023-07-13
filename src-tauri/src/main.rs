// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod hash;

use hash::calc_file_hash;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calc_file_hash])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
