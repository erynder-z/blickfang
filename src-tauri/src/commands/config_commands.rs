use crate::utils::config_utils::{read_config, write_config};
use tauri::AppHandle;

#[tauri::command]
pub fn read_config_command(app: AppHandle) -> String {
    read_config(&app)
}

#[tauri::command]
pub fn write_config_command(app: AppHandle, content: String) {
    write_config(&app, &content);
}
