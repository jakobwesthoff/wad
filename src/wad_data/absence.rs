use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsenceRecord {
    pub date: NaiveDate,
    pub hours: f32,
    pub absence_type: AbsenceType,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbsenceType {
    Vacation,
    Sick,
    OvertimeReduction,
    Holiday,
    Other(String),
}

pub trait AbsenceStorage {
    type Error;

    fn store_absence(&self, record: AbsenceRecord) -> Result<(), Self::Error>;
    fn get_absences(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<HashMap<NaiveDate, AbsenceRecord>, Self::Error>;
    fn get_absence(&self, date: NaiveDate) -> Result<Option<AbsenceRecord>, Self::Error>;
    fn delete_absence(&self, date: NaiveDate) -> Result<bool, Self::Error>;
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
        hours: f32,
        note: Option<String>,
    ) {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let record = AbsenceRecord {
            date,
            hours,
            absence_type,
            note,
        };

        let json = serde_json::to_string_pretty(&record).unwrap();
        println!("AbsenceRecord:\n{}\n", json);

        // Test round-trip deserialization
        let deserialized: AbsenceRecord = serde_json::from_str(&json).unwrap();
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
