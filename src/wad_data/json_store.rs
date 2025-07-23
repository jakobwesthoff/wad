use std::fs;
use std::path::PathBuf;

use chrono::{Datelike, NaiveDate};
use thiserror::Error;
use ulid::Ulid;

use super::{AbsenceRecord, AbsenceStorage, WadDataStore};

#[derive(Error, Debug)]
pub enum JsonDataStoreError {
    #[error("No data directory available")]
    NoDataDir,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub struct JsonDataStore {
    data_dir: PathBuf,
}

impl JsonDataStore {
    pub fn absences_dir(&self) -> PathBuf {
        self.data_dir.join("absences")
    }

    fn year_dir(&self, year: i32) -> PathBuf {
        self.absences_dir().join(year.to_string())
    }

    fn absence_file_path(&self, date: NaiveDate) -> PathBuf {
        let year_dir = self.year_dir(date.year());
        let filename = format!("{}.json", date.format("%Y-%m-%d"));
        year_dir.join(filename)
    }

    fn load_absence_file(&self, date: NaiveDate) -> Result<Vec<AbsenceRecord>, JsonDataStoreError> {
        let file_path = self.absence_file_path(date);

        if !file_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(file_path)?;
        let records: Vec<AbsenceRecord> = serde_json::from_str(&content)?;
        Ok(records)
    }

    fn save_absence_file(
        &self,
        date: NaiveDate,
        records: &[AbsenceRecord],
    ) -> Result<(), JsonDataStoreError> {
        let year_dir = self.year_dir(date.year());
        fs::create_dir_all(&year_dir)?;

        let file_path = self.absence_file_path(date);
        let content = serde_json::to_string_pretty(records)?;
        fs::write(file_path, content)?;
        Ok(())
    }

