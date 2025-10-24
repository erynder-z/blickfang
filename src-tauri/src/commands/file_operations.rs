use base64::{engine::general_purpose, Engine as _};
use exif::Reader;
use mime_guess;
use std::path::PathBuf;
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;

use crate::utils::file_system::get_directory_files;

#[tauri::command]
pub async fn open_and_read_file(
    window: tauri::Window,
) -> Result<(String, String, String, Vec<String>), String> {
    let (tx, rx) = oneshot::channel();

    window
        .dialog()
        .file()
        .add_filter("Image files", &["png", "jpg", "jpeg", "webp"])
        .pick_file(move |file_path_result| {
            let path_to_send = file_path_result.and_then(|fp| match fp {
                tauri_plugin_dialog::FilePath::Path(p) => Some(p),
                _ => None,
            });
            let _ = tx.send(path_to_send);
        });

    let file_path_option = rx
        .await
        .map_err(|e| format!("Failed to receive file path from dialog: {}", e))?;

    if let Some(path_buf) = file_path_option {
        let path_str = path_buf.to_string_lossy().to_string();
        let (image_data, exif_data) = read_image_file(path_str.clone())
            .await
            .map_err(|e| format!("Failed to read image file '{}': {}", path_str, e))?;
        let directory_files = get_directory_files(&path_str).await?;

        Ok((image_data, exif_data, path_str, directory_files))
    } else {
        Ok((String::new(), String::new(), String::new(), Vec::new()))
    }
}

#[tauri::command]
pub async fn read_image_from_path(
    path: String,
) -> Result<(String, String, String, Vec<String>), String> {
    let (image_data, exif_data) = read_image_file(path.clone())
        .await
        .map_err(|e| format!("Failed to read image file '{}': {}", path, e))?;
    let directory_files = get_directory_files(&path).await?;

    Ok((image_data, exif_data, path, directory_files))
}

pub async fn read_image_file(path: String) -> Result<(String, String), String> {
    let path_buf = PathBuf::from(&path);

    let bytes = tokio::fs::read(&path_buf)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path_buf.display(), e))?;

    let (data_url, exif_data) = tokio::task::spawn_blocking(move || {
        let mime_type = mime_guess::from_path(&path_buf).first_or_octet_stream();
        let base64_str = general_purpose::STANDARD.encode(&bytes);
        let data_url = format!("data:{};base64,{}", mime_type, base64_str);
        let exif_data = extract_exif_json(&bytes);
        (data_url, exif_data)
    })
    .await
    .map_err(|e| format!("Failed to spawn blocking task for image processing: {}", e))?;

    Ok((data_url, exif_data))
}

#[tauri::command]
pub async fn change_image(
    current_path: String,
    direction: String,
) -> Result<(String, String, String), String> {
    let files = get_directory_files(&current_path).await?;

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
    let (image_data, exif_data) = read_image_file(next_image_path.clone()).await?;

    Ok((image_data, next_image_path, exif_data))
}

fn extract_exif_json(bytes: &[u8]) -> String {
    match Reader::new().read_from_container(&mut std::io::Cursor::new(bytes)) {
        Ok(exif) => {
            let exif_map: std::collections::HashMap<_, _> = exif
                .fields()
                .map(|field| {
                    (
                        field.tag.to_string(),
                        field.display_value().with_unit(&exif).to_string(),
                    )
                })
                .collect();
            serde_json::to_string(&exif_map).unwrap_or_default()
        }
        Err(_) => String::new(),
    }
}
