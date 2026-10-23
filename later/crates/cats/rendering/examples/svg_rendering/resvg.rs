// ANCHOR: example
//! Parses an SVG and rasterizes it to a PNG with `resvg`.

const SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="160" height="100" viewBox="0 0 160 100">
  <rect width="160" height="100" fill="#f4f1ea" />
  <circle cx="80" cy="50" r="30" fill="#d34f42" />
  <path d="M80 25v50M55 50h50" stroke="#ffffff" stroke-width="5" stroke-linecap="round" />
</svg>
"##;

/// Renders the sample SVG to a PNG file in the system temporary directory.
fn render_svg() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

    let output_path = std::env::temp_dir().join("rust_howto_resvg_example.png");
    pixmap.save_png(&output_path)?;
    println!(
        "Rendered {}x{} PNG to {}",
        size.width(),
        size.height(),
        output_path.display()
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the rendering example.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    render_svg()
}
