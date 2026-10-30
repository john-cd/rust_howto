//! Rasterizes a glyph with `fontdue` and reads metrics with `ttf-parser`.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use fontdue::Font;
use fontdue::FontSettings;

/// A font bundled with the repository (Apache License 2.0).
const FONT_BYTES: &[u8] =
    include_bytes!("../../../gui/assets/Roboto-Regular.ttf");

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
fn main() -> anyhow::Result<()> {
    let (width, height, ink) = rasterize_glyph()?;
    println!("'A' is {width}x{height} pixels with {ink} total coverage");
    let (units_per_em, advance) = read_design_metrics()?;
    println!("{advance} of {units_per_em} font units advance");
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
