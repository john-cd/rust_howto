// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Laying out rich text with `parley`.

use std::sync::Arc;

use parley::Alignment;
use parley::AlignmentOptions;
use parley::FontContext;
use parley::Layout;
use parley::LayoutContext;
use parley::StyleProperty;
use parley::style::FontFamily;

// Parley is a rich text layout engine.
// It handles complex text shaping, line breaking, and bidirectional text.

/// Creates an isolated font context containing the bundled Roboto font.
fn font_context() -> Result<FontContext, &'static str> {
    let mut font_cx = FontContext::default();
    let font_data = include_bytes!("../../assets/Roboto-Regular.ttf");
    let registered_fonts = font_cx.collection.register_fonts(
        parley::fontique::Blob::new(Arc::new(font_data.as_slice())),
        None,
    );

    if registered_fonts.is_empty() {
        return Err("the bundled Roboto font could not be registered");
    }

    Ok(font_cx)
}

/// Builds a styled layout and breaks the text at the requested width.
fn layout_text(
    text: &str,
    max_width: f32,
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<[u8; 4]>,
) -> Layout<[u8; 4]> {
    let mut builder = layout_cx.ranged_builder(font_cx, text, 1.0, true);
    builder
        .push_default(StyleProperty::FontFamily(FontFamily::named("Roboto")));
    builder.push_default(StyleProperty::FontSize(16.0));

    let mut layout = builder.build(text);
    layout.break_all_lines(Some(max_width));
    layout.align(Alignment::Start, AlignmentOptions::default());
    layout
}

/// Builds a layout for a text, breaks it into lines, and prints its glyph runs.
fn main() {
    let text = "Hello, Parley! This is a rich text layout example in Rust.";
    let mut font_cx = font_context().expect("the bundled Roboto font is valid");
    let mut layout_cx: LayoutContext<[u8; 4]> = LayoutContext::new();
    let layout = layout_text(text, 300.0, &mut font_cx, &mut layout_cx);

    println!("Layout size: {}x{}", layout.width(), layout.height());

    for line in layout.lines() {
        println!("Line at y: {}", line.metrics().baseline);
        for item in line.items() {
            if let parley::PositionedLayoutItem::GlyphRun(run) = item {
                println!("  Glyph run with {} glyphs", run.glyphs().count());
            }
        }
    }
}
// ANCHOR_END: example

/// Exposes this example to the crate's dispatcher.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that Parley wraps text and respects the configured width.
    #[test]
    fn wraps_text_at_narrow_width() {
        let text = "Parley lays out this longer sentence using the bundled Roboto font.";
        let max_width = 100.0;
        let mut font_cx =
            font_context().expect("the bundled Roboto font is valid");
        let mut layout_cx: LayoutContext<[u8; 4]> = LayoutContext::new();
        let layout = layout_text(text, max_width, &mut font_cx, &mut layout_cx);

        assert!(layout.lines().count() > 1);
        assert!(layout.width() <= max_width);
    }

    /// Runs the deterministic Parley example entry point.
    #[test]
    fn test_parley_example_entry_point() {
        main();
    }
}
