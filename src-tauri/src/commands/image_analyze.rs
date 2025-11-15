use crate::utils::image_processing::{detect_c2pa, extract_png_text_chunks, extract_webp_xmp};
use exif::Reader as ExifReader;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::BufReader, path::Path};

#[derive(Serialize)]
pub struct AiDetectionResult {
    pub is_ai_generated: bool,
    pub format: String,
}

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
    pub fn new(path: String) -> Self {
        Self { path }
    }

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

    fn detect_c2pa(&self, path: &Path) -> bool {
        matches!(detect_c2pa(path), Ok((true, _)))
    }

    fn detect_exif(&self, path: &Path) -> bool {
        if let Ok(file) = File::open(path) {
            let mut buf = BufReader::new(file);
            if let Ok(exif) = ExifReader::new().read_from_container(&mut buf) {
                for field in exif.fields() {
                    let tag = field.tag.to_string().to_lowercase();
                    let value = field.display_value().with_unit(&exif).to_string().to_lowercase();
                    if AI_KEYWORDS.iter().any(|k| tag.contains(k) || value.contains(k)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn detect_png(&self, path: &Path) -> bool {
        if let Ok(bytes) = fs::read(path) {
            let text_chunks = extract_png_text_chunks(&bytes);
            for (key, val) in &text_chunks {
                let lower_key = key.to_lowercase();
                let lower_val = val.to_lowercase();
                if AI_KEYWORDS.iter().any(|k| lower_key.contains(k) || lower_val.contains(k)) {
                    return true;
                }
            }
        }
        false
    }

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

#[tauri::command]
pub fn detect_ai_image(path: String) -> Result<AiDetectionResult, String> {
    AiImageDetector::new(path).detect()
}
