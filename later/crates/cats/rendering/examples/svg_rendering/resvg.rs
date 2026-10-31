// ANCHOR: example
//! Parses an SVG and rasterizes it to a PNG with `resvg`.
//!
//! SVG is an XML format that describes drawings as shapes and paths instead of
//! pixels, so it scales without blurring. Rasterizing turns it into a pixel
//! image, such as a PNG, for places that cannot display SVG.

const SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="160" height="100" viewBox="0 0 160 100">
  <rect width="160" height="100" fill="#f4f1ea" />
  <circle cx="80" cy="50" r="30" fill="#d34f42" />
  <path d="M80 25v50M55 50h50" stroke="#ffffff" stroke-width="5" stroke-linecap="round" />
</svg>
"##;

/// Renders the sample SVG to a PNG file in the system temporary directory.
fn render_svg() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pixmap = render_svg_to_pixmap()?;
    let width = pixmap.width();
    let height = pixmap.height();

    let output_path = std::env::temp_dir().join("rust_howto_resvg_example.png");
    pixmap.save_png(&output_path)?;
    println!(
        "Rendered {}x{} PNG to {}",
        width,
        height,
        output_path.display()
    );
    Ok(())
}

/// Renders the sample SVG to an in-memory pixmap.
///
/// # Errors
///
/// Returns an error if the SVG cannot be parsed or the pixmap cannot be
/// created.
fn render_svg_to_pixmap()
-> Result<resvg::tiny_skia::Pixmap, Box<dyn std::error::Error + Send + Sync>> {
    let tree =
        resvg::usvg::Tree::from_str(SVG, &resvg::usvg::Options::default())?;
    let size = tree.size().to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "SVG viewport is too large to allocate as a pixmap",
            )
        })?;

    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    Ok(pixmap)
}
// ANCHOR_END: example

/// Runs the rendering example.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    render_svg()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the sample scene renders to the expected in-memory size.
    #[test]
    fn test_resvg_renders_sample_to_expected_dimensions()
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let pixmap = render_svg_to_pixmap()?;
        assert_eq!((pixmap.width(), pixmap.height()), (160, 100));
        Ok(())
    }
}
