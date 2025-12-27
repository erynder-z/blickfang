use base64::Engine;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Rgb};
use rusttype::{Font, Scale};
use std::io::Cursor;

static ASCII_CHARS: &str = "@#W$9876543210?!abc;:+=-,._ ";

/// Converts an image to its ASCII art representation.
///
/// This command reads the image file from the given path, corrects the image orientation if necessary,
/// creates an ASCII art representation of the image, and encodes it to a base64 string.
///
/// # Parameters
///
/// * `path` - The path to the image file to be converted.
///
/// # Returns
///
/// `Result<String, String>` - A result containing the base64 encoded ASCII art string if successful,
/// or an error string if the operation failed.
#[tauri::command]
pub fn convert_image_to_ascii_art(path: String) -> Result<String, String> {
    let file_bytes = std::fs::read(&path).map_err(|e| format!("Failed to read file: {e}"))?;
    let mut img = image::open(&path).map_err(|e| format!("Failed to open image: {e}"))?;
    img = correct_image_orientation(img, &file_bytes);
    let ascii_img = create_ascii_image(&img);
    encode_image_to_base64(&ascii_img)
}

/// Corrects the image orientation according to the EXIF data.
///
/// The function takes a DynamicImage and the file bytes as input, and returns a new DynamicImage with the correct orientation.
///
/// If the EXIF data does not contain the Orientation tag, or if the tag value is not recognized, the function returns the original image.
///
/// The function supports the following Orientation tag values:
///
/// * 1: The 0th row of pixels is on the top side of the image and the 0th column is on the left side.
/// * 2: The 0th row of pixels is on the right side of the image and the 0th column is on the top side.
/// * 3: The 0th row of pixels is on the bottom side of the image and the 0th column is on the right side.
/// * 4: The 0th row of pixels is on the left side of the image and the 0th column is on the bottom side.
/// * 5: The 0th row of pixels is on the right side of the image and the 0th column is on the top side.
/// * 6: The 0th row of pixels is on the top side of the image and the 0th column is on the right side.
/// * 7: The 0th row of pixels is on the bottom side of the image and the 0th column is on the left side.
/// * 8: The 0th row of pixels is on the left side of the image and the 0th column is on the top side.
fn correct_image_orientation(img: DynamicImage, file_bytes: &[u8]) -> DynamicImage {
    if let Ok(exif_data) = exif::Reader::new().read_from_container(&mut Cursor::new(file_bytes)) {
        if let Some(orientation_field) =
            exif_data.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
        {
            if let Some(orientation) = orientation_field.value.get_uint(0) {
                return match orientation {
                    2 => img.fliph(),
                    3 => img.rotate180(),
                    4 => img.flipv(),
                    5 => img.rotate90().fliph(),
                    6 => img.rotate90(),
                    7 => img.rotate270().fliph(),
                    8 => img.rotate270(),
                    _ => img,
                };
            }
        }
    }
    img
}

/// Encodes a given image to a base64 string.
///
/// This function writes the given image to a PNG buffer, encodes the buffer to a base64 string,
/// and returns the resulting string. The returned string is in the format of a data URI,
/// suitable for use in a web page's `<img>` tag.
///
/// # Parameters
///
/// * `img` - The image to be encoded.
///
/// # Returns
///
/// `Result<String, String>` - A result containing the base64 encoded image string if successful,
/// or an error string if the operation failed.
fn encode_image_to_base64(img: &DynamicImage) -> Result<String, String> {
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png)
        .map_err(|e| format!("Failed to encode image: {e}"))?;
    let base64 = base64::engine::general_purpose::STANDARD.encode(buffer.into_inner());
    Ok(format!("data:image/png;base64,{base64}"))
}

