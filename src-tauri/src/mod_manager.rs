use std::path::PathBuf;

use dirs;

pub(crate) fn scan_games(){

  let home_dir = dirs::home_dir().unwrap();
  let steam_local_dir = home_dir.join(".local/share/Steam/steamapps/common/");
  let steam_flatpak_dir = home_dir.join(".var/app/com.valvesoftware.Steam/data/Steam/steamapps/common/");
  let steam_dirs = [steam_local_dir, steam_flatpak_dir];
  for dir in steam_dirs {
    let steam_directories = get_directories(dir);
    println!("{:?}", steam_directories);
  }

  let mnt_dir = PathBuf::new().join("/mnt/");
  for entry in mnt_dir.read_dir().unwrap() {
    if let Ok (entry) = entry {
      let mnt_steam_dir = entry.path().join("/SteamLibrary/steamapps/common");
      println!("{}", mnt_steam_dir.display());
      if mnt_steam_dir.exists() {
        
      }
    }
  }
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