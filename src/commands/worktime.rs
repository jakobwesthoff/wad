use super::Command;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct WorktimeTodayCommand {
    // No arguments for now
}

impl Command for WorktimeTodayCommand {
    fn run(&self) -> Result<()> {
        println!("Today's work time: [placeholder]");
        Ok(())
    }
}
