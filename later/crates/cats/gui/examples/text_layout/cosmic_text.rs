// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Laying out and rasterizing text with `cosmic-text`.

use std::sync::Arc;

use cosmic_text::Attrs;
use cosmic_text::Buffer;
use cosmic_text::Color;
use cosmic_text::FontSystem;
use cosmic_text::Metrics;
use cosmic_text::Shaping;
use cosmic_text::SwashCache;

// Advanced text handling in a generic way.
// `cosmic_text` provides abstractions for shaping, font fallback,
// layout, rasterization, and editing.

/// Creates a font system containing the bundled Roboto font.
fn new_font_system() -> FontSystem {
    let font_data =
        include_bytes!("../../assets/Roboto-Regular.ttf").as_slice();
    let font_source = cosmic_text::fontdb::Source::Binary(Arc::new(font_data));

    FontSystem::new_with_fonts([font_source])
}

/// Shapes text into a buffer with the requested width.
fn layout_text(font_system: &mut FontSystem, text: &str, width: f32) -> Buffer {
    let mut buffer = Buffer::new(font_system, Metrics::new(14.0, 20.0));
    buffer.set_size(Some(width), Some(600.0));
    buffer.set_text(text, &Attrs::new(), Shaping::Advanced, None);
    buffer.shape_until_scroll(font_system, true);
    buffer
}

/// Lays out a text, prints its lines, and draws its glyphs into a buffer.
fn main() {
    let mut font_system = new_font_system();
    let mut buffer =
        layout_text(&mut font_system, "Hello, Cosmic Text!", 800.0);
    let mut swash_cache = SwashCache::new();

    for run in buffer.layout_runs() {
        println!("Line {} at y={}: {}", run.line_i, run.line_y, run.text);
    }

    buffer.draw(
        &mut font_system,
        &mut swash_cache,
        Color::rgb(0, 0, 0),
        |x, y, w, h, color| {
            // A real application would draw this glyph rectangle to a surface.
            println!("Drawing at ({x}, {y}) size {w}x{h} with color {color:?}");
        },
    );
}
// ANCHOR_END: example

/// Exposes this example to the crate's dispatcher.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a narrow buffer wraps a long string onto multiple lines.
    #[test]
    fn wraps_text_at_narrow_width() {
        let mut font_system = new_font_system();
        let buffer = layout_text(
            &mut font_system,
            "Cosmic Text lays out a long string using its bundled Roboto font.",
            120.0,
        );

        assert!(buffer.layout_runs().count() > 1);
    }

    /// Runs the deterministic cosmic text example entry point.
    #[test]
    fn test_cosmic_text_example_entry_point() {
        main();
    }
}
