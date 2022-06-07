use std::process;
use failure::{Fallible};

pub mod utils;
pub mod download;
pub mod core;

use tokio::task::spawn_blocking;

#[tauri::command]
pub async fn download(url: String) {
    println!("Downloading: {}", url); 
    match run(url).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }   
}

#[tauri::command]
pub async fn test(hi: String) {
  println!("test: {}", hi);
}

fn work(url: String) -> Fallible<()> {
    let parsed_url;
    match utils::parse_url(url.as_str()) {
        Ok(url) => { parsed_url = url },
        Err(e) => {
            eprintln!("Something went wrong while trying to parse the URL: {}", e);
            process::exit(1);
        }
    }
    download::http_download(parsed_url, false, false, "0.1.0")
}

async fn run(url_as_string: String) -> Fallible<()> {
    spawn_blocking(move || {
        work(url_as_string)
    }).await.unwrap()
}