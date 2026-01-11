use tauri::AppHandle;

/// Performs all necessary OS-specific setup tasks.
///
/// This function should be called during the Tauri app setup phase. It will
/// handle platform-specific operations like desktop file installation on Linux.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<(), String>` - Ok if setup succeeds, Err with error message otherwise.
pub fn perform_os_specific_setup(app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        perform_linux_setup(app)?;
    }

    #[cfg(target_os = "windows")]
    {
        perform_windows_setup(app)?;
    }

    #[cfg(target_os = "macos")]
    {
        perform_macos_setup(app)?;
    }

    Ok(())
}

/// Handles Linux-specific setup tasks.
///
/// On Linux, this checks the user's desktop installation preference and installs
/// the desktop file if appropriate.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<(), String>` - Ok if setup succeeds, Err with error message otherwise.
#[cfg(target_os = "linux")]
fn perform_linux_setup(app: &AppHandle) -> Result<(), String> {
    use crate::models::config::Config;
    use crate::utils::config_utils::read_config;

    use serde_json;
    use std::env;
    let config_str = read_config(app)?;
    let config: Config = serde_json::from_str(&config_str)
        .map_err(|e| format!("Failed to deserialize config: {}", e))?;

    if config.linux_desktop_install_choice == "installed" || !env::var("APPIMAGE").is_ok() {
        let _ = crate::utils::os_integration_linux::install_desktop_file();
    }

    Ok(())
}

/// Handles Windows-specific setup tasks.
///
/// Currently a placeholder for possible Windows-specific setup operations.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<(), String>` - Ok if setup succeeds, Err with error message otherwise.
#[cfg(target_os = "windows")]
fn perform_windows_setup(_app: &AppHandle) -> Result<(), String> {
    // No Windows-specific setup needed at the moment
    Ok(())
}

/// Handles macOS-specific setup tasks.
///
/// Currently a placeholder for possible macOS-specific setup operations.
///
/// # Arguments
/// * `app` - The Tauri application handle.
///
/// # Returns
/// `Result<(), String>` - Ok if setup succeeds, Err with error message otherwise.
#[cfg(target_os = "macos")]
fn perform_macos_setup(_app: &AppHandle) -> Result<(), String> {
    // No macOS-specific setup needed at the moment
    Ok(())
}
