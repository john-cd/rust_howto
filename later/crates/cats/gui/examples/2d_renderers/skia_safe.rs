// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Draw shapes into an in-memory image with `skia-safe` (CPU raster, no GPU or
//! window) and encode it as PNG.

use anyhow::Context as _;
use skia_safe::Color;
use skia_safe::EncodedImageFormat;
use skia_safe::Paint;
use skia_safe::PaintStyle;
use skia_safe::PathBuilder;
use skia_safe::Surface;
use skia_safe::surfaces;

/// Width and height of the image, in pixels.
const SIZE: i32 = 256;

/// Draws a filled circle and a stroked triangle on a white background.
///
/// # Errors
///
/// Returns an error if Skia cannot allocate the surface.
fn draw() -> anyhow::Result<Surface> {
    // A raster surface renders on the CPU into its own pixel buffer.
    let mut surface = surfaces::raster_n32_premul((SIZE, SIZE))
        .context("create a Skia raster surface")?;
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    // A `Paint` holds the color, style and anti-aliasing for each draw call.
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::RED);
    canvas.draw_circle((128, 128), 60.0, &paint);

    // Build a path from line segments, then stroke it.
    let mut builder = PathBuilder::new();
    builder.move_to((20, 236));
    builder.line_to((236, 236));
    builder.line_to((128, 150));
    builder.close();
    paint.set_color(Color::BLUE);
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(4.0);
    canvas.draw_path(&builder.detach(None), &paint);

    Ok(surface)
}

/// Draws the image and returns it encoded as PNG bytes.
///
/// # Errors
///
/// Returns an error if the surface cannot be created or encoded.
fn render_png() -> anyhow::Result<Vec<u8>> {
    let mut surface = draw()?;
    // Take an immutable snapshot of the pixels and encode it.
    let png = surface
        .image_snapshot()
        .encode(None, EncodedImageFormat::PNG, None)
        .context("encode the image as PNG")?;
    Ok(png.as_bytes().to_vec())
}

/// Renders the image and reports the size of the PNG.
fn main() -> anyhow::Result<()> {
    let png = render_png()?;
    println!(
        "Rendered a {SIZE}x{SIZE} image: {} bytes of PNG.",
        png.len()
    );
    // `std::fs::write("skia.png", &png)?;` saves it to disk.
    Ok(())
}
// ANCHOR_END: example

/// Runs the Skia Safe renderer example.
///
/// # Errors
///
/// Returns an error if rendering fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use skia_safe::AlphaType;
    use skia_safe::ColorType;
    use skia_safe::ImageInfo;

    use super::*;

    /// Reads one RGBA pixel back from the surface.
    fn pixel(surface: &mut Surface, x: i32, y: i32) -> [u8; 4] {
        let info = ImageInfo::new(
            (1, 1),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );
        let mut rgba = [0_u8; 4];
        assert!(surface.read_pixels(&info, &mut rgba, 4, (x, y)));
        rgba
    }

    /// The PNG has the PNG signature.
    #[test]
    fn encodes_a_png() -> anyhow::Result<()> {
        let png = render_png()?;
        assert_eq!(png[..8], [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
        Ok(())
    }

    /// The circle is filled with red; the background stays white.
    #[test]
    fn circle_is_filled_with_red() -> anyhow::Result<()> {
        let mut surface = draw()?;
        assert_eq!(pixel(&mut surface, 128, 100), [255, 0, 0, 255]);
        assert_eq!(pixel(&mut surface, 2, 2), [255, 255, 255, 255]);
        Ok(())
    }

    /// Runs the Skia Safe renderer example.
    #[test]
    fn test_skia_safe_renderer_example() -> anyhow::Result<()> {
        run()
    }
}
