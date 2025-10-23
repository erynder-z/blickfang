use crate::utils::{
    startup_handler::{AppReady, OpenedPathsState},
    window_utils::setup_main_window,
};

use tauri_plugin_dialog;
use tauri_plugin_fs;
use tauri_plugin_opener;

mod commands;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .manage(OpenedPathsState::default())
        .manage(AppReady::default())
        .setup(|app| {
            setup_main_window(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            utils::window_utils::show_window,
            utils::startup_handler::frontend_is_ready,
            commands::file_operations::open_and_read_file,
            commands::file_operations::read_image_file,
            commands::file_operations::change_image,
            commands::config_commands::read_config_command,
            commands::config_commands::write_config_command,
            commands::config_commands::update_language_command,
            commands::config_commands::update_theme_command,
            commands::config_commands::update_button_size_command,
            commands::config_commands::get_default_shortcuts_command,
            commands::config_commands::update_custom_shortcuts_command,
            commands::config_commands::set_active_shortcuts_to_default,
            commands::config_commands::set_active_shortcuts_to_custom,
            commands::config_commands::update_image_name_display_mode_command,
            commands::config_commands::update_edge_indicators_visible_command,
            commands::config_commands::update_remember_window_size_command
        ])
        .build(context)
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            utils::startup_handler::handle_run_event(app_handle, &event);
            utils::window_utils::handle_window_event(app_handle, &event);
        });
}
