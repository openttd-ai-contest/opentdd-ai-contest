use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, value_name = "FILE") hngjvm]
    config: PathBuf,
}

fn main() {
    println!("Hello, world!");
}
