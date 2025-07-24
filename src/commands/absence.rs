use super::Command;
use crate::config::Config;
use crate::editor::EditorSession;
use crate::utils::formatting::{self, AbsenceTypeFormat};
use crate::wad_data::{AbsenceRecord, AbsenceStorage, AbsenceType, JsonDataStore, WadDataStore};
use crate::watson::WatsonClient;
use anyhow::Result;
use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};
use owo_colors::{OwoColorize, colors::*};
use ulid::Ulid;

// Absence UI color aliases
type AbsenceIdColor = BrightBlack;
type AbsenceHoursColor = Blue;
type AbsenceDateColor = Cyan;
type AbsenceNoteColor = BrightBlack;

#[derive(Parser)]
pub struct AbsenceCommand {
    #[command(subcommand)]
    action: AbsenceAction,
}

#[derive(Subcommand)]
enum AbsenceAction {
    /// Show all absences for a specific date
    Show {
        /// Date to show absences for (YYYY-MM-DD, 'today', 'yesterday', 'tomorrow')
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
    },
    /// Add a new absence record
    Add {
        /// Date for the absence (YYYY-MM-DD, 'today', 'yesterday', 'tomorrow')
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
        /// Hours for the absence
        hours: f64,
        /// Type of absence (vacation, sick, overtime-reduction, holiday, other:custom)
        #[arg(name = "type", value_parser = parse_absence_type)]
        absence_type: AbsenceType,
        /// Optional note for the absence
        #[arg(long)]
        note: Option<String>,
    },
    /// Remove a specific absence record
    Remove {
        /// Date of the absence (YYYY-MM-DD, 'today', 'yesterday', 'tomorrow')
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
        /// ULID of the specific absence record to remove
        #[arg(name = "id", value_parser = parse_ulid)]
        ulid: Ulid,
    },
    /// Edit a specific absence record
    Edit {
        /// Date of the absence (YYYY-MM-DD, 'today', 'yesterday', 'tomorrow')
        #[arg(value_parser = parse_date)]
        date: NaiveDate,
        /// ULID of the specific absence record to edit
        #[arg(name = "id", value_parser = parse_ulid)]
        ulid: Ulid,
    },
    /// Show the path to the absence data directory
    Path,
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    match s.to_lowercase().as_str() {
        "today" => Ok(Local::now().date_naive()),
        "yesterday" => Ok(Local::now().date_naive() - chrono::Duration::days(1)),
        "tomorrow" => Ok(Local::now().date_naive() + chrono::Duration::days(1)),
        _ => NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| {
            "Invalid date format. Use YYYY-MM-DD, 'today', 'yesterday', or 'tomorrow'".to_string()
        }),
    }
}

fn parse_absence_type(s: &str) -> Result<AbsenceType, String> {
    match s.to_lowercase().as_str() {
        "vacation" => Ok(AbsenceType::Vacation),
        "sick" => Ok(AbsenceType::Sick),
        "overtime-reduction" => Ok(AbsenceType::OvertimeReduction),
        "holiday" => Ok(AbsenceType::Holiday),
        _ => {
            if let Some(custom) = s.strip_prefix("other:") {
                Ok(AbsenceType::Other(custom.to_string()))
            } else {
                Err("Invalid absence type. Use: vacation, sick, overtime-reduction, holiday, or other:custom".to_string())
            }
        }
    }
}

fn parse_ulid(s: &str) -> Result<Ulid, String> {
    Ulid::from_string(s).map_err(|_| "Invalid ULID format".to_string())
}

fn show_absences(date: NaiveDate) -> Result<()> {
    let store = JsonDataStore::open()?;
    let absences = store.get_absence(date)?;

    let formatted_date = date
        .format("%Y-%m-%d")
        .to_string()
        .fg::<AbsenceDateColor>()
        .to_string();

    if absences.is_empty() {
        println!("No absences found for {}", formatted_date);
    } else {
        println!("Absences for {}:", formatted_date);
        for absence in absences {
            let ulid_str = absence.id.to_string().fg::<AbsenceIdColor>().to_string();
            let hours = format!("{} hours", absence.hours)
                .fg::<AbsenceHoursColor>()
                .to_string();
            let absence_type = absence.absence_type.to_string_colored();
            let note = absence
                .note
                .as_deref()
                .unwrap_or("(no note)")
                .fg::<AbsenceNoteColor>()
                .to_string();

            println!("  {} | {} | {} | {}", ulid_str, hours, absence_type, note);
        }
    }
    Ok(())
}

