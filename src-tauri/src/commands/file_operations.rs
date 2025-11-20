use tauri::Window;

use crate::models::image::ImageMetadata;
use crate::utils::{
    dialog_utils::{open_image_dialog, show_save_dialog},
    file_system::get_filtered_directory_files,
    image_processing::{
        self, get_supported_image_formats as get_formats, load_image_bytes, read_image_file,
    },
};

#[tauri::command]
pub async fn open_and_read_file(
    window: Window,
) -> Result<Option<(ImageMetadata, String, Vec<String>)>, String> {
    if let Some(path_buf) = open_image_dialog(window).await? {
        let path_str = path_buf.to_string_lossy().to_string();
        let metadata = read_image_file(&path_str)
            .await
            .map_err(|e| format!("Failed to read image file '{}': {}", path_str, e))?;
        let directory_files = get_filtered_directory_files(&path_str).await?;
        Ok(Some((metadata, path_str, directory_files)))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn read_image_from_path(
    path: String,
) -> Result<(ImageMetadata, String, Vec<String>), String> {
    let metadata = read_image_file(&path)
        .await
        .map_err(|e| format!("Failed to read image file '{}': {}", path, e))?;
    let directory_files = get_filtered_directory_files(&path).await?;
    Ok((metadata, path, directory_files))
}

#[tauri::command]
pub async fn save_image_as(
    window: Window,
    path: String,
    format: String,
    quality: Option<f32>,
) -> Result<Option<String>, String> {
    if let Some(save_path) = show_save_dialog(window, &path, &format).await? {
        let bytes = load_image_bytes(&path).await?;
        let result = tokio::task::spawn_blocking(move || {
            image_processing::save_image_to_format(&bytes, &save_path, &format, quality)
        })
        .await
        .map_err(|e| format!("Task spawn error: {}", e))??;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn get_supported_image_formats() -> Result<Vec<String>, String> {
    get_formats()
}

#[tauri::command]
pub async fn change_image(
    current_path: String,
    direction: String,
) -> Result<(ImageMetadata, String), String> {
    let files = get_filtered_directory_files(&current_path).await?;

    if files.len() <= 1 {
        return Err("No other images in directory".to_string());
    }

    let current_index = files
        .iter()
        .position(|f| f == &current_path)
        .ok_or_else(|| "Current image not found in directory".to_string())?;

    let next_index = match direction.as_str() {
        "next" => (current_index + 1) % files.len(),
        "previous" | "prev" => (current_index + files.len() - 1) % files.len(),
        _ => return Err("Invalid direction. Must be 'next' or 'previous'.".to_string()),
    };

    let next_image_path = files[next_index].clone();
    let metadata = read_image_file(&next_image_path).await?;

    Ok((metadata, next_image_path))
}
