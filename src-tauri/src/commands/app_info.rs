use tauri::AppHandle;

/// Gets the application version from the Tauri context.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `String` - The application version.
#[tauri::command]
pub fn get_app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}