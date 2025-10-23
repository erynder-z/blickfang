use crate::utils::config_utils::{default_shortcuts, read_config, write_config, Config, Shortcuts};
use serde_json;
use tauri::{AppHandle, Emitter, Manager};

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

#[tauri::command]
pub fn update_theme_command(app: AppHandle, theme: String) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.theme = theme;
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}

#[tauri::command]
pub fn update_button_size_command(app: AppHandle, button_size: String) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.button_size = button_size;
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}

#[tauri::command]
pub fn update_image_name_display_mode_command(app: AppHandle, mode: String) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.image_name_display_mode = mode;
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}

#[tauri::command]
pub fn get_default_shortcuts_command() -> Shortcuts {
    default_shortcuts()
}

#[tauri::command]
pub fn update_custom_shortcuts_command(app: AppHandle, new_shortcuts: Shortcuts) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.shortcuts = new_shortcuts.clone();
        config.custom_shortcuts = new_shortcuts;
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}

#[tauri::command]
pub fn set_active_shortcuts_to_default(app: AppHandle) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.shortcuts = default_shortcuts();
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}

#[tauri::command]
pub fn set_active_shortcuts_to_custom(app: AppHandle) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.shortcuts = config.custom_shortcuts.clone();
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}

#[tauri::command]
pub fn update_edge_indicators_visible_command(app: AppHandle, visible: bool) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.edge_indicators_visible = visible;
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}

#[tauri::command]
pub fn update_remember_window_size_command(app: AppHandle, remember: bool) {
    let config_str = read_config(&app);
    if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
        config.remember_window_size = remember;
        if remember {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(size) = window.inner_size() {
                    config.window_width = Some(size.width as f64);
                    config.window_height = Some(size.height as f64);
                }
                if let Ok(position) = window.inner_position() {
                    config.window_x = Some(position.x as f64);
                    config.window_y = Some(position.y as f64);
                }
            }
        } else {
            config.window_width = None;
            config.window_height = None;
            config.window_x = None;
            config.window_y = None;
        }
        if let Ok(new_config_str) = serde_json::to_string(&config) {
            write_config(&app, &new_config_str);
            app.emit("config-updated", &config).unwrap();
        }
    }
}
