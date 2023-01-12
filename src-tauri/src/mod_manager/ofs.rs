use crate::mod_manager::game::Game;
use crate::mod_manager::Mod;
use std::path::PathBuf;
use std::process::Command;

pub(crate) struct OFSLogic {
    pub(crate) game: Game,
    pub(crate) mods: Vec<Mod>,
}

impl OFSLogic {
    pub fn exec(&self) {
        let mut mod_paths: Vec<PathBuf> = Vec::new();
        let upper_path: PathBuf = PathBuf::new()
            .join(&self.game.install_path)
            .join(&self.game.path_extension);
        let work_path: PathBuf = PathBuf::new().join(&self.game.work_path);

        for elem in &self.mods {
            mod_paths
                .push(PathBuf::new().join(&self.game.profile_path.join("mods").join(&elem.name)));
        }

        init_overlay_fs(mod_paths, &upper_path, &upper_path, &work_path);
    }
}

fn init_overlay_fs(lower: Vec<PathBuf>, upper: &PathBuf, mount: &PathBuf, workdir: &PathBuf) {
    let mut lower_arg: String = String::from("lowerdir=");
    let upper_arg: String = String::from("upperdir=").to_owned() + upper.to_str().unwrap();
    let work_arg: String = String::from("workdir=").to_owned() + workdir.to_str().unwrap();

    let mut index = 0;
    for path in &lower {
        lower_arg.push_str(path.to_str().unwrap());
        index = index + 1;
        if index < lower.len() {
            lower_arg.push(':');
        }
    }

    Command::new("pkexec")
        .arg("mount")
        .arg("-t")
        .arg("overlay")
        .arg("overlay")
        .arg("-o")
        .arg(&lower_arg)
        .arg("-o")
        .arg(upper_arg)
        .arg("-o")
        .arg(work_arg)
        .arg(mount.to_str().unwrap())
        .spawn()
        .unwrap();
}
