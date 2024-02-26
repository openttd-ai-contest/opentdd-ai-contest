use std::path::PathBuf;

use clap::Parser;

use crate::runner::run_game;

pub mod config;
pub mod dao;
pub mod runner;
pub mod help_parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, value_name = "FILE")]
    config: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    println!("Reading config file: {}", cli.config.display());
    let config = crate::config::parse(cli.config);
    println!("Parsed config: {}", config);

    let dao = crate::dao::Dao::new(&config.dao.mongodb_uri, &config.dao.database);
    let result = run_game(config.openttd_directory.as_path(), config.player_names);
    dao.insert_run_result(result);
}
