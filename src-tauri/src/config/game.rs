use std::collections::HashMap;

use json::{JsonValue, JsonError};

#[derive(Debug)]
pub struct Game {
    public_name: String,
    appid: u64,
    install_path: String,
    profile_path: String,
    //Basically if the content of mods should be put into
    //another subpath instead of the root install folder
    //e.g. on bethesda titles /Data/
    path_extension: String,
    executables: HashMap<u8, Executable>
}

#[derive(Debug)]
struct Executable {
    name: String,
    use_compatibility: bool,
    binary_path: String,
    startin_path: String,
    output_mod: String,
}

impl Game {
    pub fn from_json(json: JsonValue) -> Result<Game, JsonError> {
        let mut executables = HashMap::new();
        let mut index: u8 = 0;
        for executable in json["executables"].members() {
            executables.insert(
                index,
                Executable {
                    name: executable["name"].as_str().unwrap().to_owned(),
                    use_compatibility: executable["use_compatibility"].as_bool().unwrap().to_owned(),
                    binary_path: executable["binary_path"].as_str().unwrap().to_owned(),
                    startin_path: executable["startin_path"].as_str().unwrap().to_owned(),
                    output_mod: executable["output_mod"].as_str().unwrap().to_owned()
                }
            );
            index = index + 1;
        }
        Ok(Game {
            public_name: json["public_name"].as_str().unwrap().to_owned(),
            appid: json["appid"].as_u64().unwrap(),
            install_path: json["install_path"].as_str().unwrap().to_owned(),
            profile_path: json["profile_path"].as_str().unwrap().to_owned(),
            path_extension: json["profile_path"].as_str().unwrap().to_owned(),
            executables
        })
    }
}