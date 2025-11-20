use base64::{engine::general_purpose, Engine};
use exif::Reader;
use image::{self, DynamicImage, ImageDecoder, ImageFormat};
use mime_guess;

use crate::models::image::ImageMetadata;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use webp;

// C2PA UUID for ai generated content detection
const C2PA_UUID: [u8; 16] = [
    0xD8, 0xFE, 0x07, 0xFF, 0xF1, 0xD9, 0x4D, 0x9A, 0xA0, 0x5E, 0xA8, 0x0B, 0x5A, 0x9F, 0xD8, 0x5A,
];



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

fn get_color_depth(color_type: image::ColorType) -> Option<u8> {
    match color_type {
        image::ColorType::L8 => Some(8),
        image::ColorType::La8 => Some(8),
        image::ColorType::Rgb8 => Some(8),
        image::ColorType::Rgba8 => Some(8),
        image::ColorType::L16 => Some(16),
        image::ColorType::La16 => Some(16),
        image::ColorType::Rgb16 => Some(16),
        image::ColorType::Rgba16 => Some(16),
        image::ColorType::Rgb32F => Some(32),
        image::ColorType::Rgba32F => Some(32),
        _ => None,
    }
}

fn get_image_details(bytes: &[u8]) -> Result<((u32, u32), Option<u8>), String> {
    let reader = image::ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess image format: {}", e))?;

    let decoder = reader
        .into_decoder()
        .map_err(|e| format!("Failed to create decoder: {}", e))?;

    let dimensions = decoder.dimensions();
    let color_type = decoder.color_type();
    let color_depth = get_color_depth(color_type);

    Ok((dimensions, color_depth))
}

fn process_image_metadata(path: &Path, bytes: &[u8]) -> Result<ImageMetadata, String> {
    let (mime_type, format) = guess_image_format(path, bytes);
    let ((width, height), color_depth) = get_image_details(bytes)?;

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
        color_depth,
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
                .filter_map(|field| {
                    let value_str = match &field.value {
                        // This is always binary, so we skip it.
                        exif::Value::Undefined(_, _) => return None,

                        exif::Value::Byte(buf) => {
                            match String::from_utf8(buf.clone()) {
                                Ok(s) => s,
                                Err(_) => return None, // Skip, if not  UTF-8
                            }
                        }

                        _ => field.display_value().with_unit(&exif).to_string(),
                    };
                    Some((field.tag.to_string(), value_str))
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

pub fn detect_c2pa(path: &Path) -> Result<(bool, Option<String>), String> {
    let bytes =
        fs::read(path).map_err(|e| format!("Failed to read file for C2PA detection: {}", e))?;
    if let Some(pos) = bytes.windows(16).position(|w| w == C2PA_UUID) {
        if let Some((json, _store)) = extract_c2pa_store(&bytes[pos..]) {
            return Ok((true, Some(json)));
        }
        return Ok((true, None));
    }
    Ok((false, None))
}

pub fn extract_c2pa_store(bytes: &[u8]) -> Option<(String, Vec<u8>)> {
    if bytes.len() < 20 {
        return None;
    }
    let payload = &bytes[16..];
    let start = payload.iter().position(|&b| b == b'{')?;
    let end = payload.iter().rposition(|&b| b == b'}')?;
    if end > start {
        if let Ok(json_str) = std::str::from_utf8(&payload[start..=end]) {
            return Some((json_str.to_string(), payload.to_vec()));
        }
    }
    None
}

pub fn extract_png_text_chunks(bytes: &[u8]) -> Vec<(String, String)> {
    let mut chunks = Vec::new();
    let mut cursor = 8;
    while cursor + 8 <= bytes.len() {
        let len = u32::from_be_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
        let name = std::str::from_utf8(&bytes[cursor + 4..cursor + 8])
            .unwrap_or("")
            .to_string();
        cursor += 8;
        if cursor + len > bytes.len() {
            break;
        }
        if name == "tEXt" || name == "iTXt" {
            if let Ok(content) = std::str::from_utf8(&bytes[cursor..cursor + len]) {
                if let Some(pos) = content.find('\0') {
                    chunks.push((content[..pos].to_string(), content[pos + 1..].to_string()));
                } else {
                    chunks.push((name.clone(), content.to_string()));
                }
            }
        }
        cursor += len + 4;
    }
    chunks
}

pub fn extract_webp_xmp(bytes: &[u8]) -> Option<String> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WEBP" {
        return None;
    }
    let mut cursor = 12;
    while cursor + 8 <= bytes.len() {
        let chunk_type = &bytes[cursor..cursor + 4];
        let chunk_size =
            u32::from_le_bytes(bytes[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
        cursor += 8;
        if cursor + chunk_size > bytes.len() {
            break;
        }
        if chunk_type == b"XMP " {
            let raw = &bytes[cursor..cursor + chunk_size];
            if let Ok(xmp_str) = std::str::from_utf8(raw) {
                return Some(xmp_str.to_string());
            }
        }
        cursor += chunk_size + (chunk_size & 1);
    }
    None
}
