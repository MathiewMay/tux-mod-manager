use std::process;
use std::path::PathBuf;
use failure::{Fallible};

pub mod utils;
pub mod download;
pub mod core;

use tokio::task::spawn_blocking;

use crate::mod_manager::game::Game;

#[tauri::command]
pub async fn download(url: String, game: Game) {
    println!("Downloading: {}", url); 
    let save_path = game.profile_path.join("downloads");
    match run(url, save_path).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }   
}

fn work(url: String, save_path: PathBuf) -> Fallible<()> {
    let parsed_url;
    match utils::parse_url(url.as_str()) {
        Ok(url) => { parsed_url = url },
        Err(e) => {
            eprintln!("Something went wrong while trying to parse the URL: {}", e);
            process::exit(1);
        }
    }
    download::http_download(parsed_url, save_path, false, true, "0.1.0")
}

async fn run(url_as_string: String, save_path: PathBuf) -> Fallible<()> {
    spawn_blocking(move || {
        work(url_as_string, save_path)
    }).await.unwrap()
}