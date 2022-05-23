use std::{path::PathBuf};
use serde::{Deserialize, Serialize};

use dirs;

#[derive(Serialize, Deserialize)]
struct Game {
  name: String,
  path: String,
}

#[tauri::command]
pub(crate) fn scan_games() -> Vec<String> {
  let mut steam_games: Vec<String> = Vec::new();
  let home_dir = dirs::home_dir().unwrap();
  let steam_local_dir = home_dir.join(".local/share/Steam/steamapps/common/");
  let steam_flatpak_dir = home_dir.join(".var/app/com.valvesoftware.Steam/data/Steam/steamapps/common/");
  let steam_dirs = [steam_local_dir, steam_flatpak_dir];
  for dir in steam_dirs {
    if dir.exists() {
      let steam_directories = get_directories(dir);
      for game_entry in steam_directories {
        let game_entry_name = game_entry.file_name().unwrap().to_str().unwrap().to_string();
        let game_entry_path = game_entry.to_str().unwrap().to_string();
        let game = Game {name: game_entry_name.clone(), path: game_entry_path};
        let json = serde_json::to_string(&game).unwrap();
        steam_games.push(json);
      }
    }
  }

  let mnt_dir = PathBuf::new().join("/mnt");
  for entry in mnt_dir.read_dir().unwrap() {
    if let Ok (entry) = entry {
      let mnt_steam_dir = entry.path().join("SteamLibrary/steamapps/common");
      if mnt_steam_dir.exists() {
        let mnt_directories = get_directories(mnt_steam_dir);
        for game_entry in mnt_directories {
          let game_entry_name = game_entry.file_name().unwrap().to_str().unwrap().to_string();
          let game_entry_path = game_entry.to_str().unwrap().to_string();
          let game = Game {name: game_entry_name.clone(), path: game_entry_path};
          let json = serde_json::to_string(&game).unwrap();
          steam_games.push(json);
        }
      }
    }
  }

  steam_games.into()
}

fn get_directories(path: PathBuf) -> Vec<PathBuf> {
  let mut directories: Vec<PathBuf> = Vec::new();
  if path.exists() {
    for entry in path.read_dir().unwrap() {
      if let Ok (entry) = entry {
        if entry.path().is_dir() {
          directories.push(entry.path());
        }
      }
    }
  }
  return directories;
}