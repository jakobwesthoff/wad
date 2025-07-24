use super::super::Command;
use crate::config::Config;
use crate::utils::date::{DayTimeBreakdown, Week, WeeklyWorktime};
use crate::utils::formatting::WeekFormat;
use crate::utils::formatting::{self, TimeBreakdownFormat};
use crate::utils::spinner::{SpinnerConfig, SpinnerGuard};
use crate::wad_data::{AbsenceStorage, JsonDataStore, WadDataStore};
use crate::watson::frame::Frames;
use crate::watson::{LogQuery, WatsonClient};
use anyhow::Result;
use chrono::{Datelike, Duration, Weekday};
use clap::Parser;
use std::collections::HashMap;
use tabled::Table;
use tabled::builder::Builder;
use tabled::settings::themes::BorderCorrection;
use tabled::settings::{Alignment, Span, Style};

pub struct WeeklyTableBuilder;

impl WeeklyTableBuilder {
    pub fn build(
        week_frames: &[(&Week, Frames)],
        config: &Config,
        store: &JsonDataStore,
        show_absence_details: bool,
    ) -> Result<Table> {
        let mut b = Builder::new();
        // Headers
        b.push_record(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun", "Total"]);

        for (week, frames) in week_frames.iter() {
            // Week header
            b.push_record([&week.to_string_long()]);

            // Create row for this week
            b.push_record(Self::create_week_row(
                week,
                frames,
                config,
                store,
                show_absence_details,
            )?);
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

        Ok(table)
    }

    fn create_week_row(
        week: &Week,
        frames: &Frames,
        config: &Config,
        store: &JsonDataStore,
        show_absence_details: bool,
    ) -> Result<Vec<String>> {
        let frames_by_date = frames.by_date();
        let mut daily_breakdowns = HashMap::new();

        // Calculate breakdown for each day of the week
        for i in 0..7 {
            let date = week.start + Duration::days(i as i64);
            let weekday = date.weekday();

            let watson_duration = frames_by_date
                .get(&date)
                .map(|day_frames| day_frames.total_duration())
                .unwrap_or_else(Duration::zero);

            let absences = store.get_absence(date)?;
            let breakdown = DayTimeBreakdown::new(watson_duration, absences);

            daily_breakdowns.insert(weekday, breakdown);
        }

        // Calculate weekly total by summing all daily breakdowns
        let weekly_total: WeeklyWorktime = daily_breakdowns
            .values()
            .map(|breakdown| breakdown.total_duration())
            .fold(Duration::zero(), |acc, d| acc + d)
            .into();

        // Choose formatting based on show_absence_details flag
        let format_day = |breakdown: &DayTimeBreakdown| {
            if show_absence_details {
                breakdown.to_string_split_colored(config)
            } else {
                breakdown.to_string_combined_with_indicator(config)
            }
        };

        Ok(vec![
            format_day(&daily_breakdowns[&Weekday::Mon]),
            format_day(&daily_breakdowns[&Weekday::Tue]),
            format_day(&daily_breakdowns[&Weekday::Wed]),
            format_day(&daily_breakdowns[&Weekday::Thu]),
            format_day(&daily_breakdowns[&Weekday::Fri]),
            format_day(&daily_breakdowns[&Weekday::Sat]),
            format_day(&daily_breakdowns[&Weekday::Sun]),
            weekly_total.to_string_colored(config),
        ])
    }
}

#[derive(Parser)]
pub struct WorktimeWeeklyCommand {
    /// Number of weeks to show (default: 4)
    #[arg(long, default_value = "4")]
    weeks: u32,
    /// Show detailed absence breakdown instead of combined totals
    #[arg(long)]
    absence: bool,
}

impl Command for WorktimeWeeklyCommand {
    fn run(&self, watson_client: &WatsonClient, config: &Config, verbose: bool) -> Result<()> {
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
                let query = LogQuery::week(week).with_current();
                let frames = watson_client.log(query)?;
                week_frames.push((week, frames));
            }
            week_frames
        };

        // Open absence store once for the entire operation
        let store = JsonDataStore::open()?;
        let table = WeeklyTableBuilder::build(&week_frames, config, &store, self.absence)?;
        println!("{}", table);

        Ok(())
    }
}
