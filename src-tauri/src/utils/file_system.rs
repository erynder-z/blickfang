use std::path::{Path, PathBuf};
use tokio::fs;

/// Reads the parent directory of the given file path and returns a sorted list of
/// paths to image files found within that directory.
///
/// # Arguments
/// * `file_path` - A path to a file within the target directory.
///
/// # Returns
/// `Result<Vec<String>, String>` - A sorted vector of absolute paths to image files.
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

/// Filters a list of file paths, removing any files whose names start with a dot ('.').
///
/// # Arguments
/// * `paths` - A vector of file paths (as Strings).
///
/// # Returns
/// `Vec<String>` - The filtered list of file paths.
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

/// Retrieves a sorted list of non-hidden image files from the directory
/// containing the given path.
///
/// This function combines `get_directory_files` and `filter_dot_files`.
///
/// # Arguments
/// * `path` - A path within the target directory.
///
/// # Returns
/// `Result<Vec<String>, String>` - A list of filtered image file paths.
pub async fn get_filtered_directory_files(path: &str) -> Result<Vec<String>, String> {
    let files = get_directory_files(path).await?;
    Ok(filter_dot_files(files))
}