fn add_absence(
    date: NaiveDate,
    hours: f64,
    absence_type: AbsenceType,
    note: Option<String>,
) -> Result<()> {
    let store = JsonDataStore::open()?;

    let record = AbsenceRecord {
        id: Ulid::new(),
        date,
        hours,
        absence_type,
        note,
    };

    store.add_absence(record.clone())?;
    println!(
        "{} {} | {} | {} on {}",
        formatting::success_text("Added absence:"),
        record.id.to_string().fg::<AbsenceIdColor>(),
        format!("{} hours", record.hours).fg::<AbsenceHoursColor>(),
        record.absence_type.to_string_colored(),
        date.format("%Y-%m-%d").to_string().fg::<AbsenceDateColor>()
    );
    Ok(())
}

fn remove_absence(date: NaiveDate, ulid: Ulid) -> Result<()> {
    let store = JsonDataStore::open()?;

    let removed = store.remove_absence(date, ulid)?;
    if removed {
        println!(
            "{} {} from {}",
            formatting::success_text("Removed absence"),
            ulid.to_string().fg::<AbsenceIdColor>(),
            date.format("%Y-%m-%d").to_string().fg::<AbsenceDateColor>()
        );
    } else {
        println!(
            "{} {} on {}",
            formatting::warning_text("No absence found with ULID"),
            ulid.to_string().fg::<AbsenceIdColor>(),
            date.format("%Y-%m-%d").to_string().fg::<AbsenceDateColor>()
        );
    }
    Ok(())
}

fn edit_absence(date: NaiveDate, ulid: Ulid) -> Result<()> {
    let store = JsonDataStore::open()?;

    // Get all absences for the date to find the one to edit
    let absences = store.get_absence(date)?;
    let original_record = absences
        .iter()
        .find(|record| record.id == ulid)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No absence found with ULID {} on {}",
                ulid.to_string().fg::<AbsenceIdColor>(),
                date.format("%Y-%m-%d").to_string().fg::<AbsenceDateColor>()
            )
        })?
        .clone();

    // Create editor session and edit the record
    let editor_session = EditorSession::new(original_record.clone());
    let edited_record = match editor_session.edit() {
        Ok(record) => record,
        Err(crate::editor::EditorError::NoChanges) => {
            println!(
                "{} No changes made to absence {}",
                formatting::info_text("Info:"),
                ulid.to_string().fg::<AbsenceIdColor>()
            );
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    // Update the record in storage
    store.update_absence(date, edited_record.clone())?;

    println!(
        "{} {} | {} | {} on {}",
        formatting::success_text("Updated absence:"),
        edited_record.id.to_string().fg::<AbsenceIdColor>(),
        format!("{} hours", edited_record.hours).fg::<AbsenceHoursColor>(),
        edited_record.absence_type.to_string_colored(),
        edited_record
            .date
            .format("%Y-%m-%d")
            .to_string()
            .fg::<AbsenceDateColor>()
    );

    Ok(())
}

fn show_absence_path() -> Result<()> {
    let store = JsonDataStore::open()?;
    let absences_dir = store.absences_dir();
    println!("{}", absences_dir.display());
    Ok(())
}

impl Command for AbsenceCommand {
    fn run(&self, _watson_client: &WatsonClient, _config: &Config, _verbose: bool) -> Result<()> {
        match &self.action {
            AbsenceAction::Show { date } => show_absences(*date),
            AbsenceAction::Add {
                date,
                hours,
                absence_type,
                note,
            } => add_absence(*date, *hours, absence_type.clone(), note.clone()),
            AbsenceAction::Remove { date, ulid } => remove_absence(*date, *ulid),
            AbsenceAction::Edit { date, ulid } => edit_absence(*date, *ulid),
            AbsenceAction::Path => show_absence_path(),
        }
    }
}
