use crate::utils::config_utils::{default_shortcuts, read_config, write_config, Config, Shortcuts};
use serde_json;
use tauri::{AppHandle, Emitter, Manager};

fn update_config<F>(app: &AppHandle, updater: F) -> Result<(), String>
where
    F: FnOnce(&mut Config),
{
    let config_str = read_config(app)?;
    let mut config: Config = serde_json::from_str(&config_str)
        .map_err(|e| format!("Failed to deserialize config: {}", e))?;

    updater(&mut config);

    let new_config_str =
        serde_json::to_string(&config).map_err(|e| format!("Failed to serialize config: {}", e))?;

    write_config(app, &new_config_str)?;
    app.emit("config-updated", &config)
        .map_err(|e| format!("Failed to emit config-updated event: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn read_config_command(app: AppHandle) -> Result<String, String> {
    read_config(&app)
}

#[tauri::command]
pub fn write_config_command(app: AppHandle, content: String) -> Result<(), String> {
    write_config(&app, &content)
}

#[tauri::command]
pub fn update_language_command(app: AppHandle, language: String) -> Result<(), String> {
    update_config(&app, |config| config.language = language)
}

#[tauri::command]
pub fn update_theme_command(app: AppHandle, theme: String) -> Result<(), String> {
    update_config(&app, |config| config.theme = theme)
}

#[tauri::command]
pub fn update_toolbar_button_size_command(
    app: AppHandle,
    toolbar_button_size: String,
) -> Result<(), String> {
    update_config(&app, |config| {
        config.toolbar_button_size = toolbar_button_size
    })
}

#[tauri::command]
pub fn update_image_name_display_mode_command(app: AppHandle, mode: String) -> Result<(), String> {
    update_config(&app, |config| config.image_name_display_mode = mode)
}

#[tauri::command]
pub fn get_default_shortcuts_command() -> Result<Shortcuts, String> {
    Ok(default_shortcuts())
}

#[tauri::command]
pub fn update_custom_shortcuts_command(
    app: AppHandle,
    new_shortcuts: Shortcuts,
) -> Result<(), String> {
    update_config(&app, |config| {
        config.shortcuts = new_shortcuts.clone();
        config.custom_shortcuts = new_shortcuts;
    })
}

#[tauri::command]
pub fn set_active_shortcuts_to_default(app: AppHandle) -> Result<(), String> {
    update_config(&app, |config| config.shortcuts = default_shortcuts())
}

#[tauri::command]
pub fn set_active_shortcuts_to_custom(app: AppHandle) -> Result<(), String> {
    update_config(&app, |config| {
        config.shortcuts = config.custom_shortcuts.clone()
    })
}

#[tauri::command]
pub fn update_edge_indicators_visible_command(app: AppHandle, visible: bool) -> Result<(), String> {
    update_config(&app, |config| config.edge_indicators_visible = visible)
}

#[tauri::command]
pub fn update_remember_window_size_command(app: AppHandle, remember: bool) -> Result<(), String> {
    update_config(&app, |config| {
        config.remember_window_size = remember;

        if remember {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(size) = window.inner_size() {
                    config.window_width = Some(size.width as f64);
                    config.window_height = Some(size.height as f64);
                }
                if let Ok(pos) = window.inner_position() {
                    config.window_x = Some(pos.x as f64);
                    config.window_y = Some(pos.y as f64);
                }
            }
        } else {
            config.window_width = None;
            config.window_height = None;
            config.window_x = None;
            config.window_y = None;
        }
    })
}
