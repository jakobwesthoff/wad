pub mod absence;

pub use absence::*;

pub trait WadDataStore: AbsenceStorage {
    fn open() -> Result<Self, Self::Error>
    where
        Self: Sized;
}
