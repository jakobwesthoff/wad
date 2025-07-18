use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Watson time tracking frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub id: String,
    pub project: String,
    pub start: DateTime<Utc>,
    pub stop: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

impl Frame {
    /// Calculate the duration of this frame
    pub fn duration(&self) -> chrono::Duration {
        match self.stop {
            Some(stop) => stop - self.start,
            None => chrono::Utc::now() - self.start,
        }
    }

    /// Check if this frame is currently active (no stop time)
    pub fn is_active(&self) -> bool {
        self.stop.is_none()
    }

    /// Get a human-readable duration string
    pub fn duration_string(&self) -> String {
        let duration = self.duration();
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

/// Collection of frames with helper methods
#[derive(Debug, Clone)]
pub struct Frames {
    pub frames: Vec<Frame>,
}

impl Frames {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self { frames }
    }

    /// Get total duration across all frames
    pub fn total_duration(&self) -> chrono::Duration {
        self.frames
            .iter()
            .map(|frame| frame.duration())
            .fold(chrono::Duration::zero(), |acc, duration| acc + duration)
    }

    /// Group frames by project
    pub fn by_project(&self) -> HashMap<String, Frames> {
        let mut grouped: HashMap<String, Vec<Frame>> = HashMap::new();
        for frame in &self.frames {
            grouped
                .entry(frame.project.clone())
                .or_default()
                .push(frame.clone());
        }

        grouped
            .into_iter()
            .map(|(project, frames)| (project, Frames::from(frames)))
            .collect()
    }

    /// Get currently active frames
    pub fn active_frames(&self) -> Vec<&Frame> {
        self.frames
            .iter()
            .filter(|frame| frame.is_active())
            .collect()
    }

    /// Check if any frame is currently active
    pub fn has_active_frames(&self) -> bool {
        self.frames.iter().any(|frame| frame.is_active())
    }

    /// Group frames by date (ignoring time)
    pub fn by_date(&self) -> HashMap<NaiveDate, Frames> {
        let mut grouped: HashMap<NaiveDate, Vec<Frame>> = HashMap::new();
        for frame in &self.frames {
            let date = frame.start.date_naive();
            grouped.entry(date).or_default().push(frame.clone());
        }

        grouped
            .into_iter()
            .map(|(date, frames)| (date, Frames::from(frames)))
            .collect()
    }
}

impl From<Vec<Frame>> for Frames {
    fn from(frames: Vec<Frame>) -> Self {
        Self::new(frames)
    }
}
