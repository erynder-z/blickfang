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

/// Reads an image file from the specified path and extracts its metadata.
///
/// # Arguments
/// * `path` - The path to the image file.
///
/// # Returns
/// `Result<ImageMetadata, String>` - The extracted image metadata.
pub async fn read_image_file(path: &str) -> Result<ImageMetadata, String> {
    let path_buf = PathBuf::from(&path);
    let bytes = read_file_bytes(&path_buf).await?;
    let metadata = tokio::task::spawn_blocking(move || process_image_metadata(&path_buf, &bytes))
        .await
        .map_err(|e| format!("Failed to spawn blocking task: {}", e))??;
    Ok(metadata)
}

/// Reads the contents of a file at the specified path.
///
/// # Arguments
/// * `path` - The path to the file to read.
///
/// # Returns
/// `Result<Vec<u8>, String>` - A result containing the file contents as a vector of bytes if successful,
/// or an error string if the file cannot be read.
async fn read_file_bytes(path: &Path) -> Result<Vec<u8>, String> {
    tokio::fs::read(path)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))
}

/// Returns the color depth of the given image color type in bits.
///
/// The color depth is the number of bits used to represent each pixel in the image.
///
/// # Arguments
/// * `color_type` - The image color type to get the color depth of.
///
/// # Returns
/// `Option<u8>` - The color depth of the given color type if it is supported,
/// or `None` if the color type is not supported.
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

/// Returns the size of the file at the given path in bytes.
///
/// # Arguments
/// * `path` - The path to the file to get the size of.
///
/// # Returns
/// `Result<u64, String>` - The size of the file in bytes if successful, or an error string if the file cannot be accessed.
fn get_file_size(path: &Path) -> Result<u64, String> {
    fs::metadata(path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))
        .map(|m| m.len())
}

/// Returns the dimensions and color depth of the image given in the bytes.
///
/// # Arguments
/// * `bytes` - The image bytes to get the dimensions and color depth of.
///
/// # Returns
/// `Result<((u32, u32), Option<u8>), String>` - The dimensions and color depth of the image if successful,
/// or an error string if the image cannot be decoded.
///
/// The first element of the returned tuple is the image dimensions as a `(u32, u32)` tuple,
/// and the second element is the color depth of the image in bits as an `Option<u8>`.
///
/// If the image color type is not supported, the color depth will be `None`.
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

/// Processes the given image bytes and returns an `ImageMetadata` object containing the image data as a base64-encoded string, its EXIF data as a JSON string, its width, height, aspect ratio, format, color depth, and file size.
///
/// # Arguments
/// * `path` - The path to the image file.
/// * `bytes` - The image bytes to process.
///
/// # Returns
/// `Result<ImageMetadata, String>` - The processed image metadata if successful, or an error string if the image cannot be processed.
fn process_image_metadata(path: &Path, bytes: &[u8]) -> Result<ImageMetadata, String> {
    let (mime_type, format) = guess_image_format(path, bytes);
    let ((width, height), color_depth) = get_image_details(bytes)?;

    let aspect_ratio = compute_aspect_ratio(width, height);
    let data_url = build_data_url(&mime_type, bytes);
    let exif_data = extract_exif_json(bytes);
    let file_size = get_file_size(path)?;

    Ok(ImageMetadata {
        image_data: data_url,
        exif_data,
        width,
        height,
        aspect_ratio,
        format,
        color_depth,
        file_size,
    })
}

/// Guesses the MIME type and format of an image given its bytes and path.
///
/// If the image format can be determined using the `image` crate, it returns the MIME type and format as a string.
/// If the image format cannot be determined, it falls back to using the `mime_guess` crate to determine the MIME type and uses the file extension to determine the format.
///
/// # Arguments
///
/// * `path` - The path to the image file.
/// * `bytes` - The image bytes to guess the format of.
///
/// # Returns
///
/// A tuple containing the MIME type and format of the image as strings.
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

