use base64::{engine::general_purpose, Engine};
use exif::Reader;
use image::{self, DynamicImage, ImageFormat};
use mime_guess;
use serde::Serialize;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use webp;

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

pub async fn read_image_file(path: &str) -> Result<ImageMetadata, String> {
    let path_buf = PathBuf::from(&path);
    let bytes = read_file_bytes(&path_buf).await?;
    let metadata = tokio::task::spawn_blocking(move || process_image_metadata(&path_buf, &bytes))
        .await
        .map_err(|e| format!("Failed to spawn blocking task: {}", e))??;
    Ok(metadata)
}

async fn read_file_bytes(path: &Path) -> Result<Vec<u8>, String> {
    tokio::fs::read(path)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))
}

fn process_image_metadata(path: &Path, bytes: &[u8]) -> Result<ImageMetadata, String> {
    let (mime_type, format) = guess_image_format(path, bytes);
    let (width, height) = get_image_dimensions(bytes);
    let aspect_ratio = compute_aspect_ratio(width, height);
    let data_url = build_data_url(&mime_type, bytes);
    let exif_data = extract_exif_json(bytes);

    Ok(ImageMetadata {
        image_data: data_url,
        exif_data,
        width,
        height,
        aspect_ratio,
        format,
    })
}

fn guess_image_format(path: &Path, bytes: &[u8]) -> (String, String) {
    let image_format_guess = image::guess_format(bytes);
    let mime_type = match image_format_guess {
        Ok(format) => format.to_mime_type().to_string(),
        Err(_) => mime_guess::from_path(path)
            .first_or_octet_stream()
            .essence_str()
            .to_string(),
    };
    let format = match image_format_guess {
        Ok(f) => format!("{:?}", f).to_uppercase(),
        Err(_) => path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_uppercase(),
    };
    (mime_type, format)
}

fn get_image_dimensions(bytes: &[u8]) -> (u32, u32) {
    image::ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .ok()
        .and_then(|r| r.into_dimensions().ok())
        .unwrap_or((0, 0))
}

fn compute_aspect_ratio(width: u32, height: u32) -> String {
    if width == 0 || height == 0 {
        return String::new();
    }
    let divisor = gcd(width, height);
    format!("{}:{}", width / divisor, height / divisor)
}

fn build_data_url(mime_type: &str, bytes: &[u8]) -> String {
    let base64_str = general_purpose::STANDARD.encode(bytes);
    format!("data:{};base64,{}", mime_type, base64_str)
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

pub async fn load_image_bytes(path: &str) -> Result<Vec<u8>, String> {
    tokio::fs::read(path)
        .await
        .map_err(|e| format!("Failed to read original file '{}': {}", path, e))
}

pub fn save_image_to_format(
    bytes: &[u8],
    save_path: &Path,
    format: &str,
    quality: Option<f32>,
) -> Result<String, String> {
    let image_format = ImageFormat::from_extension(format)
        .ok_or_else(|| format!("Invalid image format: {}", format))?;
    let img = image::load_from_memory(bytes).map_err(|e| format!("Failed to decode image: {}", e))?;

    match image_format {
        ImageFormat::WebP => save_webp(&img, save_path, quality)?,
        ImageFormat::Jpeg => save_jpeg(&img, save_path, quality)?,
        _ => img
            .save_with_format(save_path, image_format)
            .map_err(|e| format!("Failed to save image: {}", e))?,
    }

    Ok(save_path.to_string_lossy().to_string())
}

fn save_webp(img: &DynamicImage, save_path: &Path, quality: Option<f32>) -> Result<(), String> {
    let rgba_image = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height());
    let memory_encoder = encoder.encode(quality.unwrap_or(75.0) as f32);
    std::fs::write(save_path, &*memory_encoder)
        .map_err(|e| format!("Failed to save WebP image: {}", e))
}

fn save_jpeg(img: &DynamicImage, save_path: &Path, quality: Option<f32>) -> Result<(), String> {
    let mut file = std::fs::File::create(save_path)
        .map_err(|e| format!("Failed to create JPEG file: {}", e))?;
    let quality_u8 = quality.map(|q| q.clamp(1.0, 100.0) as u8).unwrap_or(75);
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, quality_u8);
    img.write_with_encoder(encoder)
        .map_err(|e| format!("Failed to save JPEG image: {}", e))
}

pub fn get_supported_image_formats() -> Result<Vec<String>, String> {
    let formats = vec![
        "png".to_string(),
        "jpeg".to_string(),
        "webp".to_string(),
        "bmp".to_string(),
    ];
    Ok(formats)
}