use crate::models::image::AiDetectionResult;
use crate::utils::image_processing::{detect_c2pa, extract_png_text_chunks, extract_webp_xmp};
use exif::Reader as ExifReader;
use image::ImageReader;
use serde::Deserialize;
use std::{fs, fs::File, io::BufReader, path::Path};

#[derive(Deserialize)]
struct AiKeywords {
    keywords: Vec<String>,
}

lazy_static::lazy_static! {
    static ref AI_KEYWORDS: Vec<String> = {
        let json_str = include_str!("../resources/ai_keywords.json");
        let k: AiKeywords = serde_json::from_str(json_str)
            .expect("Invalid ai_keywords.json");
        k.keywords
    };
}

pub struct AiImageDetector {
    path: String,
}

impl AiImageDetector {
    /// Creates a new `AiImageDetector` instance.
    ///
    /// # Arguments
    /// * `path` - The file path to the image.
    ///
    /// # Returns
    /// A new `AiImageDetector` instance.
    pub fn new(path: String) -> Self {
        Self { path }
    }

    /// Analyzes the image file specified in the constructor and determines if it is likely to be AI-generated.
    ///
    /// This function checks for C2PA metadata, AI-related keywords in the image's EXIF data, and AI-related keywords within PNG text chunks and WebP XMP data.
    ///
    /// # Arguments
    /// * `self` - The `AiImageDetector` instance.
    ///
    /// # Returns
    /// `Result<AiDetectionResult, String>` - A result containing an `AiDetectionResult` indicating whether the image is considered AI-generated and its detected format, or an error message if the image cannot be analyzed.
    pub fn detect(&self) -> Result<AiDetectionResult, String> {
        let p = Path::new(&self.path);
        let mut format = "unknown".to_string();
        if let Ok(reader) = ImageReader::open(&p) {
            if let Some(f) = reader.format() {
                format = f.to_mime_type().to_string();
            }
        }

        let is_ai_generated = self.detect_c2pa(p)
            || self.detect_exif(p)
            || (format.contains("png") && self.detect_png(p))
            || (format.contains("webp") && self.detect_webp(p));

        Ok(AiDetectionResult {
            is_ai_generated,
            format,
        })
    }

    /// Detects C2PA metadata in the image.
    ///
    /// # Arguments
    /// * `path` - The path to the image file.
    ///
    /// # Returns
    /// `bool` - True if C2PA data indicating AI generation is found, false otherwise.
    fn detect_c2pa(&self, path: &Path) -> bool {
        matches!(detect_c2pa(path), Ok((true, _)))
    }

    /// Detects AI-related keywords in the image's EXIF data.
    ///
    /// # Arguments
    /// * `path` - The path to the image file.
    ///
    /// # Returns
    /// `bool` - True if AI-related keywords are found in EXIF data, false otherwise.
    fn detect_exif(&self, path: &Path) -> bool {
        if let Ok(file) = File::open(path) {
            let mut buf = BufReader::new(file);
            if let Ok(exif) = ExifReader::new().read_from_container(&mut buf) {
                for field in exif.fields() {
                    let tag = field.tag.to_string().to_lowercase();
                    let value = field
                        .display_value()
                        .with_unit(&exif)
                        .to_string()
                        .to_lowercase();
                    if AI_KEYWORDS
                        .iter()
                        .any(|k| tag.contains(k) || value.contains(k))
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Detects AI-related keywords within PNG text chunks.
    ///
    /// # Arguments
    /// * `path` - The path to the PNG image file.
    ///
    /// # Returns
    /// `bool` - True if AI-related keywords are found in PNG text chunks, false otherwise.
    fn detect_png(&self, path: &Path) -> bool {
        if let Ok(bytes) = fs::read(path) {
            let text_chunks = extract_png_text_chunks(&bytes);
            for (key, val) in &text_chunks {
                let lower_key = key.to_lowercase();
                let lower_val = val.to_lowercase();
                if AI_KEYWORDS
                    .iter()
                    .any(|k| lower_key.contains(k) || lower_val.contains(k))
                {
                    return true;
                }
            }
        }
        false
    }

    /// Detects AI-related keywords within WebP XMP data.
    ///
    /// # Arguments
    /// * `path` - The path to the WebP image file.
    ///
    /// # Returns
    /// `bool` - True if AI-related keywords are found in WebP XMP data, false otherwise.
    fn detect_webp(&self, path: &Path) -> bool {
        if let Ok(bytes) = fs::read(path) {
            if let Some(xmp_xml) = extract_webp_xmp(&bytes) {
                let lower_xmp = xmp_xml.to_lowercase();
                if AI_KEYWORDS.iter().any(|k| lower_xmp.contains(k)) {
                    return true;
                }
            }
        }
        false
    }
}

/// Detects if an image might be AI-generated based on its metadata.
///
/// This command analyzes various metadata aspects of an image, including C2PA data,
/// EXIF tags, and text chunks in PNGs or XMP in WebP files, to infer if it was
/// generated by AI.
///
/// # Arguments
/// * `path` - The file path to the image as a `String`.
///
/// # Returns
/// `Result<AiDetectionResult, String>` - A result containing an `AiDetectionResult`
/// which indicates whether the image is considered AI-generated and its detected format.
#[tauri::command]
pub fn detect_ai_image(path: String) -> Result<AiDetectionResult, String> {
    AiImageDetector::new(path).detect()
}
