use std::path::PathBuf;

use dirs;

#[tauri::command]
pub(crate) fn scan_games() -> Vec<PathBuf> {
  let mut steam_games: Vec<PathBuf> = Vec::new();
  let home_dir = dirs::home_dir().unwrap();
  let steam_local_dir = home_dir.join(".local/share/Steam/steamapps/common/");
  let steam_flatpak_dir = home_dir.join(".var/app/com.valvesoftware.Steam/data/Steam/steamapps/common/");
  let steam_dirs = [steam_local_dir, steam_flatpak_dir];
  for dir in steam_dirs {
    if dir.exists() {
      let steam_directories = get_directories(dir);
      for game in steam_directories {
        steam_games.push(game);
      }
    }
  }

  let mnt_dir = PathBuf::new().join("/mnt");
  for entry in mnt_dir.read_dir().unwrap() {
    if let Ok (entry) = entry {
      let mnt_steam_dir = entry.path().join("SteamLibrary/steamapps/common");
      if mnt_steam_dir.exists() {
        let mnt_directories = get_directories(mnt_steam_dir);
        for game in mnt_directories {
          steam_games.push(game);
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