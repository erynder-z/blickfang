use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

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
        let default_config = "{\"theme\": \"light\"}"; // Dummy config
        fs::write(&config_path, default_config).unwrap();
    }
    fs::read_to_string(config_path).unwrap_or_else(|_| "{}".to_string())
}

pub fn write_config(app: &AppHandle, content: &str) {
    let config_path = get_config_path(app);
    fs::write(config_path, content).unwrap();
}