    fn delete_absence_file(&self, date: NaiveDate) -> Result<bool, JsonDataStoreError> {
        let file_path = self.absence_file_path(date);

        if file_path.exists() {
            fs::remove_file(file_path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl WadDataStore for JsonDataStore {
    fn open() -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let data_dir = dirs::data_dir()
            .ok_or(JsonDataStoreError::NoDataDir)?
            .join("wad");

        fs::create_dir_all(&data_dir)?;

        Ok(Self { data_dir })
    }
}

impl AbsenceStorage for JsonDataStore {
    type Error = JsonDataStoreError;

    fn add_absence(&self, record: AbsenceRecord) -> Result<(), Self::Error> {
        let date = record.date;
        let mut records = self.load_absence_file(date)?;
        records.push(record);
        // Sort by ULID to maintain chronological order
        records.sort_by_key(|r| r.id);
        self.save_absence_file(date, &records)
    }

    fn get_absence(&self, date: NaiveDate) -> Result<Vec<AbsenceRecord>, Self::Error> {
        let mut records = self.load_absence_file(date)?;
        // Sort by ULID to maintain chronological order
        records.sort_by_key(|r| r.id);
        Ok(records)
    }

    fn remove_absence(&self, date: NaiveDate, id: Ulid) -> Result<bool, Self::Error> {
        let mut records = self.load_absence_file(date)?;
        let original_len = records.len();

        records.retain(|record| record.id != id);

        if records.is_empty() {
            // Remove file if no records left
            self.delete_absence_file(date)
        } else {
            // Save remaining records
            self.save_absence_file(date, &records)?;
            Ok(original_len != records.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use tempfile::TempDir;
    use ulid::Ulid;

    use crate::wad_data::{AbsenceRecord, AbsenceType};

    fn create_test_store() -> (JsonDataStore, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonDataStore {
            data_dir: temp_dir.path().to_path_buf(),
        };
        (store, temp_dir)
    }

    fn create_test_record(date_str: &str, absence_type: AbsenceType, hours: f64) -> AbsenceRecord {
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        AbsenceRecord {
            id: Ulid::new(),
            date,
            hours,
            absence_type,
            note: Some("Test record".to_string()),
        }
    }

    #[test]
    fn test_add_and_get_single_absence() {
        let (store, _temp_dir) = create_test_store();
        let record = create_test_record("2024-01-15", AbsenceType::Vacation, 8.0);
        let date = record.date;
        let id = record.id;

        // Add record
        store.add_absence(record.clone()).unwrap();

        // Retrieve and verify
        let retrieved = store.get_absence(date).unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].id, id);
        assert_eq!(retrieved[0].date, date);
        assert_eq!(retrieved[0].hours, 8.0);
    }

    #[test]
    fn test_add_multiple_absences_same_day() {
        let (store, _temp_dir) = create_test_store();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let record1 = AbsenceRecord {
            id: Ulid::new(),
            date,
            hours: 4.0,
            absence_type: AbsenceType::Sick,
            note: Some("Morning sick".to_string()),
        };

        let record2 = AbsenceRecord {
            id: Ulid::new(),
            date,
            hours: 4.0,
            absence_type: AbsenceType::Vacation,
            note: Some("Afternoon PTO".to_string()),
        };

        // Add both records
        store.add_absence(record1.clone()).unwrap();
        store.add_absence(record2.clone()).unwrap();

        // Retrieve and verify both are present and sorted by ULID
        let retrieved = store.get_absence(date).unwrap();
        assert_eq!(retrieved.len(), 2);

        // Should be ordered by ULID (chronologically)
        assert!(retrieved[0].id <= retrieved[1].id);
    }

    #[test]
    fn test_remove_absence() {
        let (store, _temp_dir) = create_test_store();
        let record = create_test_record("2024-01-15", AbsenceType::Vacation, 8.0);
        let date = record.date;
        let id = record.id;

        // Add record
        store.add_absence(record).unwrap();

        // Verify it exists
        let retrieved = store.get_absence(date).unwrap();
        assert_eq!(retrieved.len(), 1);

        // Remove it
        let removed = store.remove_absence(date, id).unwrap();
        assert!(removed);

        // Verify it's gone
        let retrieved = store.get_absence(date).unwrap();
        assert_eq!(retrieved.len(), 0);
    }

    #[test]
    fn test_remove_one_of_multiple_absences() {
        let (store, _temp_dir) = create_test_store();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let record1 = AbsenceRecord {
            id: Ulid::new(),
            date,
            hours: 4.0,
            absence_type: AbsenceType::Sick,
            note: Some("Morning".to_string()),
        };

        let record2 = AbsenceRecord {
            id: Ulid::new(),
            date,
            hours: 4.0,
            absence_type: AbsenceType::Vacation,
            note: Some("Afternoon".to_string()),
        };

        // Add both
        store.add_absence(record1.clone()).unwrap();
        store.add_absence(record2.clone()).unwrap();

        // Remove first one
        let removed = store.remove_absence(date, record1.id).unwrap();
        assert!(removed);

        // Verify only second remains
        let retrieved = store.get_absence(date).unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].id, record2.id);
    }

    #[test]
    fn test_get_absence_nonexistent_date() {
        let (store, _temp_dir) = create_test_store();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();

        let retrieved = store.get_absence(date).unwrap();
        assert_eq!(retrieved.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_absence() {
        let (store, _temp_dir) = create_test_store();
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let fake_id = Ulid::new();

        let removed = store.remove_absence(date, fake_id).unwrap();
        assert!(!removed);
    }

    #[test]
    fn test_year_directory_structure() {
        let (store, temp_dir) = create_test_store();

        // Add records in different years
        let record_2024 = create_test_record("2024-01-15", AbsenceType::Vacation, 8.0);
        let record_2025 = create_test_record("2025-01-15", AbsenceType::Sick, 4.0);

        store.add_absence(record_2024).unwrap();
        store.add_absence(record_2025).unwrap();

        // Verify directory structure
        let absences_dir = temp_dir.path().join("absences");
        assert!(absences_dir.join("2024").exists());
        assert!(absences_dir.join("2025").exists());
        assert!(absences_dir.join("2024").join("2024-01-15.json").exists());
        assert!(absences_dir.join("2025").join("2025-01-15.json").exists());
    }

    #[test]
    fn test_file_removed_when_no_records_left() {
        let (store, temp_dir) = create_test_store();
        let record = create_test_record("2024-01-15", AbsenceType::Vacation, 8.0);
        let date = record.date;
        let id = record.id;

        // Add record
        store.add_absence(record).unwrap();

        // Verify file exists
        let file_path = temp_dir
            .path()
            .join("absences")
            .join("2024")
            .join("2024-01-15.json");
        assert!(file_path.exists());

        // Remove record
        store.remove_absence(date, id).unwrap();

        // Verify file is deleted
        assert!(!file_path.exists());
    }
}
