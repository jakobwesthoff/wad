use crate::wad_data::AbsenceRecord;
use chrono::{Datelike, Duration, Local, NaiveDate};
use derive_more::{Deref, From};

/// Type-safe wrapper for daily worktime durations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, From)]
pub struct DailyWorktime(pub Duration);

/// Type-safe wrapper for weekly worktime durations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, From)]
pub struct WeeklyWorktime(pub Duration);

/// Represents a week with Monday as the first day and Sunday as the last day
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Week {
    pub start: NaiveDate, // Monday
    pub end: NaiveDate,   // Sunday
}

impl Week {
    /// Create a new Week from a Monday start date
    pub fn new(monday: NaiveDate) -> Self {
        let sunday = monday + Duration::days(6);
        Self {
            start: monday,
            end: sunday,
        }
    }

    /// Get the current week (Monday to Sunday)
    pub fn current() -> Self {
        let today = Local::now().date_naive();
        let days_from_monday = today.weekday().num_days_from_monday();
        let monday = today - Duration::days(days_from_monday as i64);
        Self::new(monday)
    }

    /// Get a week offset by the given number of weeks from the current week
    /// offset = 0: current week
    /// offset = 1: last week
    /// offset = 2: two weeks ago
    pub fn offset(weeks_back: i32) -> Self {
        let current_week = Self::current();
        let target_monday = current_week.start - Duration::weeks(weeks_back as i64);
        Self::new(target_monday)
    }

    /// Get the last N weeks
    /// Returns weeks from oldest to newest
    pub fn last_n_weeks(n: u32) -> Vec<Self> {
        (0..n)
            .map(|i| Self::offset(i as i32))
            .rev() // Reverse to get oldest to newest
            .collect()
    }
}

/// Data structure representing a day's time breakdown: work + absences
#[derive(Debug, Clone)]
pub struct DayTimeBreakdown {
    pub watson_duration: Duration,
    pub absences: Vec<AbsenceRecord>,
}

impl DayTimeBreakdown {
    pub fn new(watson_duration: Duration, absences: Vec<AbsenceRecord>) -> Self {
        Self {
            watson_duration,
            absences,
        }
    }

    pub fn total_duration(&self) -> Duration {
        let absence_duration = self
            .absences
            .iter()
            .map(|record| {
                Duration::hours(record.hours as i64)
                    + Duration::minutes(((record.hours % 1.0) * 60.0) as i64)
            })
            .fold(Duration::zero(), |acc, d| acc + d);

        self.watson_duration + absence_duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;

    #[test]
    fn test_week_new() {
        let monday = NaiveDate::from_ymd_opt(2023, 7, 17).unwrap(); // A Monday
        let week = Week::new(monday);

        assert_eq!(week.start, monday);
        assert_eq!(week.end, NaiveDate::from_ymd_opt(2023, 7, 23).unwrap()); // Sunday
    }

    #[test]
    fn test_week_current() {
        let week = Week::current();

        // Start should be a Monday
        assert_eq!(week.start.weekday(), Weekday::Mon);
        // End should be a Sunday
        assert_eq!(week.end.weekday(), Weekday::Sun);
        // Week should span exactly 7 days
        assert_eq!((week.end - week.start).num_days(), 6);
    }

    #[test]
    fn test_week_offset() {
        let current = Week::current();
        let last_week = Week::offset(1);

        // Last week should be 7 days earlier
        assert_eq!((current.start - last_week.start).num_days(), 7);
        assert_eq!((current.end - last_week.end).num_days(), 7);
    }

    #[test]
    fn test_last_n_weeks() {
        let weeks = Week::last_n_weeks(4);

        assert_eq!(weeks.len(), 4);

        // Should be ordered from oldest to newest
        for i in 1..weeks.len() {
            assert!(weeks[i - 1].start < weeks[i].start);
        }

        // Each week should be exactly 7 days apart
        for i in 1..weeks.len() {
            assert_eq!((weeks[i].start - weeks[i - 1].start).num_days(), 7);
        }
    }
}
