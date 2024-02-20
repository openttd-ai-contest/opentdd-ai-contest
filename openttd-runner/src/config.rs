use std::string::String;
use std::path::PathBuf;
use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    mongodb_url: String,
    openttd_binary: PathBuf
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