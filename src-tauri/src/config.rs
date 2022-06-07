use std::fs;
use dirs::config_dir;

mod game;

#[derive(Debug)]
enum VerificationError {
    Syntax,
    Semantics
}

#[derive(Debug)]
pub struct Config {
    max_download_workers: u8,
}

type VerificationResult = Result<Config, VerificationError>;

pub fn load_global_conf() -> Config {
    Config {
        max_download_workers: 8
    }
}

pub fn load_game_conf(appid: u64) -> game::Game {
    let mut path = config_dir().unwrap();
    path.push("tmm_stage");
    path.push(format!("{}", appid));
    path.set_extension("json");
    let contents = fs::read_to_string(path)
        .expect("Game config file could not be read!");
    let json = json::parse(contents.as_str()).unwrap();
    game::Game::from_json(json).unwrap()
}