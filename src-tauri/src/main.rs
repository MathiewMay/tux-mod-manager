#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod mod_manager;

use std::fs::File;
use std::path::{Path};
use compress_tools::*;

fn main() {
  mod_manager::scan_games();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![uncompress, mod_manager::scan_games])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn uncompress(file_path: String, target_path: String) {
  let mut source_file = File::open(file_path).unwrap();
  let target = Path::new(&target_path);
  uncompress_archive(&mut source_file,&target, Ownership::Ignore).unwrap();
}