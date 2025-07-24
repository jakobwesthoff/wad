use anyhow::Result;
use clap::Parser;

mod commands;
mod config;
mod editor;
mod utils;
mod wad_data;
mod watson;

use commands::{Command, Commands, discovery};
use config::Config;
use utils::formatting;
use watson::WatsonClient;

#[derive(Parser)]
#[command(name = "wad")]
#[command(about = "Watson Dashboard - Enhanced querying and overview for Watson time tracker")]
#[command(version)]
struct Cli {
    #[arg(short, long, global = true, help = "Enable verbose output")]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check Watson availability before executing any commands
    let watson_client = WatsonClient::new();
    if !watson_client.is_usable() {
        eprintln!(
            "{}",
            formatting::error_text("Error: Watson CLI is not available or not working properly.")
        );
        eprintln!(
            "{}",
            formatting::error_text(
                "Please make sure Watson is installed and accessible in your PATH."
            )
        );
        std::process::exit(1);
    }

    // Open configuration
    let config = Config::open().unwrap_or_else(|e| {
        if cli.verbose {
            eprintln!("{}: {}", formatting::warning_text("Config warning"), e);
        }
        Config::default()
    });

    // Print Watson info if verbose
    if cli.verbose {
        if let Ok(version) = watson_client.get_version() {
            println!(
                "{}: {}.{}.{}",
                formatting::info_text("Watson version"),
                version.major,
                version.minor,
                version.patch
            );
        }

        if let Ok(path) = watson_client.get_path() {
            println!("{}: {}", formatting::info_text("Watson path"), path);
        }
    }

    match cli.command {
        Some(command) => command.run(&watson_client, &config, cli.verbose),
        None => discovery::show_command_selection_menu(&watson_client, &config, cli.verbose),
    }
}
