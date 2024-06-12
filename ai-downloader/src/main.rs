use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, value_name = "OUTPUT")]
    output_directory: PathBuf,
    #[arg(short, value_name = "API", default_value = "https://bananas-api.openttd.org")]
    api_base_path: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Downloading all available AIs from BaNaNaS API {} to directory {}",
             cli.api_base_path,
             cli.output_directory.display());
}
