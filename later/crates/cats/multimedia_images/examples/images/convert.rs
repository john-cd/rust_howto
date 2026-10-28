// ANCHOR: example
//! Encode an image to PNG and JPEG in memory.

use std::io::Cursor;

use image::DynamicImage;
use image::ImageFormat;
use image::Rgb;
use image::RgbImage;

/// Encodes a small image in two formats without creating files.
fn main() -> image::ImageResult<()> {
    let image =
        DynamicImage::ImageRgb8(RgbImage::from_pixel(2, 2, Rgb([40, 80, 120])));
    for format in [ImageFormat::Png, ImageFormat::Jpeg] {
        let mut encoded = Cursor::new(Vec::new());
        image.write_to(&mut encoded, format)?;
        println!("{format:?}: {} bytes", encoded.get_ref().len());
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the in-memory image encoding example.
pub fn run() -> image::ImageResult<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms both encoders emit their respective file signatures.
    #[test]
    fn encoded_images_have_expected_signatures() -> image::ImageResult<()> {
        let image = DynamicImage::ImageRgb8(RgbImage::from_pixel(
            2,
            2,
            Rgb([40, 80, 120]),
        ));
        for (format, signature) in [
            (ImageFormat::Png, &b"\x89PNG\r\n\x1a\n"[..]),
            (ImageFormat::Jpeg, &b"\xff\xd8\xff"[..]),
        ] {
            let mut encoded = Cursor::new(Vec::new());
            image.write_to(&mut encoded, format)?;
            assert!(encoded.get_ref().starts_with(signature));
        }
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
