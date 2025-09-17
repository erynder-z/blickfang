use base64::{engine::general_purpose, Engine as _};
use mime_guess;
use std::path::PathBuf;
use tauri::Emitter;
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;

use crate::utils::file_system::get_directory_files;

#[tauri::command]
pub async fn open_and_read_file(
    window: tauri::Window,
) -> Result<(String, String, Vec<String>), String> {
    let (tx, rx) = oneshot::channel();

    window
        .dialog()
        .file()
        .add_filter("Image files", &["png", "jpg", "jpeg", "webp"])
        .pick_file(move |file_path| {
            tx.send(file_path).unwrap();
        });

    let file_path_option = rx.await.map_err(|e| e.to_string())?;

    if let Some(file_path) = file_path_option {
        let path_str = file_path.to_string();
        let path_buf = PathBuf::from(path_str);

        let new_image_data = read_image_file(path_buf.to_str().unwrap().to_string())?;
        let directory_files = get_directory_files(path_buf.to_str().unwrap().to_string()).await?;

        window
            .emit("new_image_path", path_buf.to_str().unwrap().to_string())
            .unwrap();

        Ok((
            new_image_data,
            path_buf.to_str().unwrap().to_string(),
            directory_files,
        ))
    } else {
        // User cancelled the dialog. Return an empty string.
        Ok(("".to_string(), "".to_string(), Vec::new()))
    }
}

#[tauri::command]
pub fn read_image_file(path: String) -> Result<String, String> {
    let path_buf = PathBuf::from(path);
    let mime_type = mime_guess::from_path(&path_buf).first_or_octet_stream();

    match std::fs::read(&path_buf) {
        Ok(bytes) => {
            let base64_str = general_purpose::STANDARD.encode(&bytes);
            let data_url = format!("data:{};base64,{}", mime_type, base64_str);
            Ok(data_url)
        }
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

#[tauri::command]
pub async fn change_image(
    current_path: String,
    direction: String,
) -> Result<(String, String), String> {
    let files = get_directory_files(current_path.clone()).await?;

    if files.len() <= 1 {
        return Err("No other images in directory".to_string());
    }

    let current_index = files
        .iter()
        .position(|f| f == &current_path)
        .ok_or("Current image not found in directory".to_string())?;

    let next_index = if direction == "next" {
        (current_index + 1) % files.len()
    } else {
        (current_index + files.len() - 1) % files.len()
    };

    let next_image_path = files[next_index].clone();
    let new_image_data = read_image_file(next_image_path.clone())?;

    Ok((new_image_data, next_image_path))
}
