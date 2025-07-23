use anyhow::Result;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigFileError {
    #[error("Failed to access config directory: {0}")]
    ConfigDirAccess(String),
    #[error("Failed to create config directory: {0}")]
    ConfigDirCreation(std::io::Error),
    #[error("Failed to read config file: {0}")]
    ConfigFileRead(ConfigError),
    #[error("Failed to write config file: {0}")]
    ConfigFileWrite(std::io::Error),
    #[error("Failed to serialize config: {0}")]
    ConfigSerialization(toml::ser::Error),
    #[error("Invalid value for {key}: {value}")]
    InvalidValue { key: String, value: String },
    #[error("Unknown config key: {0}")]
    UnknownKey(String),
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub workhours_per_week: f64,
    pub daily_worktime_low: f64,
    pub daily_worktime_medium: f64,
    pub daily_worktime_good: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workhours_per_week: 40.0,
            daily_worktime_low: 0.0,
            daily_worktime_medium: 4.0,
            daily_worktime_good: 8.0,
        }
    }
}

impl Config {
    /// Open configuration by loading from file and ensuring it's up-to-date
    /// Creates config file with defaults if missing, and updates existing files with missing fields
    pub fn open() -> Result<Self, ConfigFileError> {
        let settings = ConfigBuilder::builder()
            .add_source(File::from(Self::config_file_path()?).required(false))
            .add_source(Environment::with_prefix("WAD"))
            .build()
            .map_err(ConfigFileError::ConfigFileRead)?;

        let config: Config = settings
            .try_deserialize()
            .unwrap_or_else(|_| Config::default());

        // Always save config to ensure file exists and contains all current fields
        config.save()?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), ConfigFileError> {
        let config_dir = Self::config_dir()?;
        fs::create_dir_all(&config_dir).map_err(ConfigFileError::ConfigDirCreation)?;

        let config_file_path = Self::config_file_path()?;
        let toml_content = toml::to_string(self).map_err(ConfigFileError::ConfigSerialization)?;

        fs::write(config_file_path, toml_content).map_err(ConfigFileError::ConfigFileWrite)?;

        Ok(())
    }

    /// Get the platform-specific config directory path
    pub fn config_dir() -> Result<PathBuf, ConfigFileError> {
        dirs::config_dir()
            .ok_or_else(|| {
                ConfigFileError::ConfigDirAccess("Could not determine config directory".to_string())
            })
            .map(|dir| dir.join("wad"))
    }

    /// Get the full path to the config file
    pub fn config_file_path() -> Result<PathBuf, ConfigFileError> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Get a configuration value by key name
    pub fn get_value(&self, key: &str) -> Option<String> {
        let value = serde_json::to_value(self).ok()?;
        value.get(key).map(|v| match v {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            _ => v.to_string(),
        })
    }

    /// Set a configuration value by key name
    pub fn set_value(&mut self, key: &str, value_str: &str) -> Result<(), ConfigFileError> {
        let current_value = serde_json::to_value(&self)?;
        let mut map = current_value.as_object().unwrap().clone();

        // Check the existing field's type to preserve it
        let json_value = match map.get(key) {
            Some(serde_json::Value::Bool(_)) => {
                serde_json::Value::Bool(value_str.parse().map_err(|_| {
                    ConfigFileError::InvalidValue {
                        key: key.to_string(),
                        value: value_str.to_string(),
                    }
                })?)
            }
            Some(serde_json::Value::Number(_)) => {
                let f: f64 = value_str
                    .parse()
                    .map_err(|_| ConfigFileError::InvalidValue {
                        key: key.to_string(),
                        value: value_str.to_string(),
                    })?;
                serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap())
            }
            Some(serde_json::Value::String(_)) => serde_json::Value::String(value_str.to_string()),
            None => return Err(ConfigFileError::UnknownKey(key.to_string())),
            _ => {
                return Err(ConfigFileError::InvalidValue {
                    key: key.to_string(),
                    value: value_str.to_string(),
                });
            }
        };

        map.insert(key.to_string(), json_value);
        *self = serde_json::from_value(serde_json::Value::Object(map))?;
        Ok(())
    }

    /// Get all available configuration keys with their current values
    pub fn list_values(&self) -> Vec<(String, String)> {
        let value = serde_json::to_value(self).unwrap();
        value
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => v.to_string(),
                    },
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_config_get_set_operations() {
        let mut config = Config::default();

        // Test getting existing values
        assert_eq!(
            config.get_value("workhours_per_week"),
            Some("40.0".to_string())
        );
        assert_eq!(
            config.get_value("daily_worktime_low"),
            Some("0.0".to_string())
        );
        assert_eq!(
            config.get_value("daily_worktime_medium"),
            Some("4.0".to_string())
        );
        assert_eq!(
            config.get_value("daily_worktime_good"),
            Some("8.0".to_string())
        );

        // Test getting non-existent value
        assert_eq!(config.get_value("nonexistent"), None);

        // Test setting existing values with correct types
        assert!(config.set_value("workhours_per_week", "37.5").is_ok());
        assert_eq!(
            config.get_value("workhours_per_week"),
            Some("37.5".to_string())
        );

        assert!(config.set_value("daily_worktime_medium", "5.5").is_ok());
        assert_eq!(
            config.get_value("daily_worktime_medium"),
            Some("5.5".to_string())
        );

        // Test setting with wrong types
        assert!(
            config
                .set_value("workhours_per_week", "not_a_number")
                .is_err()
        );
        assert!(
            config
                .set_value("daily_worktime_good", "not_a_number")
                .is_err()
        );

        // Test setting non-existent key
        assert!(config.set_value("nonexistent", "value").is_err());
    }

    #[test]
    fn test_list_values_completeness() {
        let config = Config::default();
        let values = config.list_values();

        // Should contain all fields
        let keys: Vec<String> = values.iter().map(|(k, _)| k.clone()).collect();
        assert!(keys.contains(&"workhours_per_week".to_string()));
        assert!(keys.contains(&"daily_worktime_low".to_string()));
        assert!(keys.contains(&"daily_worktime_medium".to_string()));
        assert!(keys.contains(&"daily_worktime_good".to_string()));
        assert_eq!(keys.len(), 4); // Should have exactly 4 fields

        // Check default values
        let values_map: HashMap<String, String> = values.into_iter().collect();
        assert_eq!(
            values_map.get("workhours_per_week"),
            Some(&"40.0".to_string())
        );
        assert_eq!(
            values_map.get("daily_worktime_low"),
            Some(&"0.0".to_string())
        );
        assert_eq!(
            values_map.get("daily_worktime_medium"),
            Some(&"4.0".to_string())
        );
        assert_eq!(
            values_map.get("daily_worktime_good"),
            Some(&"8.0".to_string())
        );
    }

    #[test]
    fn test_partial_config_deserialization() {
        // Test that serde fills in missing fields with defaults
        let partial_toml = "workhours_per_week = 35.0";

        let config: Config = toml::from_str(partial_toml).unwrap();
        assert_eq!(config.workhours_per_week, 35.0);
        // Missing fields should use defaults
        assert_eq!(config.daily_worktime_low, 0.0);
        assert_eq!(config.daily_worktime_medium, 4.0);
        assert_eq!(config.daily_worktime_good, 8.0);
    }
}
