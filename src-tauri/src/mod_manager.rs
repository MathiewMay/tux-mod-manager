use std::{collections::HashMap, fs, path::Path, path::PathBuf, process::Command, str};

use compress_tools::*;
use dirs;
use serde::{Deserialize, Serialize};

extern crate steamlocate;
use steamlocate::{SteamApp, SteamDir};

pub mod game;
mod ofs;

use game::{Executable, Game};

#[derive(Serialize, Deserialize)]
pub struct Mod {
    name: String,
}

#[tauri::command]
pub fn deploy(mods: Vec<Mod>, game: Game) {
    let ofs = ofs::OFSLogic { game, mods };
    ofs.exec();
}

#[derive(Serialize, Deserialize)]
pub struct SupportedGame {
    app_id: u32,
    public_name: String,
    known_binaries: Vec<Executable>,
    path_extension: PathBuf,
}

#[tauri::command]
pub fn scan_games(supported_games: Vec<SupportedGame>) -> Vec<String> {
    let game_list = match scan_for_steam_games(&supported_games) {
        Some(list) if list.len() > 0 => list,  // Check if the list the steam search returned is empty, use the slower scan instead
        Some(_) => match scan_for_other_games(&supported_games) {
            Some(list) => list,
            None => {                                       // The functions should only return None if there's a file permission issue or some error
                eprintln!("Problem scanning Steam and other store libraries");
                Vec::new()
            }
        },
        None => match scan_for_other_games(&supported_games) {
            Some(list) => list,
            None => {
                eprintln!("Problem scanning Steam and other store libraries");
                Vec::new()
            }
        },
    };
    game_list
}

#[tauri::command]
pub fn scan_for_single_game(game: SupportedGame) -> Vec<String> {
    //let debug = serde_json::to_string(&game).unwrap();
    //println!("Got game: {}", debug); 

    let supported_game = vec![game];  // make the game passed into a list to reuse the functions expecting a list
    let game_list = match scan_for_steam_games(&supported_game) { // attempt to find a list of games based on the game info
        Some(list) if list.len() > 0 => list,
        Some(_) => match scan_for_other_games(&supported_game) {  // if the list length was 0 attempt the other store search
            Some(list) => list,
            None => {
                eprintln!("Problem scanning Steam and other store libararies");
                Vec::new()
            },
        },
        None => match scan_for_other_games(&supported_game) {  // if there was an issue with the steam search, attempt the other store search
            Some(list) => list,
            None => {
                eprintln!("Problem scanning Steam and other store libararies");
                Vec::new() //if nothing is found, the game_list will be an empty list
            },
        },
    };

    game_list  //  return the game list to the Vue frontend
}

fn scan_for_steam_games(supported_games: &Vec<SupportedGame>) -> Option<Vec<String>> {
    let mut steam_games: Vec<String> = Vec::new(); //create the list, steam games will be added and then returned at the end of function
    let steam_apps: HashMap<u32, Option<SteamApp>> = find_steam_apps()?;

    //create a hashmap with steam app ids as the keys, and a SupportedGame struct value
    
    for game in supported_games {
        if steam_apps.contains_key(&game.app_id) {
            let app = steam_apps[&game.app_id].as_ref().unwrap(); //Get a reference to a SteamApp struct
            let pathbuf_to_game_config = dirs::config_dir()?
                .join("tmm/")
                .join(format!("{}.json", app.appid)); // make a path with ~/.config/tmm/(The steamapp id).json for each game's config file
            let path_to_game_config = Path::new(&pathbuf_to_game_config);

            if path_to_game_config.exists() {
                //if the game already exists, add it's config data to the Steam Games list
                let json = fs::read_to_string(path_to_game_config).unwrap();
                steam_games.push(json);
            } else {
                build_steam_config(app, game, pathbuf_to_game_config, &mut steam_games)?;
            }
        }
    }

    Some(steam_games)
}

fn build_steam_config(app: &SteamApp, game: &SupportedGame, pathbuf_to_game_config: PathBuf, steam_games: &mut Vec<String>) -> Option<()> {
    let profile_path = dirs::config_dir()?
        .join("tmm/profiles/")
        .join(format!("{}", app.appid));
    let components_count = app.path.components().count();
    let work_path = app
        .path
        .components()
        .take(components_count - 4)
        .collect::<PathBuf>()
        .join([".tmm_work/", app.appid.to_string().as_str()].join("")); // The work path is 4 directories up from the game's root, should be the same level as the SteamLibrary
    let path_extension = game.path_extension.clone();
    let executables: Vec<Executable> = game.known_binaries.clone();
    let game_struct = Game {
        //create the game's data that will be both returned in a list of Strings and added to the game's json file
        public_name: app.name.as_ref().unwrap().to_owned(),
        appid: app.appid,
        install_path: app.path.clone(),
        profile_path,
        work_path,
        path_extension,
        executables,
    };
    let json = serde_json::to_string(&game_struct).unwrap();
    match make_tmm_game_directories(game_struct) {
        Ok(()) => {}
        Err(e) => {
            eprintln!(
                "Couldn't create config dirs while working on game '{}'/{}\nError: {}",
                app.name.as_ref().unwrap(),
                app.appid,
                e
            );
        }
    }
    match fs::write(&pathbuf_to_game_config, &json) {
        //  Write the contents of the game struct to the game's json file inside the config folder
        Ok(()) => {}
        Err(e) => {
            eprintln!(
                "Couldn't write to config file for game '{}'/{}\nError: {}",
                app.name.as_ref().unwrap(),
                app.appid,
                e
            );
        }
    }
    steam_games.push(json);
    Some(())
}

