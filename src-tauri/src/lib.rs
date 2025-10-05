use tauri_plugin_dialog;
use tauri_plugin_opener;

mod commands;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
