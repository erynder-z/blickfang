export interface AiDetectionResult {
  is_ai_generated: boolean;
  reasons: string[];
  exif_data: Record<string, string> | null;
  format: string;
  png_metadata: Record<string, string> | null;
  has_c2pa: boolean;
  c2pa_manifest: string | null;
}

export interface ImageMetadata {
  image_data: string;
  exif_data: string;
  width: number;
  height: number;
  aspect_ratio: string;
  format: string;
  color_depth: number | null;
  file_size: number;
}