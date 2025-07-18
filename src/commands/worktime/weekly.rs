use super::super::Command;
use crate::utils::date::Week;
use crate::utils::formatting::WeekFormat;
use crate::utils::formatting::{self, DurationFormat};
use crate::utils::spinner::{SpinnerConfig, SpinnerGuard};
use crate::watson::frame::Frames;
use crate::watson::{LogQuery, WatsonClient};
use anyhow::Result;
use chrono::{Datelike, Duration, Weekday};
use clap::Parser;
use owo_colors::{OwoColorize, colors::*};
use tabled::settings::{Alignment, Style};
use tabled::{Table, Tabled};

// Worktime-specific color aliases
type NoWorkColor = Red;
type LowWorkColor = Yellow;
type MediumWorkColor = Cyan;
type HighWorkColor = Green;

#[derive(Tabled)]
struct WeeklyTableRow {
    #[tabled(rename = "Mon")]
    mon: String,
    #[tabled(rename = "Tue")]
    tue: String,
    #[tabled(rename = "Wed")]
    wed: String,
    #[tabled(rename = "Thu")]
    thu: String,
    #[tabled(rename = "Fri")]
    fri: String,
    #[tabled(rename = "Sat")]
    sat: String,
    #[tabled(rename = "Sun")]
    sun: String,
    #[tabled(rename = "Total")]
    total: String,
}

pub struct WeeklyTableRenderer;

impl WeeklyTableRenderer {
    pub fn render_weeks(week_frames: &[(&Week, Frames)]) -> String {
        let mut result = String::new();

        for (i, (week, frames)) in week_frames.iter().enumerate() {
            if i > 0 {
                result.push('\n');
            }

            // Week header
            result.push_str(&format!("{}\n", week.to_string_long()));

            // Create table for this week
            let data_row = Self::create_week_row(week, frames);
            let table_data = vec![data_row];

            let mut table = Table::new(&table_data);
            table.with(Style::rounded()).with(Alignment::center());

            result.push_str(&table.to_string());
        }

        result
    }

    fn create_week_row(week: &Week, frames: &Frames) -> WeeklyTableRow {
        let frames_by_date = frames.by_date();
        let mut daily_durations = std::collections::HashMap::new();
        let total_duration = frames.total_duration();

        // Calculate duration for each day of the week
        for i in 0..7 {
            let date = week.start + Duration::days(i as i64);
            let weekday = date.weekday();

            let duration = frames_by_date
                .get(&date)
                .map(|day_frames| day_frames.total_duration())
                .unwrap_or_else(Duration::zero);

            daily_durations.insert(weekday, duration);
        }

        WeeklyTableRow {
            mon: Self::format_duration_with_color(daily_durations[&Weekday::Mon], true),
            tue: Self::format_duration_with_color(daily_durations[&Weekday::Tue], true),
            wed: Self::format_duration_with_color(daily_durations[&Weekday::Wed], true),
            thu: Self::format_duration_with_color(daily_durations[&Weekday::Thu], true),
            fri: Self::format_duration_with_color(daily_durations[&Weekday::Fri], true),
            sat: Self::format_duration_with_color(daily_durations[&Weekday::Sat], true),
            sun: Self::format_duration_with_color(daily_durations[&Weekday::Sun], true),
            total: Self::format_duration_with_color(total_duration, false),
        }
    }

    fn format_duration_with_color(duration: Duration, is_daily: bool) -> String {
        let hours = duration.num_hours();
        let formatted = duration.to_string_hhmm();

        if is_daily {
            // Daily thresholds: 0hrs=Red, 1-4hrs=Yellow, 5-7hrs=Cyan, 8+hrs=Green
            match hours {
                0 => formatted.fg::<NoWorkColor>().to_string(),
                1..=4 => formatted.fg::<LowWorkColor>().to_string(),
                5..=7 => formatted.fg::<MediumWorkColor>().to_string(),
                _ => formatted.fg::<HighWorkColor>().to_string(),
            }
        } else {
            // Weekly thresholds: 0hrs=Red, 1-3hrs=Yellow, 4-7hrs=Cyan, 8+hrs=Green
            match hours {
                0 => formatted.fg::<NoWorkColor>().to_string(),
                1..=3 => formatted.fg::<LowWorkColor>().to_string(),
                4..=7 => formatted.fg::<MediumWorkColor>().to_string(),
                _ => formatted.fg::<HighWorkColor>().to_string(),
            }
        }
    }
}

#[derive(Parser)]
pub struct WorktimeWeeklyCommand {
    /// Number of weeks to show (default: 4)
    #[arg(long, default_value = "4")]
    weeks: u32,
}

impl Command for WorktimeWeeklyCommand {
    fn run(&self, watson_client: &WatsonClient, verbose: bool) -> Result<()> {
        if verbose {
            println!(
                "{}",
                formatting::verbose_text("Running worktime:weekly command in verbose mode")
            );
        }

        // Get the last N weeks
        let weeks = Week::last_n_weeks(self.weeks);

        let week_frames = {
            let _spinner = SpinnerGuard::new(SpinnerConfig::default());
            let mut week_frames = vec![];

            for week in &weeks {
                let query = LogQuery::week(week);
                let frames = watson_client.log(query)?;
                week_frames.push((week, frames));
            }
            week_frames
        };

        let table = WeeklyTableRenderer::render_weeks(&week_frames);
        println!("{}", table);

        Ok(())
    }
}
