use anyhow::{Result, anyhow};
use clap::CommandFactory;
use inquire::Select;
use std::fmt;

use crate::{commands::Command, utils::formatting, watson::WatsonClient};

use super::{Commands, WorktimeTodayCommand};

/// Metadata for a command extracted from clap introspection
#[derive(Debug, Clone)]
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

/// Creates a command instance by name with default arguments
fn create_command_by_name(name: &str) -> Result<Commands> {
    match name {
        "worktime:today" => Ok(Commands::WorktimeToday(WorktimeTodayCommand::new())),
        _ => Err(anyhow!("Unknown command: {}", name)),
    }
}

/// Execute the given command by name
fn execute_command(command_name: &str, watson_client: &WatsonClient, verbose: bool) -> Result<()> {
    if verbose {
        println!(
            "{}",
            formatting::verbose_text(&format!("Executing command: {}", command_name))
        );
    }

    // Create and execute the command dynamically
    let command = create_command_by_name(command_name)?;
    command.run(watson_client, verbose)
}

/// Show a command selection menu for all commands and execute the selected one.
pub fn show_command_selection_menu(watson_client: &WatsonClient, verbose: bool) -> Result<()> {
    println!(
        "{}",
        formatting::header_text(
            "Watson Dashboard - Enhanced querying and overview for Watson time tracker"
        )
    );
    println!();

    // Get available commands dynamically from clap
    let command_options = get_all_commands();

    let selection = Select::new("Select a command to run:", command_options)
        .with_help_message("Use arrow keys to navigate, Enter to select, Esc to cancel")
        .prompt();

    match selection {
        Ok(command_metadata) => execute_command(&command_metadata.name, watson_client, verbose),
        Err(_) => {
            println!("{}", formatting::info_text("Selection cancelled"));
            Ok(())
        }
    }
}
