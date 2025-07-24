use crate::utils::date::{DayTimeBreakdown, Week};
use crate::wad_data::{AbsenceRecord, AbsenceType};
use chrono::{Datelike, Duration};
use owo_colors::{OwoColorize, colors::*};
use std::fmt;

// Semantic color type aliases
pub type SuccessColor = Green;
pub type ErrorColor = Red;
pub type WarningColor = Yellow;
pub type InfoColor = Cyan;
pub type VerboseColor = BrightMagenta;

// Worktime-specific color aliases
pub type NoWorkColor = Red;
pub type LowWorkColor = Yellow;
pub type MediumWorkColor = Cyan;
pub type HighWorkColor = Green;

// Absence-specific color aliases
pub type AbsenceIdColor = BrightBlack;
pub type AbsenceHoursColor = Blue;
pub type AbsenceNoteColor = BrightBlack;

/// Format success messages
pub fn success_text(text: &str) -> String {
    text.fg::<SuccessColor>().to_string()
}

/// Format error messages
pub fn error_text(text: &str) -> String {
    text.fg::<ErrorColor>().to_string()
}

/// Format warning messages
pub fn warning_text(text: &str) -> String {
    text.fg::<WarningColor>().to_string()
}

/// Format info messages
pub fn info_text(text: &str) -> String {
    text.fg::<InfoColor>().to_string()
}

/// Format headers/titles
pub fn header_text(text: &str) -> String {
    text.bold().to_string()
}

/// Format verbose/debug messages
pub fn verbose_text(text: &str) -> String {
    text.fg::<VerboseColor>().to_string()
}

/// Trait for formatting durations in a human-readable way
pub trait DurationFormat {
    fn to_string_hhmm(&self) -> String;
    fn to_string_long_hhmm(&self) -> String;
    fn to_string_weekly_worktime_colored(&self, config: &crate::config::Config) -> String;
}

impl DurationFormat for chrono::Duration {
    fn to_string_hhmm(&self) -> String {
        format!("{:02}:{:02}", self.num_hours(), self.num_minutes() % 60)
    }

    fn to_string_long_hhmm(&self) -> String {
        let hours = self.num_hours();
        let minutes = self.num_minutes() % 60;

        match (hours, minutes) {
            (0, 0) => "0 minutes".to_string(),
            (0, m) => format!("{} minute{}", m, if m == 1 { "" } else { "s" }),
            (h, 0) => format!("{} hour{}", h, if h == 1 { "" } else { "s" }),
            (h, m) => format!(
                "{} hour{} and {} minute{}",
                h,
                if h == 1 { "" } else { "s" },
                m,
                if m == 1 { "" } else { "s" }
            ),
        }
    }

    fn to_string_weekly_worktime_colored(&self, config: &crate::config::Config) -> String {
        let hours = self.num_hours() as f64;
        let formatted = self.to_string_hhmm();

        if hours < config.workhours_per_week {
            formatted.fg::<LowWorkColor>().to_string()
        } else {
            formatted.fg::<HighWorkColor>().to_string()
        }
    }
}

/// Trait for formatting weeks in a human-readable way
pub trait WeekFormat {
    fn to_string_long(&self) -> String;
}

impl WeekFormat for Week {
    fn to_string_long(&self) -> String {
        if self.start.month() == self.end.month() {
            format!(
                "{} - {}. {} {}",
                self.start.day(),
                self.end.day(),
                self.start.format("%B"),
                self.start.year()
            )
        } else {
            format!(
                "{}. {} - {}. {} {}",
                self.start.day(),
                self.start.format("%B"),
                self.end.day(),
                self.end.format("%B"),
                self.start.year()
            )
        }
    }
}

// Absence type color aliases
pub type VacationColor = Green;
pub type SickColor = Red;
pub type OvertimeReductionColor = Blue;
pub type HolidayColor = Magenta;
pub type OtherAbsenceColor = Yellow;

/// Trait for formatting absence types with colors
pub trait AbsenceTypeFormat {
    fn to_string_colored(&self) -> String;
    fn to_emoji(&self) -> &'static str;
}

impl AbsenceTypeFormat for AbsenceType {
    fn to_string_colored(&self) -> String {
        match self {
            AbsenceType::Vacation => "Vacation".fg::<VacationColor>().to_string(),
            AbsenceType::Sick => "Sick".fg::<SickColor>().to_string(),
            AbsenceType::OvertimeReduction => "Overtime Reduction"
                .fg::<OvertimeReductionColor>()
                .to_string(),
            AbsenceType::Holiday => "Holiday".fg::<HolidayColor>().to_string(),
            AbsenceType::Other(custom) => format!("Other: {}", custom)
                .fg::<OtherAbsenceColor>()
                .to_string(),
        }
    }

