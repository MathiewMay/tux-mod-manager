use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub public_name: String,
    pub appid: u32,
    pub install_path: PathBuf,
    pub profile_path: PathBuf,
    pub work_path: PathBuf,
    //Basically if the content of mods should be put into
    //another subpath instead of the root install folder
    //e.g. on bethesda titles /Data/
    pub path_extension: PathBuf,
    pub executables: Vec<Executable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Executable {
    pub name: String,
    pub use_compatibility: bool,
    pub binary_path: PathBuf,
    pub startin_path: PathBuf,
    pub output_mod: String,
}
