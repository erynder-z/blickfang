use crate::utils::config_utils::{read_config, write_config, Config};
use serde_json;
use std::error::Error;
use tauri::{App, AppHandle, Manager, PhysicalPosition, PhysicalSize};

pub fn setup_main_window(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = app.get_webview_window("main").unwrap();
    let app_handle = app.handle().clone();
    let config_str = read_config(&app_handle);
    if let Ok(config) = serde_json::from_str::<Config>(&config_str) {
        if config.remember_window_size {
            if let (Some(w), Some(h), Some(x), Some(y)) = (
                config.window_width,
                config.window_height,
                config.window_x,
                config.window_y,
            ) {
                window.set_size(PhysicalSize::new(w as u32, h as u32))?;
                window.set_position(PhysicalPosition::new(x as i32, y as i32))?;
                return Ok(());
            }
        }
    }

    if let Some(monitor) = window.current_monitor()? {
        let monitor_size = monitor.size();
        let mut new_height = (monitor_size.height as f64 * 0.8) as u32;
        let mut new_width = (new_height as f64 * 4.0 / 3.0) as u32;

        if new_width > monitor_size.width {
            new_width = (monitor_size.width as f64 * 0.8) as u32;
            new_height = (new_width as f64 * 3.0 / 4.0) as u32;
        }

        window.set_size(PhysicalSize::new(new_width, new_height))?;
        window.set_position(PhysicalPosition::new(50, 50))?;
    }
    Ok(())
}

#[tauri::command]
pub fn show_window(window: tauri::Window) {
    window.show().unwrap();
}

pub fn handle_window_event(app_handle: &AppHandle, event: &tauri::RunEvent) {
    if let tauri::RunEvent::WindowEvent {
        label,
        event: tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_),
        ..
    } = event
    {
        if label == "main" {
            let window = app_handle.get_webview_window("main").unwrap();
            let config_str = read_config(app_handle);
            if let Ok(mut config) = serde_json::from_str::<Config>(&config_str) {
                if config.remember_window_size {
                    if let Ok(size) = window.inner_size() {
                        config.window_width = Some(size.width as f64);
                        config.window_height = Some(size.height as f64);
                    }
                    if let Ok(position) = window.inner_position() {
                        config.window_x = Some(position.x as f64);
                        config.window_y = Some(position.y as f64);
                    }
                    if let Ok(new_config_str) = serde_json::to_string(&config) {
                        write_config(app_handle, &new_config_str);
                    }
                }
            }
        }
    }
}
