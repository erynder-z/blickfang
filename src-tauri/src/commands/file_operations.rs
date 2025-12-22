use tauri::Window;

use crate::models::image::ImageMetadata;
use crate::utils::{
    dialog_utils::{open_image_dialog, show_save_dialog},
    file_system::get_filtered_directory_files,
    image_processing::{self, get_supported_image_formats as get_formats, read_image_file},
};
use base64::Engine;

/// Opens a file dialog, reads the selected image, and returns its metadata,
/// path, and a list of other image files in the same directory.
///
/// # Arguments
/// * `window` - The Tauri window handle.
///
/// # Returns
/// `Result<Option<(ImageMetadata, String, Vec<String>)>, String>` - A result containing
/// an `Option` with the image metadata, its path, and a list of other files in the
/// directory if a file was selected, or `None` if the dialog was cancelled.
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

/// Reads image metadata and lists other files in the same directory given a specific path.
///
/// # Arguments
/// * `path` - The path to the image file as a `String`.
///
/// # Returns
/// `Result<(ImageMetadata, String, Vec<String>)>, String>` - A result containing
/// the image metadata, its path, and a list of other files in the directory.
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

/// Saves an image from base64 encoded data to a specified path and format, with optional quality.
/// This unified command works for both normal and ASCII-converted images.
///
/// # Arguments
/// * `window` - The Tauri window handle.
/// * `base64data` - The base64 encoded image data.
/// * `source_name` - The source name for filename suggestion (file path or "ascii_art").
/// * `format` - The desired output format (e.g., "png", "jpeg").
/// * `quality` - Optional quality setting for formats like JPEG (0.0-1.0).
/// * `rotation` - The rotation angle in degrees (0, 90, 180, 270).
///
/// # Returns
/// `Result<Option<String>, String>` - A result containing an `Option` with the
/// path to the saved file if successful, or `None` if the save operation was cancelled.
#[tauri::command]
pub async fn save_base64_image_as(
    window: Window,
    base64data: String,
    source_name: String,
    format: String,
    quality: Option<f32>,
    rotation: i32,
) -> Result<Option<String>, String> {
    if let Some(save_path) = show_save_dialog(window, &source_name, &format).await? {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(base64data)
            .map_err(|e| format!("Failed to decode base64: {}", e))?;

        let result = tokio::task::spawn_blocking(move || {
            image_processing::save_image_to_format(&bytes, &save_path, &format, quality, rotation)
        })
        .await
        .map_err(|e| format!("Task spawn error: {}", e))??;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

/// Returns a list of image formats supported for saving.
///
/// # Returns
/// `Result<Vec<String>, String>` - A result containing a vector of supported
/// image format strings.
#[tauri::command]
pub fn get_supported_image_formats() -> Result<Vec<String>, String> {
    get_formats()
}

/// Navigates to the next or previous image in the current directory.
///
/// # Arguments
/// * `current_path` - The path of the currently displayed image.
/// * `direction` - The navigation direction, either "next" or "previous".
///
/// # Returns
/// `Result<(ImageMetadata, String), String>` - A result containing the metadata
/// and path of the new image.
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
