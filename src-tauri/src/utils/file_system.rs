use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn get_directory_files(file_path: &str) -> Result<Vec<String>, String> {
    let path = Path::new(file_path);
    let parent_dir = path
        .parent()
        .ok_or_else(|| "Could not determine parent directory".to_string())?;

    let image_extensions = [
        "png", "jpg", "jpeg", "webp", "bmp", "dds", "gif", "hdr", "ico", "tga", "tif", "tiff",
    ];
    let mut image_files = Vec::new();

    let mut dir = fs::read_dir(parent_dir)
        .await
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    while let Some(entry) = dir
        .next_entry()
        .await
        .map_err(|e| format!("Failed to read directory entry: {}", e))?
    {
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if image_extensions.contains(&ext.to_lowercase().as_str()) {
                if let Some(path_str) = path.to_str() {
                    image_files.push(path_str.to_string());
                }
            }
        }
    }

    image_files.sort();
    Ok(image_files)
}

pub fn filter_dot_files(paths: Vec<String>) -> Vec<String> {
    paths
        .into_iter()
        .filter(|p| {
            PathBuf::from(p)
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| !name.starts_with('.'))
                .unwrap_or(true)
        })
        .collect()
}

pub async fn get_filtered_directory_files(path: &str) -> Result<Vec<String>, String> {
    let files = get_directory_files(path).await?;
    Ok(filter_dot_files(files))
}
