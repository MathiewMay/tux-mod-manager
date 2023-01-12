pub mod core;
pub mod download;
pub mod utils;

use tokio::runtime::Handle;

use tauri::Window;

use crate::mod_manager::game::Game;

//if you are coming from the Vue side of this method call and are wondering at
//what point the 'window' variable joins the mix, I don't know, but I had to dig
//through a lot to find it, but it would probably have been easier if I knew where
//to look in the documentation. Here is where I found it anyways:
//https://medium.com/@marm.nakamura/trying-to-the-tauri-gui-on-rust-4-state-management-on-the-rust-side-8899bda08936 (at 22:28 on June 8th 2022)
#[tauri::command]
pub async fn download(url: String, game: Game, window: Window) {
    let handle = Handle::current();
    handle.spawn_blocking(move || {
        let save_path = game.profile_path.join("downloads").clone();
        let parsed_url;
        match utils::parse_url(url.as_str()) {
            Ok(url) => parsed_url = url,
            Err(e) => {
                eprintln!(
                    "Something went wrong while trying to parse the url: '{}' Error message: {}",
                    url, e
                );
                return;
            }
        }
        match download::http_download(parsed_url, save_path, window, false, true, "0.1.0") {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Something went wrong while downloading: {}", e);
            }
        }
    });
}
