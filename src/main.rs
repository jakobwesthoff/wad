use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "wad")]
#[command(about = "Watson Dashboard - Enhanced querying and overview for Watson time tracker")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    // Commands will be added here
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            println!("Watson Dashboard - Enhanced querying and overview for Watson time tracker");
            println!("Use --help for more information");
        }
    }

    Ok(())
}