    fn to_emoji(&self) -> &'static str {
        match self {
            AbsenceType::Vacation => "🏖️",
            AbsenceType::Sick => "🏥",
            AbsenceType::OvertimeReduction => "⚖️",
            AbsenceType::Holiday => "🎉",
            AbsenceType::Other(_) => "📝",
        }
    }
}

/// Trait for formatting time breakdowns with split display
pub trait TimeBreakdownFormat {
    fn to_string_split_colored(&self, config: &crate::config::Config) -> String;
    fn to_string_combined_with_indicator(&self, config: &crate::config::Config) -> String;
}

impl TimeBreakdownFormat for DayTimeBreakdown {
    fn to_string_split_colored(&self, config: &crate::config::Config) -> String {
        let total = self.total_duration();
        let base_watson = self.watson_duration.to_string_hhmm();

        // Color the base duration based on total time
        let colored_watson = if total.num_hours() as f64 <= config.daily_worktime_low {
            base_watson.fg::<NoWorkColor>().to_string()
        } else if (total.num_hours() as f64) < config.daily_worktime_medium {
            base_watson.fg::<LowWorkColor>().to_string()
        } else if (total.num_hours() as f64) < config.daily_worktime_good {
            base_watson.fg::<MediumWorkColor>().to_string()
        } else {
            base_watson.fg::<HighWorkColor>().to_string()
        };

        let mut result = colored_watson;

        for absence in &self.absences {
            let absence_duration = Duration::hours(absence.hours as i64)
                + Duration::minutes(((absence.hours % 1.0) * 60.0) as i64);

            // Color the absence duration with dimmed type color
            let colored_absence_time = match absence.absence_type {
                AbsenceType::Vacation => absence_duration
                    .to_string_hhmm()
                    .fg::<VacationColor>()
                    .dimmed()
                    .to_string(),
                AbsenceType::Sick => absence_duration
                    .to_string_hhmm()
                    .fg::<SickColor>()
                    .dimmed()
                    .to_string(),
                AbsenceType::OvertimeReduction => absence_duration
                    .to_string_hhmm()
                    .fg::<OvertimeReductionColor>()
                    .dimmed()
                    .to_string(),
                AbsenceType::Holiday => absence_duration
                    .to_string_hhmm()
                    .fg::<HolidayColor>()
                    .dimmed()
                    .to_string(),
                AbsenceType::Other(_) => absence_duration
                    .to_string_hhmm()
                    .fg::<OtherAbsenceColor>()
                    .dimmed()
                    .to_string(),
            };

            result.push_str(&format!(
                "+{}{}",
                colored_absence_time,
                absence.absence_type.to_emoji()
            ));
        }

        result
    }

    fn to_string_combined_with_indicator(&self, config: &crate::config::Config) -> String {
        let total = self.total_duration();
        let formatted_total = total.to_string_hhmm();

        // Color based on total duration (Watson + absences)
        let colored_total = if total.num_hours() as f64 <= config.daily_worktime_low {
            formatted_total.fg::<NoWorkColor>().to_string()
        } else if (total.num_hours() as f64) < config.daily_worktime_medium {
            formatted_total.fg::<LowWorkColor>().to_string()
        } else if (total.num_hours() as f64) < config.daily_worktime_good {
            formatted_total.fg::<MediumWorkColor>().to_string()
        } else {
            formatted_total.fg::<HighWorkColor>().to_string()
        };

        // Add + indicator if there are absences
        if self.absences.is_empty() {
            colored_total
        } else {
            format!("{}+", colored_total)
        }
    }
}

impl fmt::Display for AbsenceRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ulid_str = self.id.to_string().fg::<AbsenceIdColor>().to_string();
        let hours = format!("{} hours", self.hours)
            .fg::<AbsenceHoursColor>()
            .to_string();
        let absence_type = self.absence_type.to_string_colored();
        let note = self
            .note
            .as_deref()
            .unwrap_or("(no note)")
            .fg::<AbsenceNoteColor>()
            .to_string();

        write!(f, "{} | {} | {} | {}", ulid_str, hours, absence_type, note)
    }
}
