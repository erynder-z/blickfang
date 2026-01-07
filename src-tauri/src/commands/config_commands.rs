use crate::models::config::{default_shortcuts, Config, Shortcuts};
use crate::utils::config_utils::{read_config, write_config};
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

/// Reads the current application configuration.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<String, String>` - A JSON string representation of the configuration.
#[tauri::command]
pub fn read_config_command(app: AppHandle) -> Result<String, String> {
    read_config(&app)
}

/// Writes a new configuration to the application's config file.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `content` - The new configuration content as a JSON string.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn write_config_command(app: AppHandle, content: String) -> Result<(), String> {
    write_config(&app, &content)
}

/// Updates the application's language setting.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `language` - The new language code (e.g., "en", "de").
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn update_language_command(app: AppHandle, language: String) -> Result<(), String> {
    update_config(&app, |config| config.language = language)
}

/// Updates the application's theme setting.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `theme` - The new theme name.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn update_theme_command(app: AppHandle, theme: String) -> Result<(), String> {
    update_config(&app, |config| config.theme = theme)
}

/// Updates the size of toolbar buttons.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `toolbar_button_size` - The new size setting for toolbar buttons.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn update_toolbar_button_size_command(
    app: AppHandle,
    toolbar_button_size: String,
) -> Result<(), String> {
    update_config(&app, |config| {
        config.toolbar_button_size = toolbar_button_size
    })
}

/// Updates the image name display mode.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `mode` - The new display mode.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn update_image_name_display_mode_command(app: AppHandle, mode: String) -> Result<(), String> {
    update_config(&app, |config| config.image_name_display_mode = mode)
}

/// Retrieves the default keyboard shortcuts.
///
/// # Returns
/// `Result<Shortcuts, String>` - The default shortcuts.
#[tauri::command]
pub fn get_default_shortcuts_command() -> Result<Shortcuts, String> {
    Ok(default_shortcuts())
}

/// Updates the custom keyboard shortcuts.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `new_shortcuts` - The new custom shortcuts.
///
/// # Returns
/// `Result<(), String>`.
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

/// Sets the active shortcuts to the default configuration.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn set_active_shortcuts_to_default(app: AppHandle) -> Result<(), String> {
    update_config(&app, |config| config.shortcuts = default_shortcuts())
}

/// Sets the active shortcuts to the custom configuration.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn set_active_shortcuts_to_custom(app: AppHandle) -> Result<(), String> {
    update_config(&app, |config| {
        config.shortcuts = config.custom_shortcuts.clone()
    })
}

/// Updates the visibility setting for edge indicators.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `visible` - Whether edge indicators should be visible.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn update_edge_indicators_visible_command(app: AppHandle, visible: bool) -> Result<(), String> {
    update_config(&app, |config| config.edge_indicators_visible = visible)
}

/// Updates the setting for remembering window size and position.
/// If `remember` is true, it captures the current window size and position.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `remember` - Whether to remember window size/position.
///
/// # Returns
/// `Result<(), String>`.
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

/// Checks if the initial application settings have been configured.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<bool, String>` - `true` if initial settings are configured, `false` otherwise.
#[tauri::command]
pub fn get_has_configured_initial_settings_command(app: AppHandle) -> Result<bool, String> {
    let config_str = read_config(&app)?;
    let config: Config = serde_json::from_str(&config_str)
        .map_err(|e| format!("Failed to deserialize config: {}", e))?;
    Ok(config.has_configured_initial_settings)
}

/// Sets the flag indicating whether initial application settings have been configured.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `value` - The new value for the flag.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn set_has_configured_initial_settings_command(
    app: AppHandle,
    value: bool,
) -> Result<(), String> {
    update_config(&app, |config| {
        config.has_configured_initial_settings = value
    })
}

/// Sets the user's choice for Linux desktop file installation.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `choice` - The user's choice ("installed", "skipped", or "not_asked").
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn set_linux_desktop_install_choice_command(
    app: AppHandle,
    choice: String,
) -> Result<(), String> {
    update_config(&app, |config| config.linux_desktop_install_choice = choice)
}

/// Gets the user's choice for Linux desktop file installation.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<String, String>` - The user's choice.
#[tauri::command]
pub fn get_linux_desktop_install_choice_command(app: AppHandle) -> Result<String, String> {
    let config_str = read_config(&app)?;
    let config: Config = serde_json::from_str(&config_str)
        .map_err(|e| format!("Failed to deserialize config: {}", e))?;
    Ok(config.linux_desktop_install_choice)
}

/// Updates the ASCII character set used for ASCII art conversion.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `ascii_chars` - The new ASCII character set name.
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn update_ascii_chars_command(app: AppHandle, ascii_chars: String) -> Result<(), String> {
    update_config(&app, |config| config.ascii_chars = ascii_chars)
}

/// Updates the background color used for ASCII art conversion.
///
/// # Arguments
/// * `app` - The Tauri application handle.
/// * `background_color` - The new background color as a hex string (e.g., "#000000").
///
/// # Returns
/// `Result<(), String>`.
#[tauri::command]
pub fn update_ascii_background_color_command(
    app: AppHandle,
    background_color: String,
) -> Result<(), String> {
    update_config(&app, |config| {
        config.ascii_background_color = background_color
    })
}
