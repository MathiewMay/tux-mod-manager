use std::path::PathBuf;
use std::sync::mpsc;

pub mod utils;
pub mod download;
pub mod core;

use tokio::runtime::Handle;

use crate::mod_manager::game::Game;
use crate::mod_downloader::download::ProgressTracker;

enum UrlParsed<E> {
    Ok(),
    Err(E)
}

#[tauri::command]
pub async fn download(url: String, game: Game) {
    let handle = Handle::current();
    handle.spawn_blocking(move || {
        let progress_tracker = ProgressTracker {
            filename: url.as_str().to_owned(),
            current: None,
            length: None
        };
        let save_path = game.profile_path.join("downloads").clone();
        let parsed_url;
        match utils::parse_url(url.as_str()) {
            Ok(url) => { parsed_url = url },
            Err(e) => {
                eprintln!("Something went wrong while trying to parse the url: '{}'", url);
                return;
            }
        }
        match download::http_download(parsed_url, save_path, progress_tracker, false, true, "0.1.0") {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Something went wrong while downloading: {}", e);
            }
        }
    });
}