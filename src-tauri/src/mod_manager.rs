use std::{path::PathBuf, fs};
use serde::{Deserialize, Serialize};
use dirs;

mod ofs;

#[derive(Serialize, Deserialize)]
pub struct Game {
  name: String,
  path: String,
  work: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mod {
  name: String,
  path: String,
}

#[tauri::command]
pub(crate) fn deploy(mods: Vec<String>, game: String) {
  let mut mods_vec: Vec<Mod> = Vec::new();
  let game_struct: Game = serde_json::from_str(game.as_str()).unwrap();
  for string in mods {
    let json: Mod = serde_json::from_str(string.as_str()).unwrap();
    mods_vec.push(json);
  }

  let ofs = ofs::OFSLogic{ game: game_struct, mods: mods_vec };
  ofs.exec();
}

#[tauri::command]
pub(crate) fn scan_games() -> Vec<String> {
  let mut steam_games: Vec<String> = Vec::new();
  let home_dir = dirs::home_dir().unwrap();
  let steam_local_dir = home_dir.join(".local/share/Steam/steamapps/common/");
  let steam_flatpak_dir = home_dir.join(".var/app/com.valvesoftware.Steam/data/Steam/steamapps/common/");
  let mut steam_dirs: Vec<PathBuf> = vec!(steam_local_dir, steam_flatpak_dir);

  let mnt_dir = PathBuf::new().join("/mnt");
  for entry in mnt_dir.read_dir().unwrap() {
    if let Ok (entry) = entry {
      make_mnt_stage_directory(entry.path());
      let mnt_steam_dir = entry.path().join("SteamLibrary/steamapps/common");
      if mnt_steam_dir.exists() {
        steam_dirs.push(mnt_steam_dir);
      }
    }
  }

  for dir in steam_dirs {
    if dir.exists() {
      let dir_string = dir.to_str().unwrap().to_string();
      let steam_directories = get_directories(&dir);
      for game_entry in steam_directories {
        let game_entry_name = game_entry.file_name().unwrap().to_str().unwrap().to_string();
        let game_entry_path = game_entry.to_str().unwrap().to_string();
        let mut work_path = home_dir.join(".config/tmm_stage/work/");
        if dir_string.starts_with("/mnt/") {
          let work_prefix = dir_string.replace("SteamLibrary/steamapps/common", "");
          work_path = PathBuf::new().join(work_prefix+".config/tmm_stage/work/");
        }
        make_work_directory(game_entry_name.clone(), work_path.clone());
        let game = Game {name: game_entry_name.clone(), path: game_entry_path, work: work_path.join(game_entry_name).to_str().unwrap().to_string()};
        let json = serde_json::to_string(&game).unwrap();
        steam_games.push(json);
      }
    }
  }
  steam_games.into()
}
#[tauri::command]
pub(crate) fn make_stage_directory() {
  let home_dir = dirs::home_dir().unwrap();
  let stage_dir = home_dir.join(".config/tmm_stage/");
  if !stage_dir.exists() {
    let games_dir = stage_dir.join("games/");
    fs::create_dir(stage_dir).unwrap();
    if !games_dir.exists() {
      fs::create_dir(games_dir).unwrap();
    }
  }
}
#[tauri::command]
pub(crate) fn make_game_stage_directory(game_name: String) {
  let stage_dir = dirs::home_dir().unwrap().join(".config/tmm_stage/games/");
  let game_dir = stage_dir.join(game_name+"/");
  if !game_dir.exists() {
    let mods_dir = game_dir.join("mods/");
    fs::create_dir(game_dir).unwrap();
    if !mods_dir.exists() {
      fs::create_dir(mods_dir).unwrap();
    }
  }
}
#[tauri::command]
pub(crate) fn get_mods_name(game_name: String) -> Vec<String>{
  let join_path = ".config/tmm_stage/games/".to_owned()+game_name.as_str()+"/mods/";
  let game_dir = dirs::home_dir().unwrap().join(join_path);
  let mods_directories = get_directories(&game_dir);
  let mut mods: Vec<String> = Vec::new();
  for path in mods_directories {
    let mod_entry_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let mod_entry_path = path.to_str().unwrap().to_string();
    let mod_ = Mod {name: mod_entry_name.clone(), path: mod_entry_path};
    let json = serde_json::to_string(&mod_).unwrap();
    mods.push(json);
  }
  mods.into()
}
fn make_mnt_stage_directory(mnt: PathBuf) {
  let config_dir = mnt.join(".config/");
  let stage_dir = config_dir.join("tmm_stage/");
  if !config_dir.exists() {
    fs::create_dir(config_dir.clone()).unwrap();
  }
  if !stage_dir.exists() {
    fs::create_dir(stage_dir).unwrap();
  }
}
fn make_work_directory(game_name: String, prefix: PathBuf) {
  if !prefix.exists() {
    fs::create_dir(prefix.clone()).unwrap();
  }
  let work_dir = prefix.join(game_name);
  if !work_dir.exists() {
    fs::create_dir(work_dir).unwrap();
  }
}
fn get_directories(path: &PathBuf) -> Vec<PathBuf> {
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