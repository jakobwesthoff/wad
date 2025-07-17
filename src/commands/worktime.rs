use super::Command;
use crate::utils::formatting;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct WorktimeTodayCommand {
    // No arguments for now
}

impl Command for WorktimeTodayCommand {
    fn run(&self, verbose: bool) -> Result<()> {
        if verbose {
            println!(
                "{}",
                formatting::verbose_text("Running worktime:today command in verbose mode")
            );
        }
        println!(
            "{}: {}",
            formatting::header_text("Today's work time"),
            formatting::info_text("[placeholder]")
        );
        Ok(())
    }
}
