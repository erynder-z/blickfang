use crate::utils::{
    os_specific_setup::perform_os_specific_setup,
    startup_handler::{AppReady, OpenedPathsState},
    window_utils::setup_main_window,
};

use tauri_plugin_dialog;
use tauri_plugin_fs;
use tauri_plugin_opener;

mod commands;
mod models;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .manage(OpenedPathsState::default())
        .manage(AppReady::default())
        .setup(|app| {
            perform_os_specific_setup(&app.handle())?;
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
            commands::file_operations::read_image_from_path,
            commands::file_operations::change_image,
            commands::file_operations::save_base64_image_as,
            commands::file_operations::get_supported_image_formats,
            commands::config_commands::read_config_command,
            commands::config_commands::write_config_command,
            commands::config_commands::update_language_command,
            commands::config_commands::update_theme_command,
            commands::config_commands::update_toolbar_button_size_command,
            commands::config_commands::get_default_shortcuts_command,
            commands::config_commands::update_custom_shortcuts_command,
            commands::config_commands::set_active_shortcuts_to_default,
            commands::config_commands::set_active_shortcuts_to_custom,
            commands::config_commands::update_image_name_display_mode_command,
            commands::config_commands::update_edge_indicators_visible_command,
            commands::config_commands::update_remember_window_size_command,
            commands::config_commands::get_has_configured_initial_settings_command,
            commands::config_commands::set_has_configured_initial_settings_command,
            commands::config_commands::set_linux_desktop_install_choice_command,
            commands::config_commands::get_linux_desktop_install_choice_command,
            commands::config_commands::update_ascii_chars_command,
            commands::config_commands::update_ascii_background_color_command,
            commands::linux_integration::install_linux_desktop_file_command,
            commands::linux_integration::is_running_as_appimage_command,
            commands::image_analyze::detect_ai_image,
            commands::ascii_art::convert_image_to_ascii_art,
            commands::ascii_art::get_available_ascii_char_sets
        ])
        .build(context)
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            utils::startup_handler::handle_run_event(app_handle, &event);
            utils::window_utils::handle_window_event(app_handle, &event);
        });
}
