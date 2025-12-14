use std::env;
use std::fs;
use std::path::PathBuf;

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
