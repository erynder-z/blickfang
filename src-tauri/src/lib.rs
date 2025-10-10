use tauri::{Manager, PhysicalPosition, PhysicalSize};
use tauri_plugin_dialog;
use tauri_plugin_opener;

mod commands;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
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
                window.show()?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::file_operations::open_and_read_file,
            commands::file_operations::read_image_file,
            commands::file_operations::change_image,
            commands::config_commands::read_config_command,
            commands::config_commands::write_config_command,
            commands::config_commands::update_language_command,
            commands::config_commands::update_theme_command,
            commands::config_commands::get_default_shortcuts_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
