use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ImageMetadata {
    pub image_data: String,
    pub exif_data: String,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: String,
    pub format: String,
    pub color_depth: Option<u8>,
}

#[derive(Serialize)]
pub struct AiDetectionResult {
    pub is_ai_generated: bool,
    pub format: String,
}
