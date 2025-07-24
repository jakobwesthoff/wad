use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::editor::EditableDocument;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AbsenceRecord {
    pub id: Ulid,
    pub date: NaiveDate,
    pub hours: f64,
    pub absence_type: AbsenceType,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AbsenceType {
    Vacation,
    Sick,
    OvertimeReduction,
    Holiday,
    Other(String),
}

pub trait AbsenceStorage {
    type Error;

    fn add_absence(&self, record: AbsenceRecord) -> Result<(), Self::Error>;
    fn get_absence(&self, date: NaiveDate) -> Result<Vec<AbsenceRecord>, Self::Error>;
    fn remove_absence(&self, date: NaiveDate, id: Ulid) -> Result<bool, Self::Error>;
    fn update_absence(
        &self,
        date: NaiveDate,
        updated_record: AbsenceRecord,
    ) -> Result<(), Self::Error>;
}

impl EditableDocument for AbsenceRecord {
    fn validate(&self, original: &Self) -> Result<(), String> {
        // ULID must not be changed (immutable identity)
        if self.id != original.id {
            return Err("ULID cannot be changed".to_string());
        }

        // Hours must be non-negative
        if self.hours < 0.0 {
            return Err("Hours cannot be negative".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("2024-01-15", AbsenceType::Vacation, 8.0, Some("Annual leave".to_string()); "vacation with note")]
    #[test_case("2024-01-16", AbsenceType::Sick, 4.0, None; "sick half day no note")]
    #[test_case("2024-01-17", AbsenceType::OvertimeReduction, 8.0, Some("Comp time".to_string()); "overtime reduction")]
    #[test_case("2024-01-18", AbsenceType::Holiday, 8.0, Some("New Year's Day".to_string()); "holiday")]
    #[test_case("2024-01-19", AbsenceType::Other("Bereavement".to_string()), 8.0, Some("Family emergency".to_string()); "bereavement")]
    #[test_case("2024-01-20", AbsenceType::Other("Mental Health Day".to_string()), 4.0, None; "mental health day")]
    fn test_absence_record_serialization(
        date_str: &str,
        absence_type: AbsenceType,
        hours: f64,
        note: Option<String>,
    ) {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let record = AbsenceRecord {
            id: Ulid::new(),
            date,
            hours,
            absence_type,
            note,
        };

        let json = serde_json::to_string_pretty(&record).unwrap();
        println!("AbsenceRecord:\n{}\n", json);

        // Test round-trip deserialization
        let deserialized: AbsenceRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.date, deserialized.date);
        assert_eq!(record.hours, deserialized.hours);
        assert_eq!(record.note, deserialized.note);

        // Compare absence types
        match (&record.absence_type, &deserialized.absence_type) {
            (AbsenceType::Other(orig), AbsenceType::Other(deser)) => assert_eq!(orig, deser),
            _ => assert_eq!(
                std::mem::discriminant(&record.absence_type),
                std::mem::discriminant(&deserialized.absence_type)
            ),
        }
    }
}
