use thiserror::Error;

#[derive(Error, Debug)]
pub enum WatsonError {
    #[error("Watson command not found - please install Watson CLI")]
    CommandNotFound,

    #[error("Watson command failed: {0}")]
    CommandFailed(String),

    #[error("Failed to parse Watson version: {0}")]
    VersionParseError(String),
}
