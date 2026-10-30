//! Rasterizes an SVG document at a bounded output size.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use resvg::tiny_skia::Pixmap;
use resvg::tiny_skia::Transform;

const SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="400" height="200" viewBox="0 0 400 200">
  <rect width="400" height="200" fill="#ffffff" />
  <circle cx="100" cy="100" r="80" fill="#d34f42" />
</svg>
"##;

/// Rasterizes `svg` so that its longer side is `max_side` pixels.
///
/// A vector document has no pixel size of its own: the caller picks one, and
/// should bound it when the input is untrusted.
fn rasterize(svg: &str, max_side: u32) -> anyhow::Result<Pixmap> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default())?;
    let size = tree.size();
    let scale = max_side as f32 / size.width().max(size.height());
    let width = (size.width() * scale).ceil() as u32;
    let height = (size.height() * scale).ceil() as u32;

    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| anyhow::anyhow!("invalid output size"))?;
    resvg::render(
        &tree,
        Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    Ok(pixmap)
}

/// Rasterizes the document and reports the output size.
fn main() -> anyhow::Result<()> {
    let pixmap = rasterize(SVG, 100)?;
    println!("rendered at {}x{} pixels", pixmap.width(), pixmap.height());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Verifies the output is scaled to the requested bound.
    #[test]
    fn scales_the_longer_side_to_the_bound() -> anyhow::Result<()> {
        let pixmap = rasterize(SVG, 100)?;
        assert_eq!((pixmap.width(), pixmap.height()), (100, 50));
        Ok(())
    }

    /// Verifies the circle is painted red and the corner stays white.
    #[test]
    fn paints_the_circle() -> anyhow::Result<()> {
        let pixmap = rasterize(SVG, 100)?;
        let center = pixmap.pixel(25, 25).expect("pixel is in bounds");
        let corner = pixmap.pixel(95, 5).expect("pixel is in bounds");
        assert_eq!((center.red(), center.green()), (0xd3, 0x4f));
        assert_eq!((corner.red(), corner.green()), (0xff, 0xff));
        Ok(())
    }

    /// Verifies that malformed markup is reported as an error.
    #[test]
    fn rejects_malformed_svg() {
        assert!(rasterize("<svg", 100).is_err());
    }
}