fn find_steam_apps() -> Option<HashMap<u32, Option<SteamApp>>> {
    let mut dir = SteamDir::locate()?;

    let apps = dir.apps().clone();
    Some(apps)
}

fn scan_for_other_games(games: &Vec<SupportedGame>) -> Option<Vec<String>> {
    //scan for games another way, if no steam games were found or couldn't get steam data

    // TODO
    // * Improve the search using find to search common folders first leaving / root as a last resort
    // will hopefully improve speed. Example: /media /mnt /run/media etc...
    // * Another idea would to rebase the search function to instead search for a single game but multithreaded,
    // so instead of sending a list to rust for it to go through one at a time, search for several games at once
    // I will need to keep reading the rust book until the section on multithreading

    let mut game_list: Vec<String> = Vec::new(); //  create a buffer for the list
    for game in games {
        //  for each game in the supported games list
        let game_config_pathbuf = dirs::config_dir()?
            .join("tmm/")
            .join(format!("{}.json", game.app_id));
        let game_config_path = Path::new(&game_config_pathbuf);

        if game_config_path.exists() {  // if the game has a config file already, read its data and return that immediately, otherwise generate the data below
            let json = fs::read_to_string(game_config_path).unwrap();
            game_list.push(json);
            continue;
        }

        let mut game_paths: Vec<PathBuf> = Vec::new(); //  create a buffer for all paths we find
        for exec in &game.known_binaries {
            //  for each executable name we'll search for
            let name = exec.binary_path.file_name().unwrap().to_owned(); //  we search for the game using only the file name, ie Cyberpunk2077.exe,
            let exec_path = match find_game(&name.to_str().unwrap()) {
                //because the supported games json has some binary files inside a couple different folders from the game's root folder
                Some(path) => path,
                None => continue,
            };

            for line in exec_path.lines() {
                //  if the find command finds multiple files they'll be on seperate lines, check each one is the 'real' one
                if line.contains(&exec.binary_path.to_str().unwrap()) {
                    //  Check for the folder layout as listed in binary path, should prevent any rare circumstances where there's an incorrect .exe stored somewhere
                    let exec_depth = &exec.binary_path.components().count(); //  Check how many folders the exec is in from the game's root folder
                    let exec_pathbuf = PathBuf::from(line.trim());
                    let exec_path_count = exec_pathbuf.components().count(); //  Convert the path string into a pathbuf to quickly count the folders
                    let game_root_path: PathBuf = exec_pathbuf
                        .components()
                        .take(exec_path_count - exec_depth)
                        .collect(); //  This should trim down the end of the full path to direct to the game's root folder
                    game_paths.push(game_root_path);
                }
            }
        }

        if game_paths.len() == 0 {  // Ran into a panic on let install_path, check if nothing was found and skip to the next supported game
            continue
        }

        // create all the different data to be returned for the game
        let public_name = game.public_name.clone();
        let appid = game.app_id;
        let install_path = game_paths[0].clone(); //  We'll just assume the first path found is the correct one
        let profile_path = dirs::config_dir()?
            .join("tmm/profiles/")
            .join(format!("{}", appid)); //  Pathbuf to the tmm config directory from before adding a profiles folder for each game
        let work_path = dirs::config_dir()?
            .join("tmm/non_steam_mods")
            .join(format!("{}", appid));
        let path_extension = game.path_extension.clone();
        let executables = game.known_binaries.clone();

        let game_data = Game {
            //  Collect all the game data together for json serde
            public_name,
            appid,
            install_path,
            profile_path,
            work_path,
            path_extension,
            executables,
        };

        let json_data = serde_json::to_string(&game_data).unwrap();

        match make_tmm_game_directories(game_data) {
            Ok(()) => {}
            Err(e) => {
                eprintln!(
                    "Couldn't create config dirs while working on game '{}'/{}\nError: {}",
                    game.public_name, appid, e
                );
            }
        }

        match fs::write(&game_config_path, &json_data) {
            Ok(()) => {}
            Err(e) => {
                eprintln!(
                    "Couldn't create config file for game '{}'/{}\nError: {}",
                    game.public_name, appid, e
                )
            }
        }

        game_list.push(json_data)
    }

    Some(game_list)
}

