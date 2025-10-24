use base64::{engine::general_purpose, Engine as _};
use exif::Reader;
use mime_guess;
use std::path::PathBuf;
use tauri::Emitter;
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
        .pick_file(move |file_path| {
            let _ = tx.send(file_path.map(|f| match f {
                tauri_plugin_dialog::FilePath::Path(p) => p,
                _ => PathBuf::new(),
            }));
        });

    let file_path_option = rx.await.map_err(|e| e.to_string())?;

    if let Some(path_buf) = file_path_option {
        let path_str = path_buf.to_string_lossy().to_string();
        let (image_data, exif_data) = read_image_file(path_str.clone())?;
        let directory_files: Vec<String> = get_directory_files(&path_str)
            .await?
            .into_iter()
            .filter_map(|p| p.to_str().map(|s| s.to_string()))
            .collect();

        window.emit("new_image_path", path_str.clone()).ok();

        Ok((image_data, exif_data, path_str, directory_files))
    } else {
        Ok((String::new(), String::new(), String::new(), Vec::new()))
    }
}

#[tauri::command]
pub fn read_image_file(path: String) -> Result<(String, String), String> {
    let path_buf = PathBuf::from(&path);

    let bytes = std::fs::read(&path_buf)
        .map_err(|e| format!("Failed to read file '{}': {}", path_buf.display(), e))?;

    let mime_type = mime_guess::from_path(&path_buf).first_or_octet_stream();

    let base64_str = general_purpose::STANDARD.encode(&bytes);
    let data_url = format!("data:{};base64,{}", mime_type, base64_str);

    let exif_data = extract_exif_json(&bytes);

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

    let files_str: Vec<String> = files
        .iter()
        .filter_map(|p| p.to_str().map(|s| s.to_string()))
        .collect();

    let current_index = files_str
        .iter()
        .position(|f| f == &current_path)
        .ok_or_else(|| "Current image not found in directory".to_string())?;

    let next_index = match direction.as_str() {
        "next" => (current_index + 1) % files_str.len(),
        "previous" | "prev" => (current_index + files_str.len() - 1) % files_str.len(),
        _ => return Err("Invalid direction. Must be 'next' or 'previous'.".to_string()),
    };

    let next_image_path = files_str[next_index].clone();
    let (image_data, exif_data) = read_image_file(next_image_path.clone())?;

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
