#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod mod_downloader;
mod mod_manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            mod_manager::uncompress,
            mod_manager::scan_games,
            mod_manager::deploy,
            mod_manager::get_mods,
            mod_manager::remove_mod,
            mod_manager::scan_for_single_game,
            mod_downloader::download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
