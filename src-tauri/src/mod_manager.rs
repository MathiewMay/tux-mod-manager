use std::{path::PathBuf, path::Path, collections::HashMap, fs, str, process::Command};

use serde::{Deserialize, Serialize};
use dirs;
use compress_tools::*;


extern crate steamlocate;
use steamlocate::{SteamDir, SteamApp};

mod ofs;
pub mod game;

use game::{Game, Executable};

#[derive(Serialize, Deserialize)]
pub struct Mod {
  name: String,
}

#[tauri::command]
pub fn deploy(mods: Vec<Mod>, game: Game) {
  let ofs = ofs::OFSLogic{ 
    game, 
    mods, 
  };
  ofs.exec();
}

#[derive(Serialize, Deserialize)]
pub struct SupportedGame {
  app_id: u32,
  public_name: String,
  known_binaries: Vec<Executable>,
  path_extension: PathBuf
}

#[tauri::command]
pub fn scan_games(supported_games: Vec<SupportedGame>) -> Vec<String> {
  let game_list = match scan_for_steam_games(&supported_games) {
    Some(list) => list,
    None => scan_for_other_games(&supported_games),  //todo
  };
  game_list
}

fn scan_for_steam_games(supported_games: &Vec<SupportedGame>) -> Option<Vec<String>> {
  let mut steam_games: Vec<String> = Vec::new(); //create the list, steam games will be added and then returned at the end of function
  let steam_apps: HashMap<u32, Option<SteamApp>> = find_steam_apps()?;

  let mut supported: HashMap<u32, &SupportedGame> = HashMap::new();  //create a hashmap with steam app ids as the keys, and a SupportedGame struct value
  for game in supported_games {
    supported.insert(
      game.app_id,
      game
    );
  }

  for key in steam_apps.keys() {  //for every steam game found on the system...
    let app = steam_apps[key].as_ref().unwrap(); //Get a reference to a SteamApp struct
    let pathbuf_to_game_config = dirs::config_dir()?.join("tmm/").join(format!("{}.json", app.appid));  // make a path with ~/.config/tmm/(The steamapp id).json for each game's config file
    let path_to_game_config = Path::new(&pathbuf_to_game_config);
    
    if path_to_game_config.exists() {  //if the game already exists, add it's config data to the Steam Games list
      let json = fs::read_to_string(path_to_game_config).unwrap();
      steam_games.push(json);
    } 
    else if !supported.contains_key(&app.appid) {  //if the app id doesn't match a supported game, skip this steam app
      continue
    } 
    else {
      let profile_path = dirs::config_dir()?.join("tmm/profiles/").join(format!("{}", app.appid)); //pathbuf to the tmm config directory from before adding a profiles folder for each game
      let components_count = app.path.components().count();
      let work_path = app.path.components().take(components_count-4).collect::<PathBuf>().join([".tmm_work/", app.appid.to_string().as_str()].join("")); //pathbuf to a .tmm_work folder 4 folders up...
      // from the game's folder so at the same level as the whole SteamLibrary
      let path_extension = supported.get(&app.appid).unwrap().path_extension.clone();  //get a path extension from the path_extension struct value, some games dont have one
      let executables: Vec<Executable> = supported.get(&app.appid).unwrap().known_binaries.clone();
      let game = Game {  //create the game's data that will be both returned in a list of Strings and added to the game's json file
        public_name: app.name.as_ref().unwrap().to_owned(),
        appid: app.appid,
        install_path: app.path.clone(),
        profile_path,
        work_path,
        path_extension,
        executables
      };
      let json = serde_json::to_string(&game).unwrap();
      match make_tmm_game_directories(game) {
        Ok(()) => {},  
        Err(e) => {
          eprintln!("Couldn't create config dirs while working on game '{}'/{}\nError: {}", app.name.as_ref().unwrap(), app.appid, e);
        },
      }
      match fs::write(&pathbuf_to_game_config, &json) {   // write the contents of the game struct to the game's json file inside the config folder
        Ok(()) => {},
        Err(e) => {
          eprintln!("Couldn't write to config file for game '{}'/{}\nError: {}", app.name.as_ref().unwrap(), app.appid, e);
        },
      }
      steam_games.push(json);
    }
  }

  Some(steam_games)
}

fn find_steam_apps() -> Option<HashMap<u32, Option<SteamApp>>> {
  let mut dir = SteamDir::locate()?;

  let apps = dir.apps().clone();
  Some(apps)
}

fn scan_for_other_games(games: &Vec<SupportedGame>) -> Vec<String> {  //scan for games another way, if no steam games were found or couldn't get steam data
  //TODO
  //Scan for games, getting their location and some info
  //put that info into a Game struct, game::Game
  //write that struct as a json string to a file in the tmm config folder
  //create the folders for the game make_tmm_game_directories
  //return a Vec with each game found converted from the Game struct to a string

  let mut executable_list: Vec<PathBuf> = Vec::new(); //create a buffer for the list
  for game in games {  //for each game in the supported games list
    let mut exec_paths: Vec<String> = Vec::new();  //create a buffer for all paths we find
    for exec in &game.known_binaries {  //for each executable name we'll search for
      let name = &exec.binary_path.file_name().unwrap().to_owned();
      let path = match find_game(&name.to_str().unwrap()) {
        Some(path) => path,
        None => continue,
      };

      let exec_depth = exec.binary_path.components();
      exec_paths.push(path);
    }

    let public_name = &game.public_name;
    let app_id = &game.app_id;
    
    //let install_path = exec_path //todo use the exec_depth above to 'cut off' the binary name from the pathbuf to return the game's root folder instead

  }

  Vec::new()
}

fn find_game(game: &str) -> Option<String> {  //scan for one specific game, is a helper function for scan_for_other_games and will later be used to have the user scan for any games the steam search didn't find
  let output = Command::new("/usr/bin/find")
    .arg("/")
    .arg("-type")
    .arg("f")
    .arg("-name")
    .arg(&game)
    .output().expect("failed command");

    let s = match str::from_utf8(&output.stdout) {
      Ok(s) => s,
      Err(_) => return None,
    };

    Some(s.to_owned())
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

pub(crate) fn make_tmm_game_directories(game: Game) -> Result<()> {
  fs::create_dir_all(&game.profile_path)?;
  fs::create_dir_all(&game.work_path)?;
  fs::create_dir_all(&game.profile_path.join("downloads/"))?;
  fs::create_dir_all(&game.profile_path.join("mods/"))?;
  fs::create_dir_all(dirs::config_dir().unwrap().join("tmm"))?;
  Ok(())
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
  let mut source_file =  fs::File::open(file_path).expect("Failed to read file {file_path}");
  let target = game.profile_path.join("mods/").join(file_name);
  uncompress_archive(&mut source_file,&target, Ownership::Ignore).unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_single_game_search() -> Result<()> {
    let game = "Cyberpunk2077.exe";  //I have Cyberpunk installed through GOG on my system, switch this to whatever game you want to test on your system
    let path = match find_game(&game) {
      Some(path) => path,
      None => panic!("Couldn't find cyberpunk or search failed"),
    };
    println!("Got path: {}", path);
    Ok(())
  }
}