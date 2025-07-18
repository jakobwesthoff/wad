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
use tabled::Table;
use tabled::builder::Builder;
use tabled::settings::themes::BorderCorrection;
use tabled::settings::{Alignment, Span, Style};

// Worktime-specific color aliases
type NoWorkColor = Red;
type LowWorkColor = Yellow;
type MediumWorkColor = Cyan;
type HighWorkColor = Green;

pub struct WeeklyTableBuilder;

impl WeeklyTableBuilder {
    pub fn build(week_frames: &[(&Week, Frames)]) -> Table {
        let mut b = Builder::new();
        // Headers
        b.push_record(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun", "Total"]);

        for (week, frames) in week_frames.iter() {
            // Week header
            b.push_record([&week.to_string_long()]);

            // Create row for this week
            b.push_record(Self::create_week_row(week, frames));
        }

        let mut table = b.build();
        table
            .with(Style::modern_rounded())
            .with(Alignment::center());

        // Set the span for the weekday headers.
        for row_id in 0..table.count_rows() {
            if row_id % 2 == 1 {
                table.modify((row_id, 0), Span::column(0));
            }
        }
        table.with(BorderCorrection::span());

        table
    }

    fn create_week_row(week: &Week, frames: &Frames) -> Vec<String> {
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

        vec![
            Self::format_duration_with_color(daily_durations[&Weekday::Mon], true),
            Self::format_duration_with_color(daily_durations[&Weekday::Tue], true),
            Self::format_duration_with_color(daily_durations[&Weekday::Wed], true),
            Self::format_duration_with_color(daily_durations[&Weekday::Thu], true),
            Self::format_duration_with_color(daily_durations[&Weekday::Fri], true),
            Self::format_duration_with_color(daily_durations[&Weekday::Sat], true),
            Self::format_duration_with_color(daily_durations[&Weekday::Sun], true),
            Self::format_duration_with_color(total_duration, false),
        ]
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

        let table = WeeklyTableBuilder::build(&week_frames);
        println!("{}", table);

        Ok(())
    }
}
