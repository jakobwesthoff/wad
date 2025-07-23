use super::Command;
use crate::config::Config;
use crate::utils::formatting;
use crate::watson::WatsonClient;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct ConfigCommand {
    #[command(subcommand)]
    action: ConfigAction,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Print the configuration directory path
    Path,
    /// Get a specific configuration value
    Get {
        /// Configuration key to retrieve
        key: String,
    },
    /// Set a specific configuration value
    Set {
        /// Configuration key to set
        key: String,
        /// Value to set
        value: String,
    },
    /// List all configuration values
    List,
}

impl Command for ConfigCommand {
    fn run(&self, _watson_client: &WatsonClient, config: &Config, verbose: bool) -> Result<()> {
        match &self.action {
            ConfigAction::Path => {
                let config_dir = Config::config_dir()
                    .map_err(|e| anyhow::anyhow!("Failed to get config directory: {}", e))?;
                println!("{}", config_dir.display());
            }
            ConfigAction::Get { key } => {
                if let Some(value) = config.get_value(key) {
                    println!("{}", value);
                } else {
                    return Err(anyhow::anyhow!("Unknown config key: {}", key));
                }
            }
            ConfigAction::Set { key, value } => {
                // We can clone and write a "new" config, as the command is finished after this and
                // for the next call the config will be read again.
                let mut new_config = config.clone();
                new_config
                    .set_value(key, value)
                    .map_err(|e| anyhow::anyhow!("Failed to set config value: {}", e))?;

                new_config
                    .save()
                    .map_err(|e| anyhow::anyhow!("Failed to save config: {}", e))?;

                if verbose {
                    println!(
                        "{}",
                        formatting::success_text(&format!("Set {} = {}", key, value))
                    );
                }
            }
            ConfigAction::List => {
                let values = config.list_values();
                for (key, value) in values {
                    println!("{} = {}", key, value);
                }
            }
        }
        Ok(())
    }
}
