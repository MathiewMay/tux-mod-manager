use crate::mod_manager::{Game, Mod};
use std::process::Command;
use std::path::PathBuf;

pub(crate) struct OFSLogic {
  pub(crate) game: Game,
  pub(crate) mods: Vec<Mod>,
}

impl OFSLogic {
  pub fn exec(&self) {
    //before game here create the overlayfs
    let mut mod_paths: Vec<PathBuf> = Vec::new();
    let upper_path: PathBuf = PathBuf::new().join(&self.game.path);
    let home_dir = dirs::home_dir().unwrap();
    let mount_path: PathBuf = home_dir.clone().join(".config/tmm_stage/mount/").join(&self.game.name);
    let tmp_path: PathBuf = home_dir.clone().join(&self.game.work);
    for mod_ in &self.mods {
      mod_paths.push(PathBuf::new().join(&mod_.path));
    }
    //make sure to create the directories before running init overlay fs
    init_overlay_fs(mod_paths, upper_path, mount_path, tmp_path);
    //run game here
  }
}

fn init_overlay_fs(lower: Vec<PathBuf>, upper: PathBuf, mount: PathBuf, workdir: PathBuf) {
  println!("Started a overlay fs init");
  let mut mod_dirs: String = String::from("lowerdir=");
  let game_dir: String = String::from("upperdir=").to_owned()+upper.to_str().unwrap();
  let tmp_dir: String = String::from("workdir=").to_owned()+workdir.to_str().unwrap();
  let mut index = 0;
  for path in &lower {
    mod_dirs.push_str(path.to_str().unwrap());
    index = index + 1;
    if index < lower.len() {
      mod_dirs.push(':');
    }
  }
  println!("mount -t overlay overlay -o {} -o {} -o {} {}", mod_dirs, game_dir, tmp_dir, mount.to_str().unwrap());
  Command::new("pkexec")
    .arg("mount")
    .arg("-t")
    .arg("overlay")
    .arg("overlay")
    .arg("-o")
    .arg(&mod_dirs)
    .arg("-o")
    .arg(game_dir)
    .arg("-o")
    .arg(tmp_dir)
    .arg(mount.to_str().unwrap())
    .spawn()
    .unwrap();
}