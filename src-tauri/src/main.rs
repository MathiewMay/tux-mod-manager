#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod mod_manager;
mod mod_downloader;

use mod_downloader::Download;

struct Downloads(Vec<Download>);

fn main() {
  tauri::Builder::default()
    .manage(Downloads(Vec::new()))
    .invoke_handler(tauri::generate_handler![
      mod_manager::uncompress, 
      mod_manager::scan_games, 
      mod_manager::deploy,
      mod_manager::get_mods,
      mod_manager::remove_mod,
      mod_downloader::download
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

