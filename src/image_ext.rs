use image::codecs::png::PngEncoder;
use image::{imageops, ImageResult, RgbaImage};
#[cfg(feature = "image-codecs")]
use image::codecs::jpeg::JpegEncoder;
#[cfg(feature = "image-codecs")]
use image::codecs::png;

pub fn resize_within_area(img: RgbaImage, max_area: u32) -> RgbaImage {
    let cur_area = img.width() * img.height();
    if max_area >= cur_area {
        return img
    }
    let scale = max_area as f64 / cur_area as f64;
    let target_width = (img.width() as f64 * scale).floor() as u32;
    let target_height = (img.height() as f64 * scale).floor() as u32;
    imageops::resize(&img, target_width, target_height, imageops::FilterType::Triangle)
}

#[cfg(feature = "image-codecs")]
pub fn adaptive_compress(img: RgbaImage, max_size: usize, allow_png: bool) -> ImageResult<Option<Vec<u8>>> {
    let mut buf = vec![];
    if allow_png {
        img.write_with_encoder(&mut PngEncoder::new_with_quality(&mut buf, png::CompressionType::Best, png::FilterType::default()))?;
        if buf.len() <= max_size {
            return Ok(Some(buf))
        }
    }
    for level in 0..9 {
        let quality = 100 - 5 * level;
        buf.clear();
        img.write_with_encoder(&mut JpegEncoder::new_with_quality(&mut buf, quality))?;
        if buf.len() <= max_size {
            return Ok(Some(buf))
        }
    }
    Ok(None)
}