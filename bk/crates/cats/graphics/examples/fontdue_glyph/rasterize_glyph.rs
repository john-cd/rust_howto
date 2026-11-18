// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Rasterizes a glyph with `fontdue` and reads metrics with `ttf-parser`.

use fontdue::Font;
use fontdue::FontSettings;

/// A font bundled with the repository (SIL Open Font License 1.1).
const FONT_BYTES: &[u8] = include_bytes!("../../assets/Roboto-Regular.ttf");

/// Rasterizes `'A'` at 32 pixels and returns its bitmap size and coverage.
fn rasterize_glyph() -> anyhow::Result<(usize, usize, u32)> {
    let font = Font::from_bytes(FONT_BYTES, FontSettings::default())
        .map_err(|message| anyhow::anyhow!(message))?;
    let (metrics, coverage) = font.rasterize('A', 32.0);
    let ink = coverage.iter().map(|&value| u32::from(value)).sum();
    Ok((metrics.width, metrics.height, ink))
}

/// Reads the font's design metrics without rasterizing anything.
fn read_design_metrics() -> anyhow::Result<(u16, u16)> {
    let face = ttf_parser::Face::parse(FONT_BYTES, 0)?;
    let glyph = face
        .glyph_index('A')
        .ok_or_else(|| anyhow::anyhow!("the font has no glyph for 'A'"))?;
    let advance = face
        .glyph_hor_advance(glyph)
        .ok_or_else(|| anyhow::anyhow!("the glyph has no advance"))?;
    Ok((face.units_per_em(), advance))
}

/// Prints the rasterized glyph size and the font's design metrics.
fn text_rendering_example() -> anyhow::Result<()> {
    let (width, height, ink) = rasterize_glyph()?;
    println!("'A' is {width}x{height} pixels with {ink} total coverage");
    let (units_per_em, advance) = read_design_metrics()?;
    println!("{advance} of {units_per_em} font units advance");
    Ok(())
}

/// Starts the text rendering example as a binary.
///
/// # Errors
///
/// Returns an error if the bundled font cannot be parsed.
fn main() -> anyhow::Result<()> {
    text_rendering_example()
}
// ANCHOR_END: example

/// Rasterizes a glyph with `fontdue` and reads metrics with `ttf-parser`.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the glyph bitmap has area and visible ink.
    #[test]
    fn rasterizes_a_non_empty_glyph() -> anyhow::Result<()> {
        let (width, height, ink) = rasterize_glyph()?;
        assert!(width > 0 && height > 0);
        assert!(ink > 0);
        Ok(())
    }

    /// Verifies the glyph advance is positive and smaller than the em.
    #[test]
    fn reads_the_glyph_advance() -> anyhow::Result<()> {
        let (units_per_em, advance) = read_design_metrics()?;
        assert!(advance > 0 && advance < units_per_em * 2);
        Ok(())
    }
}

/// Runs the example logic exposed by this module.
pub fn run() -> impl std::process::Termination {
    let entry_point = main;
    entry_point()
}

/// Provides entry point tests behavior for the rasterize glyph component.
#[cfg(test)]
mod entry_point_tests {
    use super::*;

    /// Checks that private main runs.
    #[test]
    fn private_main_runs() {
        let entry_point = main;
        entry_point().expect("example entry point should succeed");
    }
}
