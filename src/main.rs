use anyhow::Result;
use clap::Parser;

mod commands;
mod watson;

use commands::{Command, Commands};
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
        eprintln!("Error: Watson CLI is not available or not working properly.");
        eprintln!("Please make sure Watson is installed and accessible in your PATH.");
        std::process::exit(1);
    }

    // Print Watson info if verbose
    if cli.verbose {
        if let Ok(version) = watson_client.get_version() {
            println!(
                "Watson version: {}.{}.{}",
                version.major, version.minor, version.patch
            );
        }

        if let Ok(path) = watson_client.get_path() {
            println!("Watson path: {}", path);
        }
    }

    match cli.command {
        Some(command) => command.run(cli.verbose),
        None => {
            println!("Watson Dashboard - Enhanced querying and overview for Watson time tracker");
            println!("Use --help for more information");
            Ok(())
        }
    }
}