/// Computes the aspect ratio of an image given its width and height.
///
/// The aspect ratio is returned as a string in the format "width:height".
///
/// If either the width or height is zero, an empty string is returned.
///
/// # Arguments
///
/// * `width` - The width of the image.
/// * `height` - The height of the image.
///
/// # Returns
///
/// A string containing the aspect ratio of the image.
fn compute_aspect_ratio(width: u32, height: u32) -> String {
    if width == 0 || height == 0 {
        return String::new();
    }
    let divisor = gcd(width, height);
    format!("{}:{}", width / divisor, height / divisor)
}

/// Builds a data URL string from the given mime type and bytes.
///
/// # Arguments
///
/// * `mime_type` - The mime type of the data.
/// * `bytes` - The bytes of the data.
///
/// # Returns
///
/// A string containing the data URL.
fn build_data_url(mime_type: &str, bytes: &[u8]) -> String {
    let base64_str = general_purpose::STANDARD.encode(bytes);
    format!("data:{};base64,{}", mime_type, base64_str)
}

/// Extracts the EXIF data from the given bytes and returns it as a JSON string.
///
/// The extracted EXIF data is a map of tag names to their corresponding values.
/// The values are strings, and are either the original byte value if it is a valid UTF-8 string,
/// or the value of `display_value` with the unit if it is not a valid UTF-8 string.
///
/// If the bytes do not contain valid EXIF data, an empty string is returned.
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

