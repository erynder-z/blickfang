use crate::models::config::Config;
use serde_json;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// Returns the path to the configuration file as a `PathBuf`.
///
/// The path is determined by creating a directory named "blickfang" in the application data directory,
/// and then appending "config.json" to it. If the directory or file cannot be created,
/// an error string is returned.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// A `Result` containing the path to the configuration file as a `PathBuf`, or an error string if the path cannot be created.
///
/// # Errors
///
/// If the application data directory cannot be obtained, or if the "blickfang" directory or "config.json" file cannot be created, an error string is returned.
fn get_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut config_path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    config_path.push("blickfang");
    fs::create_dir_all(&config_path)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    config_path.push("config.json");
    Ok(config_path)
}

/// Reads the application configuration from the config file.
/// If the configuration file does not exist, a default configuration is created and written.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<String, String>` - A JSON string representation of the configuration.
pub fn read_config(app: &AppHandle) -> Result<String, String> {
    let config_path = get_config_path(app)?;

    if !config_path.exists() {
        let default_config = Config::default();
        let default_str = serde_json::to_string_pretty(&default_config)
            .map_err(|e| format!("Failed to serialize default config: {}", e))?;
        fs::write(&config_path, &default_str)
            .map_err(|e| format!("Failed to write default config file: {}", e))?;
        return Ok(default_str);
    }

    let raw = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: Config = serde_json::from_str(&raw).unwrap_or_else(|_| Config::default());

    let normalized = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to reserialize normalized config: {}", e))?;

    if normalized != raw {
        let _ = fs::write(&config_path, &normalized);
    }

    Ok(normalized)
}

/// Writes the provided configuration content to the config file.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `content` - The configuration content as a string (expected to be JSON).
///
/// # Returns
/// `Result<(), String>`.
pub fn write_config(app: &AppHandle, content: &str) -> Result<(), String> {
    let config_path = get_config_path(app)?;
    fs::write(config_path, content).map_err(|e| format!("Failed to write config file: {}", e))?;
    Ok(())
}