/// Creates an ASCII art representation of a given image.
///
/// This function converts an input `DynamicImage` into a new `DynamicImage`
/// composed of ASCII characters, where each character's brightness corresponds
/// to the average brightness of a cell in the original image.
///
/// # Arguments
/// * `img` - A reference to the input `DynamicImage` to be converted.
///
/// # Returns
/// `DynamicImage` - A new `DynamicImage` representing the ASCII art version of the input image.
fn create_ascii_image(img: &DynamicImage) -> DynamicImage {
    let cell_w: u32 = 10;
    let cell_h: u32 = 18;
    let gamma: f32 = 0.6;
    let bg = Rgb([0, 0, 0]);

    let (w, h) = img.dimensions();
    let cols = (w / cell_w).max(1);
    let rows = (h / cell_h).max(1);

    let font_data = include_bytes!("../../../src/assets/SUSEMono-ExtraBold.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Failed to load font");

    let scale = Scale::uniform(cell_h as f32 * 1.15);
    let v_metrics = font.v_metrics(scale);

    let mut out = ImageBuffer::from_pixel(cols * cell_w, rows * cell_h + cell_h, bg);

    for y in 0..rows {
        for x in 0..cols {
            let avg_color = average_cell(img, x, y, cell_w, cell_h);
            let luma = perceptual_luminance(&avg_color, gamma);
            let ch = brightness_to_char(luma);

            let glyph = font.glyph(ch).scaled(scale).positioned(rusttype::point(
                (x * cell_w) as f32,
                (y * cell_h) as f32 + v_metrics.ascent,
            ));

            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, alpha| {
                    let px = bb.min.x + gx as i32;
                    let py = bb.min.y + gy as i32;

                    if px >= 0 && py >= 0 && (px as u32) < out.width() && (py as u32) < out.height()
                    {
                        let alpha = alpha.clamp(0.0, 1.0);

                        let blended = blend(avg_color, bg, alpha);
                        out.put_pixel(px as u32, py as u32, blended);
                    }
                });
            }
        }
    }

    DynamicImage::ImageRgb8(out)
}

/// Calculates the average RGB color of a specified cell within an image.
///
/// # Arguments
/// * `img` - The `DynamicImage` from which to extract the cell.
/// * `cx` - The column index of the cell.
/// * `cy` - The row index of the cell.
/// * `cw` - The width of the cell.
/// * `ch` - The height of the cell.
///
/// # Returns
/// `Rgb<u8>` - The average RGB color of the pixels within the specified cell.
fn average_cell(img: &DynamicImage, cx: u32, cy: u32, cw: u32, ch: u32) -> Rgb<u8> {
    let mut r = 0u32;
    let mut g = 0u32;
    let mut b = 0u32;
    let mut count = 0u32;

    for dy in 0..ch {
        for dx in 0..cw {
            let x = cx * cw + dx;
            let y = cy * ch + dy;

            if x < img.width() && y < img.height() {
                let p = img.get_pixel(x, y);
                r += p[0] as u32;
                g += p[1] as u32;
                b += p[2] as u32;

                count += 1;
            }
        }
    }

    Rgb([(r / count) as u8, (g / count) as u8, (b / count) as u8])
}

/// Calculates the standard luminance of an RGB pixel.
///
/// # Arguments
/// * `p` - A reference to an `Rgb<u8>` pixel.
///
/// # Returns
/// `f32` - The luminance value of the pixel, ranging from 0.0 to 255.0.
fn luminance(p: &Rgb<u8>) -> f32 {
    0.2126 * p[0] as f32 + 0.7152 * p[1] as f32 + 0.0722 * p[2] as f32
}

/// Calculates the perceptual luminance of an RGB pixel with gamma correction.
///
/// This provides a more perceptually accurate brightness value.
///
/// # Arguments
/// * `p` - A reference to an `Rgb<u8>` pixel.
/// * `gamma` - The gamma correction factor.
///
/// # Returns
/// `f32` - The perceptual luminance value of the pixel.
fn perceptual_luminance(p: &Rgb<u8>, gamma: f32) -> f32 {
    let l = luminance(p) / 255.0;
    (l.powf(gamma)) * 255.0
}

/// Maps a brightness (luminance) value to a character from `ASCII_CHARS`.
///
/// Brighter values map to characters earlier in the string, and darker values to characters later in the string.
///
/// # Arguments
/// * `luma` - The luminance value (0.0 - 255.0).
///
/// # Returns
/// `char` - The ASCII character corresponding to the brightness.
fn brightness_to_char(luma: f32) -> char {
    let idx = (luma / 255.0 * (ASCII_CHARS.len() - 1) as f32)
        .round()
        .clamp(0.0, (ASCII_CHARS.len() - 1) as f32) as usize;

    ASCII_CHARS.as_bytes()[idx] as char
}

/// Blends a foreground color with a background color using a given alpha value.
///
/// # Arguments
/// * `fg` - The foreground `Rgb<u8>` color.
/// * `bg` - The background `Rgb<u8>` color.
/// * `alpha` - The alpha value for blending (0.0 for full background, 1.0 for full foreground).
///
/// # Returns
/// `Rgb<u8>` - The blended `Rgb<u8>` color.
fn blend(fg: Rgb<u8>, bg: Rgb<u8>, alpha: f32) -> Rgb<u8> {
    Rgb([
        (fg[0] as f32 * alpha + bg[0] as f32 * (1.0 - alpha)) as u8,
        (fg[1] as f32 * alpha + bg[1] as f32 * (1.0 - alpha)) as u8,
        (fg[2] as f32 * alpha + bg[2] as f32 * (1.0 - alpha)) as u8,
    ])
}
