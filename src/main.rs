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

    match cli.command {
        Some(command) => command.run(cli.verbose),
        None => {
            println!("Watson Dashboard - Enhanced querying and overview for Watson time tracker");
            println!("Use --help for more information");
            Ok(())
        }
    }
}
