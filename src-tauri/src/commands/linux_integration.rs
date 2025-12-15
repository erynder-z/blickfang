use crate::utils::os_integration_linux::install_desktop_file;
use tauri::AppHandle;

/// Installs the Linux desktop file for the application.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<(), String>` - Ok if installation succeeds, Err with error message otherwise.
#[tauri::command]
pub fn install_linux_desktop_file_command(_app: AppHandle) -> Result<(), String> {
    install_desktop_file().map_err(|e| format!("Failed to install desktop file: {}", e))
}

/// Checks if the application is running as an AppImage on Linux.
///
/// # Returns
/// `Result<bool, String>` - true if running as AppImage, false otherwise.
#[tauri::command]
pub fn is_running_as_appimage_command() -> Result<bool, String> {
    #[cfg(target_os = "linux")]
    {
        Ok(std::env::var("APPIMAGE").is_ok())
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok(false)
    }
}