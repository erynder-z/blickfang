use std::path::PathBuf;
use tauri::Window;
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;

pub async fn open_image_dialog(window: Window) -> Result<Option<PathBuf>, String> {
    let (tx, rx) = oneshot::channel();

    window
        .dialog()
        .file()
        .add_filter("Image files", &["png", "jpg", "jpeg", "webp", "bmp"])
        .pick_file(move |result| {
            let path_to_send = result.and_then(|fp| match fp {
                tauri_plugin_dialog::FilePath::Path(p) => Some(p),
                _ => None,
            });
            let _ = tx.send(path_to_send);
        });

    rx.await
        .map_err(|e| format!("Failed to receive file path from dialog: {}", e))
}

pub async fn show_save_dialog(
    window: Window,
    source_path: &str,
    format: &str,
) -> Result<Option<PathBuf>, String> {
    let (tx, rx) = oneshot::channel();

    let file_stem = PathBuf::from(source_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image")
        .to_string();

    let new_extension = format.to_lowercase();
    let suggested_filename = format!("{}.{}", file_stem, new_extension);

    window
        .dialog()
        .file()
        .set_file_name(&suggested_filename)
        .add_filter(
            &format!("{} files", format.to_uppercase()),
            &[&new_extension],
        )
        .save_file(move |file_path_result| {
            let path_to_send = file_path_result.and_then(|fp| match fp {
                tauri_plugin_dialog::FilePath::Path(p) => Some(p),
                _ => None,
            });
            let _ = tx.send(path_to_send);
        });

    rx.await
        .map_err(|e| format!("Failed to receive save path from dialog: {}", e))
}