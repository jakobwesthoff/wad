use anyhow::Result;
use clap::{CommandFactory, FromArgMatches};
use std::fmt;

use super::{Command as CommandTrait, Commands};
use crate::{
    config::Config,
    utils::{formatting, selection::SelectionMenu},
    watson::WatsonClient,
};

/// Metadata for a command extracted from clap introspection
#[derive(Clone)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
}

impl fmt::Display for CommandMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.description)
    }
}

/// Discovers all available commands from the clap Command structure
fn get_all_commands() -> Vec<CommandMetadata> {
    let cmd = Commands::command();
    let mut commands = Vec::new();

    for subcommand in cmd.get_subcommands() {
        let name = subcommand.get_name().to_string();
        let description = subcommand
            .get_about()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "No description available".to_string());

        commands.push(CommandMetadata { name, description });
    }

    commands
}

/// Show a command selection menu for all commands and execute the selected one.
pub fn show_command_selection_menu(
    watson_client: &WatsonClient,
    config: &Config,
    verbose: bool,
) -> Result<()> {
    println!(
        "{}",
        formatting::header_text(
            "Watson Dashboard - Enhanced querying and overview for Watson time tracker"
        )
    );
    println!();

    // Get available commands dynamically from clap
    let command_options = get_all_commands();

    let selection =
        SelectionMenu::from_display_items("Select a command to run:", command_options).prompt();

    match selection {
        Ok(command_metadata) => {
            // Use clap's parsing to convert command name back to Commands enum variant for execution
            let program_name = std::env::args().next().unwrap_or_else(|| "wad".to_string());
            let args = vec![program_name, command_metadata.name.clone()];
            let matches = Commands::command().try_get_matches_from(args)?;
            let command = Commands::from_arg_matches(&matches)?;
            command.run(watson_client, config, verbose)
        }
        Err(_) => {
            println!("{}", formatting::info_text("Selection cancelled"));
            Ok(())
        }
    }
}
