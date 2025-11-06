use base64::engine::general_purpose;
use base64::Engine;
use exif::Reader;
use image::ImageFormat;
use mime_guess;
use serde::Serialize;
use std::io::Cursor;
use std::path::PathBuf;
use tauri::Window;
use tauri_plugin_dialog::DialogExt;
use tokio::sync::oneshot;
use webp;

use crate::utils::file_system::get_directory_files;

#[derive(Serialize, Clone)]
pub struct ImageMetadata {
    pub image_data: String,
    pub exif_data: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: String,
    pub format: String,
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn filter_dot_files(paths: Vec<String>) -> Vec<String> {
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

#[tauri::command]
pub async fn open_and_read_file(
    window: tauri::Window,
) -> Result<Option<(ImageMetadata, String, Vec<String>)>, String> {
    let (tx, rx) = oneshot::channel();

    window
        .dialog()
        .file()
        .add_filter("Image files", &["png", "jpg", "jpeg", "webp", "bmp"])
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
        let metadata = read_image_file(path_str.clone())
            .await
            .map_err(|e| format!("Failed to read image file '{}': {}", path_str, e))?;
        let mut directory_files = get_directory_files(&path_str).await?;
        directory_files = filter_dot_files(directory_files);

        Ok(Some((metadata, path_str, directory_files)))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn read_image_from_path(
    path: String,
) -> Result<(ImageMetadata, String, Vec<String>), String> {
    let metadata = read_image_file(path.clone())
        .await
        .map_err(|e| format!("Failed to read image file '{}': {}", path, e))?;
    let mut directory_files = get_directory_files(&path).await?;
    directory_files = filter_dot_files(directory_files);

    Ok((metadata, path, directory_files))
}

pub async fn read_image_file(path: String) -> Result<ImageMetadata, String> {
    let path_buf = PathBuf::from(&path);

    let bytes = tokio::fs::read(&path_buf)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path_buf.display(), e))?;

    let metadata = tokio::task::spawn_blocking(move || {
        let image_format_guess = image::guess_format(&bytes);

        let mime_type = match image_format_guess {
            Ok(format) => format.to_mime_type().to_string(),
            Err(_) => mime_guess::from_path(&path_buf)
                .first_or_octet_stream()
                .essence_str()
                .to_string(),
        };

        let format = match image_format_guess {
            Ok(f) => format!("{:?}", f).to_uppercase(),
            Err(_) => path_buf
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_uppercase(),
        };

        let (width, height) = (|| -> Result<(u32, u32), image::ImageError> {
            let reader = image::ImageReader::new(Cursor::new(&bytes)).with_guessed_format()?;
            reader.into_dimensions()
        })()
        .unwrap_or((0, 0));

        let aspect_ratio = if width > 0 && height > 0 {
            let divisor = gcd(width, height);
            format!("{}:{}", width / divisor, height / divisor)
        } else {
            String::new()
        };

        let base64_str = general_purpose::STANDARD.encode(&bytes);
        let data_url = format!("data:{};base64,{}", mime_type, base64_str);
        let exif_data = extract_exif_json(&bytes);

        ImageMetadata {
            image_data: data_url,
            exif_data,
            width,
            height,
            aspect_ratio,
            format,
        }
    })
    .await
    .map_err(|e| format!("Failed to spawn blocking task for image processing: {}", e))?;

    Ok(metadata)
}

#[tauri::command]
pub async fn save_image_as(
    window: Window,
    path: String,
    format: String,
    quality: Option<f32>,
) -> Result<Option<String>, String> {
    let save_path_option = show_save_dialog(window, &path, &format).await?;
    if save_path_option.is_none() {
        return Ok(None);
    }

    let save_path = save_path_option.unwrap();

    let bytes = load_image_bytes(&path).await?;

    let result = tokio::task::spawn_blocking(move || {
        save_image_to_format(&bytes, &save_path, &format, quality)
    })
    .await
    .map_err(|e| format!("Task spawn error: {}", e))??;

    Ok(Some(result))
}

async fn show_save_dialog(
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

async fn load_image_bytes(path: &str) -> Result<Vec<u8>, String> {
    tokio::fs::read(path)
        .await
        .map_err(|e| format!("Failed to read original file '{}': {}", path, e))
}

fn save_image_to_format(
    bytes: &[u8],
    save_path: &PathBuf,
    format: &str,
    quality: Option<f32>,
) -> Result<String, String> {
    let image_format = ImageFormat::from_extension(format)
        .ok_or_else(|| format!("Invalid image format: {}", format))?;

    let img =
        image::load_from_memory(bytes).map_err(|e| format!("Failed to decode image: {}", e))?;

    match image_format {
        ImageFormat::WebP => save_webp(&img, save_path, quality)?,
        ImageFormat::Jpeg => save_jpeg(&img, save_path, quality)?,
        _ => img
            .save_with_format(save_path, image_format)
            .map_err(|e| format!("Failed to save image: {}", e))?,
    }

    Ok(save_path.to_string_lossy().to_string())
}

fn save_webp(
    img: &image::DynamicImage,
    save_path: &PathBuf,
    quality: Option<f32>,
) -> Result<(), String> {
    let rgba_image = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height());
    let memory_encoder = encoder.encode(quality.unwrap_or(75.0) as f32);
    std::fs::write(save_path, &*memory_encoder)
        .map_err(|e| format!("Failed to save WebP image: {}", e))
}

fn save_jpeg(
    img: &image::DynamicImage,
    save_path: &PathBuf,
    quality: Option<f32>,
) -> Result<(), String> {
    let mut file = std::fs::File::create(save_path)
        .map_err(|e| format!("Failed to create JPEG file: {}", e))?;
    let quality_u8 = quality.map(|q| q.clamp(1.0, 100.0) as u8).unwrap_or(75);
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, quality_u8);
    img.write_with_encoder(encoder)
        .map_err(|e| format!("Failed to save JPEG image: {}", e))
}

#[tauri::command]
pub fn get_supported_image_formats() -> Result<Vec<String>, String> {
    let formats = vec![
        "png".to_string(),
        "jpeg".to_string(),
        "webp".to_string(),
        "bmp".to_string(),
    ];
    Ok(formats)
}

#[tauri::command]
pub async fn change_image(
    current_path: String,
    direction: String,
) -> Result<(ImageMetadata, String), String> {
    let mut files = get_directory_files(&current_path).await?;
    files = filter_dot_files(files);

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
    let metadata = read_image_file(next_image_path.clone()).await?;

    Ok((metadata, next_image_path))
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
