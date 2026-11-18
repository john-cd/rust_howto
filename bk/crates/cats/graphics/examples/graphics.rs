// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Draw a triangle over a background and write the result as SVG.

use std::io;
use std::io::Write;

/// Writes a 160 by 120 SVG scene to the supplied output.
///
/// # Errors
///
/// Returns an error if the output cannot be written.
fn draw(mut output: impl Write) -> io::Result<()> {
    writeln!(
        output,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 120">
  <rect width="160" height="120" fill="midnightblue"/>
  <polygon points="80,20 140,100 20,100" fill="gold"/>
</svg>"#
    )
}

/// Writes the scene to standard output.
///
/// # Errors
///
/// Returns an error if standard output cannot be written.
fn main() -> io::Result<()> {
    draw(io::stdout().lock())
}
// ANCHOR_END: example

/// Checks SVG painter order so the foreground triangle follows the background.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the triangle is written after the background.
    #[test]
    fn draws_shapes_in_order() -> Result<(), Box<dyn std::error::Error>> {
        let mut output = Vec::new();
        draw(&mut output)?;
        let svg = String::from_utf8(output)?;
        assert!(svg.contains("fill=\"midnightblue\"/>\n  <polygon"));
        Ok(())
    }
}
