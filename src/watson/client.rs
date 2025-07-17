use super::error::WatsonError;
use std::process::Command;
use which::which;

#[derive(Debug, Clone, PartialEq)]
pub struct WatsonVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl WatsonVersion {
    pub fn parse(version_string: &str) -> Result<Self, WatsonError> {
        // Expected format: "Watson, version 2.1.0"
        let version_part = version_string
            .strip_prefix("Watson, version ")
            .ok_or_else(|| {
                WatsonError::VersionParseError(format!(
                    "Invalid version format: {}",
                    version_string
                ))
            })?;

        let parts: Vec<&str> = version_part.split('.').collect();
        if parts.len() != 3 {
            return Err(WatsonError::VersionParseError(format!(
                "Expected 3 version parts, got {}",
                parts.len()
            )));
        }

        let major = parts[0].parse::<u32>().map_err(|_| {
            WatsonError::VersionParseError(format!("Invalid major version: {}", parts[0]))
        })?;

        let minor = parts[1].parse::<u32>().map_err(|_| {
            WatsonError::VersionParseError(format!("Invalid minor version: {}", parts[1]))
        })?;

        let patch = parts[2].parse::<u32>().map_err(|_| {
            WatsonError::VersionParseError(format!("Invalid patch version: {}", parts[2]))
        })?;

        Ok(WatsonVersion {
            major,
            minor,
            patch,
        })
    }
}

pub struct WatsonClient;

impl WatsonClient {
    pub fn new() -> Self {
        WatsonClient
    }

    pub fn is_usable(&self) -> bool {
        match Command::new("watson").arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    let version_string = String::from_utf8_lossy(&output.stdout);
                    version_string.starts_with("Watson, version ")
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    pub fn get_version(&self) -> Result<WatsonVersion, WatsonError> {
        let output = Command::new("watson")
            .arg("--version")
            .output()
            .map_err(|_| WatsonError::CommandNotFound)?;

        if !output.status.success() {
            return Err(WatsonError::CommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let version_string = String::from_utf8_lossy(&output.stdout);
        WatsonVersion::parse(version_string.trim())
    }

    pub fn get_path(&self) -> Result<String, WatsonError> {
        let path = which("watson").map_err(|_| WatsonError::CommandNotFound)?;

        Ok(path.to_string_lossy().to_string())
    }
}
