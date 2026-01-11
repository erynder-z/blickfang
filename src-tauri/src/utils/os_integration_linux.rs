use std::env;
use std::fs;
use std::path::PathBuf;

/// Installs a Linux desktop file for the application.
///
/// This function will create a .desktop file in ~/.local/share/applications
/// and update the desktop database. The file will contain the path to the AppImage
/// as specified in the APPIMAGE environment variable. If the .desktop file already
/// exists, this function does nothing.
///
/// # Errors
///
/// If any of the following operations fail, an error is returned:
/// - Reading the APPIMAGE environment variable
/// - Reading the HOME environment variable
/// - Creating the ~/.local/share/applications directory
/// - Writing the .desktop file
/// - Updating the desktop database
///
/// # Panics
///
/// If any of the following operations panic, this function will panic:
/// - Creating the ~/.local/share/applications directory
/// - Writing the .desktop file
/// - Updating the desktop database
pub fn install_desktop_file() -> Result<(), Box<dyn std::error::Error>> {
    let appimage_path = env::var("APPIMAGE")?;

    let home = env::var("HOME")?;
    let applications_dir = PathBuf::from(home).join(".local/share/applications");
    fs::create_dir_all(&applications_dir)?;

    let desktop_path = applications_dir.join("blickfang.desktop");

    if desktop_path.exists() {
        return Ok(());
    }

    let template = include_str!("../../../packaging/linux/blickfang.desktop");
    let content = template.replace("%APPIMAGE_PATH%", &appimage_path);

    fs::write(&desktop_path, content)?;

    let _ = std::process::Command::new("update-desktop-database")
        .arg(&applications_dir)
        .status();

    Ok(())
}
