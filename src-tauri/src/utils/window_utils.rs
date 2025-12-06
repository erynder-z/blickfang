use crate::models::config::Config;
use crate::utils::config_utils::{read_config, write_config};
use serde_json;
use tauri::{App, AppHandle, Manager, PhysicalPosition, PhysicalSize, RunEvent, WindowEvent};

fn load_config(app_handle: &AppHandle) -> Result<Config, String> {
    let config_str = read_config(app_handle)?;
    serde_json::from_str(&config_str).map_err(|e| format!("Failed to deserialize config: {}", e))
}

fn save_config(app_handle: &AppHandle, config: &Config) {
    if let Ok(json) = serde_json::to_string(config) {
        if let Err(e) = write_config(app_handle, &json) {
            eprintln!("Failed to write config: {e}");
        }
    } else {
        eprintln!("Failed to serialize config for saving.");
    }
}

/// Configures the main application window's size and position based on saved settings
/// in the config file, or calculates a default size/position if no settings are found
/// or `remember_window_size` is false.
///
/// # Arguments
/// * `app` - A mutable reference to the Tauri `App` instance.
///
/// # Returns
/// `Result<(), String>`.
pub fn setup_main_window(app: &mut App) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    let config = load_config(&app.handle())?;

    if config.remember_window_size {
        if let (Some(w), Some(h), Some(x), Some(y)) = (
            config.window_width,
            config.window_height,
            config.window_x,
            config.window_y,
        ) {
            window
                .set_size(PhysicalSize::new(w as u32, h as u32))
                .map_err(|e| format!("Failed to set window size: {e}"))?;
            window
                .set_position(PhysicalPosition::new(x as i32, y as i32))
                .map_err(|e| format!("Failed to set window position: {e}"))?;
            return Ok(());
        }
    }

    if let Some(monitor) = window.current_monitor().map_err(|e| e.to_string())? {
        let monitor_size = monitor.size();
        let mut new_height = (monitor_size.height as f64 * 0.8) as u32;
        let mut new_width = (new_height as f64 * 4.0 / 3.0) as u32;

        if new_width > monitor_size.width {
            new_width = (monitor_size.width as f64 * 0.8) as u32;
            new_height = (new_width as f64 * 3.0 / 4.0) as u32;
        }

        window
            .set_size(PhysicalSize::new(new_width, new_height))
            .map_err(|e| format!("Failed to set default window size: {e}"))?;
        window
            .set_position(PhysicalPosition::new(50, 50))
            .map_err(|e| format!("Failed to set default window position: {e}"))?;
    }

    Ok(())
}

/// Shows the specified Tauri window.
///
/// # Arguments
/// * `window` - The Tauri `Window` to show.
#[tauri::command]
pub fn show_window(window: tauri::Window) {
    if let Err(e) = window.show() {
        eprintln!("Failed to show window: {e}");
    }
}

/// Handles Tauri window events, specifically `Moved` and `Resized` for the main window.
/// If `remember_window_size` is enabled in the config, it saves the new position and size.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle.
/// * `event` - The `RunEvent` to handle.
pub fn handle_window_event(app_handle: &AppHandle, event: &RunEvent) {
    if let RunEvent::WindowEvent {
        label,
        event: WindowEvent::Moved(_) | WindowEvent::Resized(_),
        ..
    } = event
    {
        if label == "main" {
            if let Some(window) = app_handle.get_webview_window("main") {
                if let Ok(mut config) = load_config(app_handle) {
                    if config.remember_window_size {
                        if let Ok(size) = window.inner_size() {
                            config.window_width = Some(size.width as f64);
                            config.window_height = Some(size.height as f64);
                        }
                        if let Ok(pos) = window.outer_position() {
                            config.window_x = Some(pos.x as f64);
                            config.window_y = Some(pos.y as f64);
                        }
                        save_config(app_handle, &config);
                    }
                }
            }
        }
    }
}
