// ANCHOR: example
//! Parses and inspects the normalized SVG tree produced by `usvg`.

const SVG: &str = r##"
<svg xmlns="http://www.w3.org/2000/svg" width="160" height="100" viewBox="0 0 160 100">
  <rect width="160" height="100" fill="#f4f1ea" />
  <circle id="marker" cx="80" cy="50" r="30" fill="#d34f42" />
</svg>
"##;

/// Parses the sample document and reports its viewport and identified node.
fn inspect_svg() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tree = usvg::Tree::from_str(SVG, &usvg::Options::default())?;
    let size = tree.size();
    let has_marker = tree.node_by_id("marker").is_some();

    println!(
        "Normalized SVG viewport: {}x{}",
        size.width(),
        size.height()
    );
    println!("Tree contains the marker node: {has_marker}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the parsing and inspection example.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    inspect_svg()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the sample SVG can be parsed and inspected.
    #[test]
    fn test_usvg_example_entry_point()
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        run()
    }
}
