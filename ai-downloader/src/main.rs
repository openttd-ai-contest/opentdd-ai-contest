use std::path::PathBuf;
use clap::Parser;
use bananas_api::apis::configuration::Configuration;
use bananas_api::apis::discover_api;
use bananas_api::models::ContentType;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, value_name = "OUTPUT")]
    output_directory: PathBuf,
    #[arg(short, value_name = "API", default_value = "https://bananas-api.openttd.org")]
    api_base_path: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("Downloading all available AIs from BaNaNaS API {} to directory {}",
             cli.api_base_path,
             cli.output_directory.display());

    let api = Configuration {
        base_path: cli.api_base_path,
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None,
    };

    let ai_packages = discover_api::package_content_type_get(
        &api,
        ContentType::Ai,
        None).await.expect("Failed to list AI packages");

    for package in ai_packages {
        println!("Found package: {package:?}");
    }
}
