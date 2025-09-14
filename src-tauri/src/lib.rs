use tauri_plugin_opener;
use tauri_plugin_dialog;

mod commands;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::file_operations::open_and_read_file,
            commands::file_operations::read_image_file,
            commands::file_operations::change_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}