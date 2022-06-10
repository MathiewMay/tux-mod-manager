use std::{path::PathBuf, path::Path, fs};

use serde::{Deserialize, Serialize};
use dirs;
use compress_tools::*;


extern crate steamlocate;
use steamlocate::SteamDir;

mod ofs;
pub mod game;

use game::{Game, Executable};

// #[derive(Serialize, Deserialize)]
// pub struct Game {
//   appid: u32,
//   name: String,
//   path: PathBuf,
//   stage_path: PathBuf,
//   work_path: PathBuf,
// }

#[derive(Serialize, Deserialize)]
pub struct Mod {
  name: String,
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

  let known_path_extensions_json = dirs::config_dir().unwrap().join("tmm/known_path_extensions.json");
  let path_extension_contents = fs::read_to_string(known_path_extensions_json).unwrap();
  let known_path_extensions: Vec<(u32, PathBuf)> = serde_json::from_str(path_extension_contents.as_str()).unwrap();

  let supported_games_json = dirs::config_dir().unwrap().join("tmm/supported_games.json");
  let supported_games_contents = fs::read_to_string(supported_games_json).unwrap();
  let supported_games: Vec<u32> = serde_json::from_str(supported_games_contents.as_str()).unwrap();

  // println!("Known Path Extensions: {:?}", known_path_extensions);
  for key in steam_apps.keys() {
    let app = steam_apps[key].as_ref().unwrap();

    let pathbuf_to_game_config = dirs::config_dir().unwrap().join("tmm/").join(format!("{}.json", app.appid));
    let path_to_game_config = Path::new(&pathbuf_to_game_config);
    let already_found = path_to_game_config.exists();

    if already_found {
      // println!("There already exists a config for game: '{}'", app.name.as_ref().unwrap());
      let json = fs::read_to_string(path_to_game_config).unwrap();
      steam_games.push(json);
    } else if !supported_games.contains(&app.appid) {
      // println!("Game: {} not currently supported.", app.name.as_ref().unwrap());
    } else {
      let profile_path = dirs::config_dir().unwrap().join("tmm/profiles/").join(format!("{}", app.appid));
      let components_count = app.path.to_path_buf().components().count();
      let work_path = app.path.to_path_buf().components().take(components_count-4).collect::<PathBuf>().join([".tmm_work/", app.appid.to_string().as_str()].join(""));
      println!("Game work_directory: {}", &work_path.to_str().unwrap());
      let mut path_extension: PathBuf = PathBuf::new();
      for pair in &known_path_extensions {
        let (id, extension) = pair;
        if id == &app.appid {
          path_extension = extension.to_owned();
          break;
        }
      }
      let executables: Vec<Executable> = Vec::new();
      let game = Game {
        public_name: app.name.as_ref().unwrap().to_owned(),
        appid: app.appid,
        install_path: app.path.to_path_buf(),
        profile_path,
        work_path,
        path_extension,
        executables
      };

      let json = serde_json::to_string(&game).unwrap();
      let mut app_config_path = dirs::config_dir().unwrap().join("tmm").join(format!("{}", app.appid));
      app_config_path.set_extension("json");
      match fs::create_dir_all(dirs::config_dir().unwrap().join("tmm")) {
        Ok(()) => {},
        Err(e) => {
          eprintln!("Couldn't create config dir while working on game '{}'/{}\nError: {}", app.name.as_ref().unwrap(), app.appid, e);
        }
      }
      match fs::write(&app_config_path, &json) {
        Ok(()) => {},
        Err(e) => {
          eprintln!("Couldn't write to config file for game '{}'/{}\nError: {}", app.name.as_ref().unwrap(), app.appid, e);
        }
      }
      make_tmm_game_directories(game);
      steam_games.push(json);
    }
    // let app = steam_apps[key].as_ref().unwrap();
    // let stage_path = dirs::home_dir().unwrap().join([".config/tmm_stage/games/", app.appid.to_string().as_str()].join(""));
    // let components_count = app.path.to_path_buf().components().count();
    // let work_path = app.path.to_path_buf().components().take(components_count-4).collect::<PathBuf>().join([".tmm_work/", app.appid.to_string().as_str()].join(""));
    // let game = Game{ appid: app.appid, name: app.name.as_ref().unwrap().to_string(), path: app.path.to_path_buf(), stage_path, work_path };
    // let json = serde_json::to_string(&game).unwrap();
    // make_tmm_game_directories(game);
    // steam_games.push(json);
  }
  steam_games.into()
}

#[tauri::command]
pub fn get_mods(game: Game) -> Vec<String>{
  let mut mods: Vec<String> = Vec::new();
  for path in get_directories(&game.profile_path.join("mods")) {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let mod_struct: Mod = Mod { name };
    let mod_json: String = serde_json::to_string(&mod_struct).unwrap();
    mods.push(mod_json);
  }
  mods.into()
}

#[tauri::command]
pub fn remove_mod(mod_struct: Mod, game: Game) {
  let mod_dir = game.profile_path.join("mods").join(mod_struct.name);
  fs::remove_dir_all(mod_dir).unwrap();
}

pub(crate) fn make_tmm_game_directories(game: Game) {
  fs::create_dir_all(&game.profile_path).unwrap();
  fs::create_dir_all(&game.work_path).unwrap();
  fs::create_dir_all(&game.profile_path.join("downloads/")).unwrap();
  fs::create_dir_all(&game.profile_path.join("mods/")).unwrap();
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
pub async fn uncompress(file_path: String, file_name: String, game: Game) {
  let mut source_file = fs::File::open(file_path).unwrap();
  let target = game.profile_path.join("mods/").join(file_name);
  uncompress_archive(&mut source_file,&target, Ownership::Ignore).unwrap();
}