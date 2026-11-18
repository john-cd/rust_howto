// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Scales a glyph outline with `skrifa` and rasterizes it with `tiny-skia`.

use anyhow::Context;
use anyhow::anyhow;
use skrifa::instance::LocationRef;
use skrifa::instance::Size;
use skrifa::outline::DrawSettings;
use skrifa::outline::OutlinePen;
use skrifa::prelude::FontRef;
use skrifa::prelude::MetadataProvider;
use tiny_skia::Color;
use tiny_skia::FillRule;
use tiny_skia::Paint;
use tiny_skia::PathBuilder;
use tiny_skia::Pixmap;
use tiny_skia::Transform;

/// A font bundled with the repository under the SIL Open Font License 1.1.
const FONT_BYTES: &[u8] = include_bytes!("../../assets/Roboto-Regular.ttf");

/// Glyph size in pixels passed to the outline scaler.
const FONT_SIZE: f32 = 32.0;
/// Width in pixels of the destination pixmap.
const PIXMAP_WIDTH: u32 = 48;
/// Height in pixels of the destination pixmap.
const PIXMAP_HEIGHT: u32 = 48;
/// Destination row at which the glyph baseline is drawn.
const BASELINE_Y: f32 = 40.0;

/// Collects a scaled glyph outline as a tiny-skia path.
#[derive(Default)]
struct GlyphPath(
    /// Receives contour commands in the font’s y-up coordinate system.
    PathBuilder,
);

impl OutlinePen for GlyphPath {
    /// Starts a contour at the scaled outline coordinates.
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, y);
    }

    /// Adds a straight segment to the contour.
    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, y);
    }

    /// Adds a quadratic Bézier segment using one control point.
    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.0.quad_to(cx, cy, x, y);
    }

    /// Adds a cubic Bézier segment using two control points.
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.0.cubic_to(cx0, cy0, cx1, cy1, x, y);
    }

    /// Closes the contour so it can be filled.
    fn close(&mut self) {
        self.0.close();
    }
}

/// Rasterizes `'A'` at 32 pixels and returns its pixmap size, coverage, and advance.
fn rasterize_glyph() -> anyhow::Result<(u32, u32, u32, f32)> {
    let font = FontRef::new(FONT_BYTES).context("parse bundled font")?;
    let glyph_id = font
        .charmap()
        .map('A')
        .ok_or_else(|| anyhow!("the font has no glyph for 'A'"))?;
    let glyph = font
        .outline_glyphs()
        .get(glyph_id)
        .ok_or_else(|| anyhow!("the font has no scalable outline for 'A'"))?;

    let mut outline = GlyphPath::default();
    let metrics = glyph
        .draw(
            DrawSettings::unhinted(Size::new(FONT_SIZE), LocationRef::default()),
            &mut outline,
        )
        .context("scale glyph outline")?;
    let path = outline
        .0
        .finish()
        .ok_or_else(|| anyhow!("the glyph outline is empty"))?;

    let mut pixmap = Pixmap::new(PIXMAP_WIDTH, PIXMAP_HEIGHT)
        .ok_or_else(|| anyhow!("could not allocate glyph pixmap"))?;
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(220, 90, 40, 255));
    // Flip the y-up outline before translating it to the pixmap baseline.
    let font_to_pixels = Transform::from_row(1.0, 0.0, 0.0, -1.0, 4.0, BASELINE_Y);
    pixmap.fill_path(&path, &paint, FillRule::Winding, font_to_pixels, None);

    let coverage = pixmap
        .pixels()
        .iter()
        .map(|pixel| u32::from(pixel.alpha()))
        .sum();
    let advance = metrics
        .advance_width
        .ok_or_else(|| anyhow!("the glyph has no advance width"))?;
    Ok((PIXMAP_WIDTH, PIXMAP_HEIGHT, coverage, advance))
}

/// Prints the rasterized glyph's pixmap coverage and scaled advance.
fn text_rendering_example() -> anyhow::Result<()> {
    let (width, height, coverage, advance) = rasterize_glyph()?;
    println!("'A' uses a {width}x{height} pixmap with {coverage} coverage units");
    println!("its advance is {advance:.1}px at {FONT_SIZE:.0}px");
    Ok(())
}

/// Starts the text rendering example as a binary.
///
/// # Errors
///
/// Returns an error if the bundled font or glyph outline cannot be rendered.
fn main() -> anyhow::Result<()> {
    text_rendering_example()
}
// ANCHOR_END: example

/// Scales and rasterizes a glyph outline.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the glyph bitmap has area and visible ink.
    #[test]
    fn rasterizes_a_non_empty_glyph() -> anyhow::Result<()> {
        let (width, height, coverage, _) = rasterize_glyph()?;
        assert!(width > 0 && height > 0);
        assert!(coverage > 0);
        Ok(())
    }

    /// Verifies the scaled glyph advance is positive and fits the pixmap.
    #[test]
    fn reads_the_glyph_advance() -> anyhow::Result<()> {
        let (_, _, _, advance) = rasterize_glyph()?;
        assert!(advance > 0.0 && advance < PIXMAP_WIDTH as f32);
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
