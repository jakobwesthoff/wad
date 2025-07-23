use super::super::Command;
use crate::config::Config;
use crate::utils::date::DayTimeBreakdown;
use crate::utils::formatting::{self, DurationFormat, TimeBreakdownFormat};
use crate::utils::spinner::{SpinnerConfig, SpinnerGuard};
use crate::wad_data::{AbsenceStorage, JsonDataStore, WadDataStore};
use crate::watson::{LogQuery, WatsonClient};
use anyhow::Result;
use chrono::Local;
use clap::Parser;
use owo_colors::{OwoColorize, colors::*};

#[derive(Parser)]
pub struct WorktimeTodayCommand {
    /// Show breakdown by projects
    #[arg(long)]
    projects: bool,
}

impl Command for WorktimeTodayCommand {
    fn run(&self, watson_client: &WatsonClient, config: &Config, verbose: bool) -> Result<()> {
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

        // Load today's absences
        let today = Local::now().date_naive();
        let absences = {
            let store = JsonDataStore::open()?;
            store.get_absence(today)?
        };

        // Create day breakdown
        let watson_duration = frames.total_duration();
        let day_breakdown = DayTimeBreakdown::new(watson_duration, absences);

        // Display split format
        let split_display = day_breakdown.to_string_split_colored(config);
        let total_duration = day_breakdown.total_duration();
        let long_duration = total_duration.to_string_long_hhmm();

        println!("Worktime today: {} ({})", split_display, long_duration);
        Ok(())
    }
}