fn find_game(game: &str) -> Option<String> {
    //  Scan for one specific game, is a helper function for scan_for_other_games and will later be used to have the user scan for any games the steam search didn't find
    let output = match Command::new("find") //  Run the find command, currently searches starting from root which is probably slow on some systems, will later limit the search to some more common folders
        .arg("/")
        .arg("-type")
        .arg("f")
        .arg("-name")
        .arg(&game)
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to run find command {}", e);
            return None;
        }
    }; //  Returns an Output struct type, the stdout is a Vec<u8> that needs to be converted into a string

    let s = match str::from_utf8(&output.stdout) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to convert find output {}", e);
            return None;
        }
    };

    Some(s.to_owned())
}

#[tauri::command]
pub fn get_mods(game: Game) -> Vec<String> {
    let mut mods: Vec<String> = Vec::new();  
    for path in get_directories(&game.profile_path.join("mods")) {  //for each mod found in the mod folder
        let name = path.file_name().unwrap().to_str().unwrap().to_string();  //take the file name, and add it to the list
        let mod_struct: Mod = Mod { name };
        let mod_json: String = serde_json::to_string(&mod_struct).unwrap();
        mods.push(mod_json);
    }
    mods
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
            if let Ok(entry) = entry {
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
    let mut source_file = fs::File::open(file_path).expect("Failed to read file {file_path}");
    let target = game.profile_path.join("mods/").join(file_name);
    uncompress_archive(&mut source_file, &target, Ownership::Ignore).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn return_test_list() -> Vec<SupportedGame> {
        let test_game_list: Vec<SupportedGame> = vec![
            SupportedGame {
                app_id: 1091500,
                public_name: String::from("Cyberpunk 2077"),
                known_binaries: vec![Executable {
                    name: String::from("Cyberpunk2077"),
                    use_compatibility: true,
                    binary_path: PathBuf::from("/bin/x64/Cyberpunk2077.exe"),
                    startin_path: PathBuf::from(""),
                    output_mod: String::from("overwrite"),
                }],
                path_extension: PathBuf::from(""),
            },
            SupportedGame {
                app_id: 22380,
                public_name: String::from("Fallout: New Vegas"),
                known_binaries: vec![ Executable {
                    name: String::from("FalloutNV"),
                    use_compatibility: true,
                    binary_path: PathBuf::from("/FalloutNV.exe"),
                    startin_path: PathBuf::from(""),
                    output_mod: String::from("overwrite"),
                }],
                path_extension: PathBuf::from("Data/"),
            }
        ];
        test_game_list
    }
    

    #[test]
    fn test_find_game() -> Result<()> {
        let game = "Cyberpunk2077.exe"; //I have Cyberpunk installed through GOG on my system, switch this to whatever game you want to test on your system
        let path = match find_game(&game) {
            Some(path) => path,
            None => panic!("Couldn't find cyberpunk or search failed"),
        };
        println!("Got path: {}", path);
        Ok(())
    }

    #[test]
    fn test_other_game_search() -> Result<()> {
        let test_game_list = return_test_list();
        let results = match scan_for_other_games(&test_game_list) {
            Some(result) => result,
            None => panic!("Search failed"),
        };

        println!("Got result: {:?}", results);

        Ok(())
    }

    #[test]
    fn test_steam_game_search() -> Result<()> {
        let test_game_list = return_test_list();
        let results = match scan_for_steam_games(&test_game_list) {
            Some(list) if list.len() > 0 => list,
            Some(_) => { 
                println!("list appears to be empty");
                Vec::new()
            },
            None => { 
                println!("there was an issue with the search"); 
                Vec::new() 
            },
        };

        println!("Got result: {:?}", results);
        Ok(())

    }

    #[test]
    fn test_steam_app_search() -> Result<()> {
        let map = match find_steam_apps() {
            Some(map) => map,
            None => { 
                println!("there was an issue with the search"); 
                HashMap::new()
            },
        };

        if map.len() == 0 {
            println!("hashmap is empty, no apps found");
            return Ok(())
        }

        let mut dir = SteamDir::locate().unwrap();

        let libraries = dir.libraryfolders();

        println!("Libraries: {:?}", libraries);  

        //println!("new apps: {:?}", dir.apps());  // For some reason the apps function isn't returning the apps from the second steam library on my system
        //println!("Got result: {:#?}", map);      // only the first one in the home folder, it found both libraries with the libraryfolders() function
        Ok(())

    }

    #[test]
    fn test_single_game_search() -> Result<()> {
        let game = SupportedGame {
            app_id: 1091500,
            public_name: String::from("Cyberpunk 2077"),
            known_binaries: vec![Executable {
                name: String::from("Cyberpunk2077"),
                use_compatibility: true,
                binary_path: PathBuf::from("/bin/x64/Cyberpunk2077.exe"),
                startin_path: PathBuf::from(""),
                output_mod: String::from("overwrite"),
            }],
            path_extension: PathBuf::from(""),
        };

        let result = scan_for_single_game(game);
        println!("Got result: {:?}", result);
        
        Ok(())
    }
}
