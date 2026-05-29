use std::io::Cursor;

use anyhow::{Context, Result};
use fontdue::Font as FdFont;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, Rgb, RgbImage};

use crate::util::human_size;

const THUMBNAIL_SIZE: u32 = 300;
const CAPTION_HEIGHT: u32 = 16;
const FONT_SIZE: f32 = 14.0;
const JPEG_QUALITY: u8 = 95;
const FONT_DATA: &[u8] = include_bytes!("fonts/DejaVuSerif.ttf");

/// Loaded font for thumbnail captions.
pub struct Font(FdFont);

/// Detect image format from raw bytes.
pub fn get_image_ext(data: &[u8]) -> Result<ImageFormat> {
    image::guess_format(data).context("unable to detect image format")
}

/// Load the embedded font.
pub fn get_font() -> Font {
    Font(
        FdFont::from_bytes(FONT_DATA, fontdue::FontSettings::default())
            .expect("embedded font is valid"),
    )
}

/// Generate a JPEG thumbnail with a text caption showing dimensions, format, and file size.
pub fn make_thumbnail(data: &[u8], font: &Font) -> Result<Vec<u8>> {
    let format = get_image_ext(data)?;
    let img = image::load_from_memory(data).context("failed to decode image")?;
    let (orig_w, orig_h) = (img.width(), img.height());

    let resized = img.resize(THUMBNAIL_SIZE, THUMBNAIL_SIZE, FilterType::Lanczos3);
    let rgb = resized.to_rgb8();

    let (w, h) = (rgb.width(), rgb.height());
    let mut canvas = RgbImage::from_pixel(w, h + CAPTION_HEIGHT, Rgb([255, 255, 255]));
    image::imageops::overlay(&mut canvas, &rgb, 0, 0);

    let caption = format!(
        "{orig_w}x{orig_h} ({}) [{}]",
        format_name(format),
        human_size(data.len())
    );
    draw_text(&mut canvas, &font.0, w / 5, h, &caption);

    let mut buf = Cursor::new(Vec::with_capacity(32 * 1024));
    let encoder = JpegEncoder::new_with_quality(&mut buf, JPEG_QUALITY);
    DynamicImage::ImageRgb8(canvas)
        .write_with_encoder(encoder)
        .context("failed to encode thumbnail")?;

    Ok(buf.into_inner())
}

fn format_name(fmt: ImageFormat) -> &'static str {
    match fmt {
        ImageFormat::Png => "PNG",
        ImageFormat::Jpeg => "JPEG",
        ImageFormat::Gif => "GIF",
        ImageFormat::WebP => "WebP",
        ImageFormat::Bmp => "BMP",
        ImageFormat::Tiff => "TIFF",
        _ => "Unknown",
    }
}

#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    reason = "glyph coordinates and pixel blending operate within safe bounds after bounds check"
)]
fn draw_text(canvas: &mut RgbImage, font: &FdFont, x: u32, y: u32, text: &str) {
    let ascent = font
        .horizontal_line_metrics(FONT_SIZE)
        .map_or(FONT_SIZE, |m| m.ascent);
    let baseline_y = y as f32 + ascent;
    let mut pen_x = x as f32;

    for ch in text.chars() {
        let (m, bitmap) = font.rasterize(ch, FONT_SIZE);
        let gx = pen_x as i32 + m.xmin;
        let gy = baseline_y as i32 - m.height as i32 - m.ymin;

        for row in 0..m.height {
            for col in 0..m.width {
                let alpha = bitmap[row * m.width + col];
                if alpha == 0 {
                    continue;
                }
                let px = gx + col as i32;
                let py = gy + row as i32;
                if px >= 0
                    && py >= 0
                    && (px as u32) < canvas.width()
                    && (py as u32) < canvas.height()
                {
                    let (ux, uy) = (px as u32, py as u32);
                    let bg = canvas.get_pixel(ux, uy).0;
                    let a = f32::from(alpha) / 255.0;
                    let blend = |c: u8| (f32::from(c) * (1.0 - a)) as u8;
                    canvas.put_pixel(ux, uy, Rgb(bg.map(blend)));
                }
            }
        }

        pen_x += m.advance_width;
    }
}

#[cfg(test)]
pub(crate) fn create_test_png() -> Vec<u8> {
    let img = RgbImage::from_pixel(100, 100, Rgb([0, 0, 0]));
    let mut buf = Cursor::new(Vec::new());
    DynamicImage::ImageRgb8(img)
        .write_to(&mut buf, ImageFormat::Png)
        .expect("encoding test PNG should never fail");
    buf.into_inner()
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_get_image_ext_png() {
        let data = create_test_png();
        let fmt = get_image_ext(&data).unwrap();
        assert_eq!(fmt, ImageFormat::Png);
    }

    #[test]
    fn test_get_image_ext_invalid() {
        let result = get_image_ext(b"not an image");
        assert!(result.is_err());
    }

    #[test]
    fn test_make_thumbnail() {
        let data = create_test_png();
        let font = get_font();
        let thumb = make_thumbnail(&data, &font).unwrap();
        assert!(!thumb.is_empty());

        let fmt = get_image_ext(&thumb).unwrap();
        assert_eq!(fmt, ImageFormat::Jpeg);
    }

    #[test]
    fn test_format_name() {
        assert_eq!(format_name(ImageFormat::Png), "PNG");
        assert_eq!(format_name(ImageFormat::Jpeg), "JPEG");
        assert_eq!(format_name(ImageFormat::Gif), "GIF");
        assert_eq!(format_name(ImageFormat::WebP), "WebP");
    }
}
