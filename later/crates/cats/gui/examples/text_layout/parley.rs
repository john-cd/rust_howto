// ANCHOR: example
use parley::{Alignment, AlignmentOptions, FontContext, LayoutContext, StyleProperty};

// Parley is a rich text layout engine.
// It handles complex text shaping, line breaking, and bidirectional text.

pub fn main() {
    // Initialize font context
    let mut font_cx = FontContext::new();
    // Initialize layout context with default brush type [u8; 4]
    let mut layout_cx: LayoutContext<[u8; 4]> = LayoutContext::new();

    let text = "Hello, Parley! This is a rich text layout example in Rust.";

    // Use ranged_builder for flat list of spans or simple text
    let mut builder = layout_cx.ranged_builder(&mut font_cx, text, 1.0, true);

    // Set default style for the entire text
    builder.push_default(StyleProperty::FontSize(16.0));

    // Build the layout
    let mut layout = builder.build(text);

    // Break lines at a specific width
    layout.break_all_lines(Some(300.0));

    // Align the text
    layout.align(Alignment::Start, AlignmentOptions::default());

    println!("Layout size: {}x{}", layout.width(), layout.height());

    // Iterate through lines and items (glyph runs)
    for line in layout.lines() {
        let metrics = line.metrics();
        println!("Line at y: {}", metrics.baseline);
        for item in line.items() {
            if let parley::PositionedLayoutItem::GlyphRun(run) = item {
                println!("  Glyph run with {} glyphs", run.glyphs().count());
            }
        }
    }
}
// ANCHOR_END: example
