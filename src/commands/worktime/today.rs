use super::super::Command;
use crate::utils::formatting::{self, DurationFormat};
use crate::utils::spinner::{SpinnerConfig, SpinnerGuard};
use crate::watson::{LogQuery, WatsonClient};
use anyhow::Result;
use clap::Parser;
use owo_colors::{OwoColorize, colors::*};

// Worktime-specific color aliases
type NoWorkColor = Red;
type LowWorkColor = Yellow;
type MediumWorkColor = Cyan;
type HighWorkColor = Green;

#[derive(Parser)]
pub struct WorktimeTodayCommand {
    /// Show breakdown by projects
    #[arg(long)]
    projects: bool,
}

impl Command for WorktimeTodayCommand {
    fn run(&self, watson_client: &WatsonClient, verbose: bool) -> Result<()> {
        if verbose {
            println!(
                "{}",
                formatting::verbose_text("Running worktime:today command in verbose mode")
            );
        }

        let frames = {
            let _spinner = SpinnerGuard::new(SpinnerConfig::default());
            let query = LogQuery::today().with_current();
            watson_client.log(query)?
        };

        // Show project breakdown if requested
        if self.projects {
            let projects = frames.by_project();
            for (project_name, project_frames) in projects {
                let project_duration = project_frames.total_duration();
                let short_duration = project_duration.to_string_hhmm();
                let long_duration = project_duration.to_string_long_hhmm();

                println!(
                    "{}: {} ({})",
                    project_name.fg::<Cyan>(),
                    short_duration.fg::<Blue>(),
                    long_duration
                );
            }
            println!(); // Empty line before total
        }

        let total_duration = frames.total_duration();
        let hours = total_duration.num_hours();
        let short_duration = total_duration.to_string_hhmm();
        let long_duration = total_duration.to_string_long_hhmm();

        // Color based on hours worked
        let colored_duration = match hours {
            0 => short_duration.fg::<NoWorkColor>().to_string(),
            1..=3 => short_duration.fg::<LowWorkColor>().to_string(), // <4h
            4..=7 => short_duration.fg::<MediumWorkColor>().to_string(), // <8h
            _ => short_duration.fg::<HighWorkColor>().to_string(),    // >=8h
        };

        println!("Worktime today: {} ({})", colored_duration, long_duration);
        Ok(())
    }
}
