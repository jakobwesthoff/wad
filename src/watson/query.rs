use chrono::NaiveDate;

/// Parameters for Watson log command
#[derive(Debug, Clone)]
pub struct LogQuery {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub include_current: bool,
}

impl LogQuery {
    /// Create a new log query
    pub fn new(from: NaiveDate, to: NaiveDate) -> Self {
        Self {
            from,
            to,
            include_current: false,
        }
    }

    /// Create a log query for today
    pub fn today() -> Self {
        let today = chrono::Utc::now().date_naive();
        Self::new(today, today)
    }

    /// Include current/active frames in the query
    pub fn with_current(mut self) -> Self {
        self.include_current = true;
        self
    }

    /// Convert to Watson command line arguments
    pub fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            "log".to_string(),
            "--from".to_string(),
            self.from.format("%Y-%m-%d").to_string(),
            "--to".to_string(),
            self.to.format("%Y-%m-%d").to_string(),
            "--json".to_string(),
        ];

        if self.include_current {
            args.push("--current".to_string());
        }

        args
    }
}
