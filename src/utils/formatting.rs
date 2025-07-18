use crate::utils::date::Week;
use chrono::Datelike;
use owo_colors::{OwoColorize, colors::*};

// Semantic color type aliases
pub type SuccessColor = Green;
pub type ErrorColor = Red;
pub type WarningColor = Yellow;
pub type InfoColor = Cyan;
pub type VerboseColor = BrightMagenta;

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
}

/// Trait for formatting weeks in a human-readable way
pub trait WeekFormat {
    fn to_string_short(&self) -> String;
    fn to_string_long(&self) -> String;
}

impl WeekFormat for Week {
    fn to_string_short(&self) -> String {
        format!(
            "{} - {}",
            self.start.format("%d.%m"),
            self.end.format("%d.%m")
        )
    }

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
