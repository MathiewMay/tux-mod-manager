use std::{path::PathBuf, fs};

use serde::{Deserialize, Serialize};
use dirs;
use compress_tools::*;


extern crate steamlocate;
use steamlocate::SteamDir;

mod ofs;

#[derive(Serialize, Deserialize)]
pub struct Game {
  appid: u32,
  name: String,
  path: PathBuf,
  stage_path: PathBuf,
  work_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct Mod {
  name: String,
  path: String,
}

#[tauri::command]
pub fn deploy(mods: Vec<Mod>, game: Game) {
  let ofs = ofs::OFSLogic{ game, mods };
  ofs.exec();
}

#[tauri::command]
pub fn scan_games() -> Vec<String> {
  let mut steam_games: Vec<String> = Vec::new();
  let steam_apps = SteamDir::locate().unwrap().apps().clone();
  for key in steam_apps.keys() {
    let app = steam_apps[key].as_ref().unwrap();
    let stage_path = dirs::home_dir().unwrap().join([".config/tmm_stage/games/", app.appid.to_string().as_str()].join(""));
    let work_path = app.path.to_path_buf().components().take(3).collect::<PathBuf>().join([".config/tmm_stage/work/", app.appid.to_string().as_str()].join(""));
    let game = Game{ appid: app.appid, name: app.name.as_ref().unwrap().to_string(), path: app.path.to_path_buf(), stage_path, work_path };
    let json = serde_json::to_string(&game).unwrap();
    make_tmm_game_directories(game);
    steam_games.push(json);
  }
  steam_games.into()
}

#[tauri::command]
pub fn get_mods(game: Game) -> Vec<String>{
  let mut mods: Vec<String> = Vec::new();
  for path in get_directories(&game.stage_path) {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let mod_struct: Mod = Mod { name, path: path.to_str().unwrap().to_string() };
    let mod_json: String = serde_json::to_string(&mod_struct).unwrap();
    mods.push(mod_json);
  }
  mods.into()
}

#[tauri::command]
pub fn remove_mod(mod_struct: Mod) {
  fs::remove_dir_all(mod_struct.path).unwrap();
}

pub(crate) fn make_tmm_game_directories(game: Game) {
  fs::create_dir_all(game.work_path).unwrap();
  fs::create_dir_all(game.stage_path).unwrap();
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

#[tauri::command]
pub async fn uncompress(file_path: String, target_path: String) {
  let mut source_file = fs::File::open(file_path).unwrap();
  let target = PathBuf::new().join(&target_path);
  uncompress_archive(&mut source_file,&target, Ownership::Ignore).unwrap();
}