use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_language() -> String {
    "en".to_string()
}

fn default_theme() -> String {
    "default".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: default_language(),
            theme: default_theme(),
        }
    }
}

fn get_config_path(app: &AppHandle) -> PathBuf {
    let mut config_path = app.path().app_data_dir().unwrap();
    config_path.push("simple-image-viewer");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("config.json");
    config_path
}

pub fn read_config(app: &AppHandle) -> String {
    let config_path = get_config_path(app);
    if !config_path.exists() {
        let default_config = Config::default();
        let default_config_str = serde_json::to_string(&default_config).unwrap();
        fs::write(&config_path, default_config_str).unwrap();
    }
    fs::read_to_string(config_path).unwrap_or_else(|_| "{}".to_string())
}

pub fn write_config(app: &AppHandle, content: &str) {
    let config_path = get_config_path(app);
    fs::write(config_path, content).unwrap();
}
