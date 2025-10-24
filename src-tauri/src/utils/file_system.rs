use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn get_directory_files(path: &str) -> Result<Vec<PathBuf>, String> {
    let path = Path::new(path);

    let parent_dir = path
        .parent()
        .ok_or_else(|| "Could not determine parent directory".to_string())?;

    let image_extensions = ["png", "jpg", "jpeg", "webp"];
    let mut image_files = Vec::new();

    let mut read_dir = fs::read_dir(parent_dir)
        .await
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    while let Some(entry) = read_dir
        .next_entry()
        .await
        .map_err(|e| format!("Failed to read directory entry: {}", e))?
    {
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if image_extensions.contains(&ext.to_lowercase().as_str()) {
                image_files.push(path);
            }
        }
    }

    image_files.sort();

    Ok(image_files)
}
