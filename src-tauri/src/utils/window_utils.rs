use crate::models::config::Config;
use crate::utils::config_utils::{read_config, write_config};
use serde_json;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, RunEvent, WindowEvent};

/// Loads the application configuration from the config file.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle.
///
/// # Returns
/// `Result<Config, String>` - A `Config` object if the configuration is successfully loaded, an error string otherwise.
///
/// # Errors
///
/// If the configuration file cannot be read or deserialized, an error string is returned.
fn load_config(app_handle: &AppHandle) -> Result<Config, String> {
    let config_str = read_config(app_handle)?;
    serde_json::from_str(&config_str).map_err(|e| format!("Failed to deserialize config: {}", e))
}

/// Saves the provided configuration to the application's config file.
///
/// If the configuration cannot be serialized to JSON, an error message is printed.
/// If the configuration cannot be written to the config file, an error message is printed.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle.
/// * `config` - The configuration to be saved.
///
/// # Errors
///
/// If the configuration cannot be serialized or written, an error message is printed.
fn save_config(app_handle: &AppHandle, config: &Config) {
    if let Ok(json) = serde_json::to_string(config) {
        if let Err(e) = write_config(app_handle, &json) {
            eprintln!("Failed to write config: {e}");
        }
    } else {
        eprintln!("Failed to serialize config for saving.");
    }
}

/// Sets up the main window according to the application configuration.
///
/// If the configuration indicates that the window size should be remembered, it tries to set the window to the remembered size and position.
/// If the window size is not remembered, it tries to set a default size based on the monitor size.
///
/// # Arguments
///
/// * `app_handle` - The Tauri application handle.
///
/// # Returns
///
/// `Result<(), String>` - Ok if the window is successfully set up, an error string otherwise.
///
/// # Errors
///
/// If the configuration cannot be loaded, an error string is returned.
/// If the window cannot be set to the desired size or position, an error message is printed.
pub fn setup_main_window(app_handle: &AppHandle) -> Result<(), String> {
    let config = load_config(app_handle)?;

    if config.remember_window_size {
        if let Some(true) = config.window_maximized {
            let handle_for_call = app_handle.clone();
            let handle_for_closure = handle_for_call.clone();

            let _ = handle_for_call.run_on_main_thread(move || {
                // run on main thread in order to make sure the window is ready before doing any window operations
                if let Some(window) = handle_for_closure.get_webview_window("main") {
                    if let Err(e) = window.maximize() {
                        eprintln!("Failed to maximize window: {e}");
                    }
                }
            });

            return Ok(());
        }
    }

    let window = app_handle
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

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
/// If `remember_window_size` is enabled in the config, it saves the new position, size, and maximized state.
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

                        if let Ok(is_maximized) = window.is_maximized() {
                            config.window_maximized = Some(is_maximized);
                        }
                        save_config(app_handle, &config);
                    }
                }
            }
        }
    }
}
