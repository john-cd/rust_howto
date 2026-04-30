// ANCHOR: example
use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};

// Advanced text handling in a generic way.
// `cosmic_text` provides abstractions for shaping, font discovery,
// font fallback, layout, rasterization, and editing.

pub fn main() {
    // Initialize the font system (allows access to any installed system fonts)
    // Create one per application
    let mut font_system = FontSystem::new();
    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new();

    // Define the text and its attributes
    let text = "Hello, Cosmic Text!";
    let attrs = Attrs::new();

    // Text metrics indicate the font size and line height of a buffer
    let metrics = Metrics::new(14.0, 20.0);

    // Create a Buffer provides shaping and layout for a UTF-8 string
    // Create one per text widget
    let mut buffer = Buffer::new(&mut font_system, metrics);

    // Set buffer size
    buffer.set_size(&mut font_system, Some(800.0), Some(600.0));

    // Set text with attributes and shaping strategy
    buffer.set_text(&mut font_system, text, &attrs, Shaping::Advanced);

    // Shape text for rendering
    buffer.shape_until_scroll(&mut font_system, true);

    // Render the buffer
    let text_color = Color::rgb(0, 0, 0);

    buffer.draw(
        &mut font_system,
        &mut swash_cache,
        text_color,
        |x, y, w, h, color| {
            // In a real app, you'd draw a rectangle here to a surface
            println!("Drawing at ({x}, {y}) size {w}x{h} with color {color:?}");
        },
    );
}
// ANCHOR_END: example
