use std::path::PathBuf;

use clap::Parser;

pub mod config;

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
}
