use std::error::Error;
use tauri::{App, Manager, PhysicalPosition, PhysicalSize};

pub fn setup_main_window(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = app.get_webview_window("main").unwrap();
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
