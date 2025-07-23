pub mod absence;
pub mod json_store;

pub use absence::*;
pub use json_store::*;

pub trait WadDataStore: AbsenceStorage {
    fn open() -> Result<Self, Self::Error>
    where
        Self: Sized;
}
