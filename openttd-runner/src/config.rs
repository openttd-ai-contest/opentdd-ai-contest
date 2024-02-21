use std::string::String;
use std::path::PathBuf;
use std::vec::Vec;
use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mongodb_url: String,
    pub openttd_directory: PathBuf,
    pub player_names: Vec<String>
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({:#?})", self)
    }
}

pub fn parse(path: PathBuf) -> Config {
    let config_yaml = std::fs::read_to_string(path.as_path())
        .expect(&std::format!("Unable to read config file: {}", path.display()));
    return serde_yaml::from_str(&config_yaml).unwrap();
}