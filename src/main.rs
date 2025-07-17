use anyhow::Result;
use clap::Parser;

mod commands;
use commands::{Command, Commands};

#[derive(Parser)]
#[command(name = "wad")]
#[command(about = "Watson Dashboard - Enhanced querying and overview for Watson time tracker")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => command.run(),
        None => {
            println!("Watson Dashboard - Enhanced querying and overview for Watson time tracker");
            println!("Use --help for more information");
            Ok(())
        }
    }
}
