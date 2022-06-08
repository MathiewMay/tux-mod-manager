use std::path::PathBuf;

pub mod utils;
pub mod download;
pub mod core;

use tokio::task::spawn_blocking;

use crate::mod_manager::game::Game;

enum UrlParsed<E> {
    Ok(),
    Err(E)
}

#[tauri::command]
pub async fn download(url: String, game: Game) {
    println!("Downloading: '{}'", url);
    let save_path = game.profile_path.join("downloads");
    match run(url, save_path).await {
        UrlParsed::Ok() => {}
        UrlParsed::Err(e) => {
            eprintln!("error: {}", e);
        }
    }   
}

fn work(url: String, save_path: PathBuf) -> UrlParsed<String> {
    let parsed_url;
    match utils::parse_url(url.as_str()) {
        Ok(url) => { parsed_url = url },
        Err(e) => {
            return UrlParsed::Err(format!("Url couldn't be parsed: '{}'\nError Message:\n{}", &url, e).to_owned());
        }
    }
    match download::http_download(parsed_url, save_path, false, true, "0.1.0") {
        Ok(_) => { return UrlParsed::Ok(); },
        Err(e) => {
            return UrlParsed::Err(format!("Failure while downloading file: '{}'\nError Message:\n{}", &url, e).to_owned());
        }
    }
}

async fn run(url_as_string: String, save_path: PathBuf) -> UrlParsed<String> {
    spawn_blocking(move || {
        work(url_as_string, save_path)
    }).await.unwrap()
}