// TODO Reviewed ??-??-2026.

// ANCHOR: example
use ::tiny_skia::FillRule;
use ::tiny_skia::Paint;
use ::tiny_skia::PathBuilder;
use ::tiny_skia::Pixmap;
use ::tiny_skia::Stroke;
use ::tiny_skia::Transform;

/// Draws a filled circle and a stroked line into a CPU-backed pixmap.
///
/// The example has no window, display server, or GPU dependency.
fn draw_scene() -> Result<Pixmap, Box<dyn std::error::Error + Send + Sync>> {
    let mut pixmap = Pixmap::new(64, 64).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "could not allocate the 64 by 64 pixmap",
        )
    })?;

    let circle =
        PathBuilder::from_circle(32.0, 32.0, 20.0).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "circle path is empty",
            )
        })?;
    let mut fill = Paint::default();
    fill.set_color_rgba8(220, 90, 40, 255);
    pixmap.fill_path(
        &circle,
        &fill,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    let mut line = PathBuilder::new();
    line.move_to(16.0, 48.0);
    line.line_to(48.0, 16.0);
    let line = line.finish().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "line path is empty",
        )
    })?;
    let mut stroke = Paint::default();
    stroke.set_color_rgba8(255, 255, 255, 255);
    pixmap.stroke_path(
        &line,
        &stroke,
        &Stroke {
            width: 4.0,
            ..Stroke::default()
        },
        Transform::identity(),
        None,
    );

    Ok(pixmap)
}

/// Returns the RGBA components of a pixel, or `None` when it is out of bounds.
fn pixel_rgba(pixmap: &Pixmap, x: u32, y: u32) -> Option<[u8; 4]> {
    pixmap
        .pixel(x, y)
        .map(|pixel| [pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
}

/// Renders the scene and encodes its PNG bytes in memory.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pixmap = draw_scene()?;
    let png = pixmap.encode_png()?;
    let sample = pixel_rgba(&pixmap, 32, 40).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "sample pixel is out of bounds",
        )
    })?;
    println!(
        "Encoded a {}x{} PNG ({} bytes); pixel (32, 40) is RGBA {:?}.",
        pixmap.width(),
        pixmap.height(),
        png.len(),
        sample
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the example from the `2d_raster_graphics` package target.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::draw_scene;
    use super::pixel_rgba;

    /// Verifies circle fill, line stroke, and the transparent pixmap corner.
    #[test]
    fn draws_shapes_into_a_headless_pixmap() {
        let pixmap = draw_scene().expect("the sample scene should render");

        assert_eq!(pixel_rgba(&pixmap, 32, 40), Some([220, 90, 40, 255]));
        assert_eq!(pixel_rgba(&pixmap, 32, 32), Some([255, 255, 255, 255]));
        assert_eq!(pixel_rgba(&pixmap, 4, 4), Some([0, 0, 0, 0]));
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