/// Extracts the original orientation of an image from its EXIF data.
///
/// Returns `None` if the bytes do not contain valid EXIF data or if the Orientation tag is not present.
/// Returns `Some(orientation)` if the Orientation tag is present, where `orientation` is the value of the tag as a `u16`.
///
/// # Arguments
///
/// * `bytes` - The raw image data as a byte slice.
fn extract_original_orientation(bytes: &[u8]) -> Option<u16> {
    match Reader::new().read_from_container(&mut std::io::Cursor::new(bytes)) {
        Ok(exif) => {
            if let Some(orientation_field) =
                exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
            {
                orientation_field.value.get_uint(0).map(|val| val as u16)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Applies the orientation correction to the given image according to the given orientation.
///
/// The orientation is expected to be a value from the EXIF Orientation tag.
///
/// The function returns the corrected image, or the original image if the orientation is not valid.
///
/// # Arguments
///
/// * `img` - The image to correct.
/// * `orientation` - The orientation value from the EXIF Orientation tag.
fn apply_orientation_correction(img: DynamicImage, orientation: u16) -> DynamicImage {
    match orientation {
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate270().fliph(),
        8 => img.rotate270(),
        _ => img,
    }
}

/// Saves image bytes to a specified path and format, with optional quality settings.
///
/// # Arguments
/// * `bytes` - The raw image data as a byte slice.
/// * `save_path` - The destination path to save the image.
/// * `format` - The desired output format (e.g., "png", "jpeg", "webp").
/// * `quality` - Optional quality setting for formats like JPEG/WebP (0.0-100.0).
/// * `rotation` - Optional rotation angle in degrees (0, 90, 180, 270).
///
/// # Returns
/// `Result<String, String>` - The path to the saved file if successful.
pub fn save_image_to_format(
    bytes: &[u8],
    save_path: &Path,
    format: &str,
    quality: Option<f32>,
    rotation: i32,
) -> Result<String, String> {
    let image_format = ImageFormat::from_extension(format)
        .ok_or_else(|| format!("Invalid image format: {}", format))?;
    
    // First, extract the original orientation from the EXIF data
    let original_orientation = extract_original_orientation(bytes);
    
    let mut img =
        image::load_from_memory(bytes).map_err(|e| format!("Failed to decode image: {}", e))?;

    // For formats that don't preserve EXIF (like PNG), we need to bake the orientation
    // into the pixel data. For JPEG, the EXIF orientation will be preserved.
    let should_bake_orientation = !matches!(image_format, ImageFormat::Jpeg);
    
    if should_bake_orientation {
        // Apply the original EXIF orientation to the pixel data
        if let Some(orientation) = original_orientation {
            img = apply_orientation_correction(img, orientation);
        }
    }

    // Apply any additional rotation from user interaction
    img = match rotation {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img,
    };

    match image_format {
        ImageFormat::WebP => save_webp(&img, save_path, quality)?,
        ImageFormat::Jpeg => save_jpeg(&img, save_path, quality)?,
        _ => img
            .save_with_format(save_path, image_format)
            .map_err(|e| format!("Failed to save image: {}", e))?,
    }

    Ok(save_path.to_string_lossy().to_string())
}

/// Saves a `DynamicImage` to a WebP file.
///
/// # Arguments
///
/// * `img` - The image to save.
/// * `save_path` - The destination path to save the image.
/// * `quality` - Optional quality setting for the saved WebP image (0.0-100.0). Defaults to 75.0 if not provided.
///
/// # Returns
/// `Result<(), String>` - The result of the save operation. Returns an error string if the save operation fails.
fn save_webp(img: &DynamicImage, save_path: &Path, quality: Option<f32>) -> Result<(), String> {
    let rgba_image = img.to_rgba8();
    let encoder = webp::Encoder::from_rgba(&rgba_image, rgba_image.width(), rgba_image.height());
    let memory_encoder = encoder.encode(quality.unwrap_or(75.0) as f32);
    std::fs::write(save_path, &*memory_encoder)
        .map_err(|e| format!("Failed to save WebP image: {}", e))
}

/// Saves a `DynamicImage` to a JPEG file.
///
/// # Arguments
///
/// * `img` - The image to save.
/// * `save_path` - The destination path to save the image.
/// * `quality` - Optional quality setting for the saved JPEG image (1.0-100.0). Defaults to 75.0 if not provided.
///
/// # Returns
/// `Result<(), String>` - The result of the save operation. Returns an error string if the save operation fails.
fn save_jpeg(img: &DynamicImage, save_path: &Path, quality: Option<f32>) -> Result<(), String> {
    let mut file = std::fs::File::create(save_path)
        .map_err(|e| format!("Failed to create JPEG file: {}", e))?;
    let quality_u8 = quality.map(|q| q.clamp(1.0, 100.0) as u8).unwrap_or(75);
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, quality_u8);
    img.write_with_encoder(encoder)
        .map_err(|e| format!("Failed to save JPEG image: {}", e))
}

/// Returns a list of image formats supported for saving.
///
/// # Returns
/// `Result<Vec<String>, String>` - A vector of supported image format strings (e.g., "png", "jpeg", "webp", "bmp").
pub fn get_supported_image_formats() -> Result<Vec<String>, String> {
    let formats = vec![
        "png".to_string(),
        "jpeg".to_string(),
        "webp".to_string(),
        "bmp".to_string(),
    ];
    Ok(formats)
}

/// Detects C2PA (Content Authenticity Initiative) metadata in an image file.
///
/// # Arguments
/// * `path` - The path to the image file.
///
/// # Returns
/// `Result<(bool, Option<String>), String>` - A tuple where the boolean indicates
/// if C2PA data was found, and the `Option<String>` contains the C2PA JSON payload if available.
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

/// Extracts the C2PA store from a given byte slice.
///
/// The function expects the byte slice to contain the C2PA UUID (16 bytes) followed by the store data.
/// It returns the JSON payload of the store as a string, along with the store data as a vector of bytes.
///
/// If the byte slice does not contain the C2PA UUID or the store data is malformed, the function returns `None`.
///
/// # Arguments
/// * `bytes` - The byte slice containing the C2PA UUID and store data.
///
/// # Returns
/// `Option<(String, Vec<u8>)>` - A tuple containing the JSON payload of the store as a string and the store data as a vector of bytes. If the byte slice does not contain the C2PA UUID or the store data is malformed, the function returns `None`.
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

/// Extracts text chunks (tEXt, iTXt) from PNG image bytes.
///
/// # Arguments
/// * `bytes` - The raw PNG image data.
///
/// # Returns
/// `Vec<(String, String)>` - A list of (key, value) pairs for extracted text chunks.
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

/// Extracts XMP metadata from WebP image bytes.
///
/// # Arguments
/// * `bytes` - The raw WebP image data.
///
/// # Returns
/// `Option<String>` - An `Option` containing the XMP XML string if found.
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
