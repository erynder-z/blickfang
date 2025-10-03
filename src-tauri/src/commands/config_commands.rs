use crate::utils::config_utils::{read_config, write_config, Config};
use serde_json;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn read_config_command(app: AppHandle) -> String {
    read_config(&app)
}

#[tauri::command]
pub fn write_config_command(app: AppHandle, content: String) {
    write_config(&app, &content);
}

#[tauri::command]
pub fn update_language_command(app: AppHandle, language: String) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.language = language;
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}
