use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use tempfile::NamedTempFile;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("Failed to create temporary file: {0}")]
    TempFile(std::io::Error),
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Editor execution failed: {0}")]
    EditorExecution(std::io::Error),
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("No changes detected")]
    NoChanges,
}

pub trait EditableDocument: Serialize + DeserializeOwned + Clone + PartialEq {
    fn validate(&self, original: &Self) -> Result<(), String>;
}

pub struct EditorSession<T> {
    original: T,
}

impl<T> EditorSession<T>
where
    T: EditableDocument,
{
    pub fn new(original: T) -> Self {
        Self { original }
    }

    pub fn edit(&self) -> Result<T, EditorError> {
        // Serialize the original to pretty JSON
        let json_content = serde_json::to_string_pretty(&self.original)?;

        // Create temporary file with .json extension for syntax highlighting
        let mut temp_file = NamedTempFile::with_suffix(".json").map_err(EditorError::TempFile)?;
        temp_file
            .write_all(json_content.as_bytes())
            .map_err(EditorError::TempFile)?;
        temp_file.flush().map_err(EditorError::TempFile)?;

        // Open the temporary file in editor
        edit::edit_file(temp_file.path()).map_err(EditorError::EditorExecution)?;

        // Read the edited content back from the filesystem
        // (can't use temp_file.reopen() because editors often replace the file)
        let edited_content =
            std::fs::read_to_string(temp_file.path()).map_err(EditorError::TempFile)?;

        // Parse the edited content
        let edited: T = serde_json::from_str(&edited_content)?;

        // Check if anything changed
        if edited == self.original {
            return Err(EditorError::NoChanges);
        }

        // Validate the edited document
        edited
            .validate(&self.original)
            .map_err(EditorError::Validation)?;

        Ok(edited)
    }
}
