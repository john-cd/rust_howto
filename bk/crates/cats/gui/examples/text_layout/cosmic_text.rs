// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use cosmic_text::Attrs;
// use cosmic_text::Buffer;
// use cosmic_text::BufferLine;
// use cosmic_text::FontSystem;
// use cosmic_text::SwashCache;
// use cosmic_text::Metrics;

// // Advanced text handling in a generic way.
// // `cosmic_text` provides abstractions for shaping, font discovery,
// // font fallback, layout, rasterization, and editing.

// fn main() {
//     // Initialize the font system (allows access to any installed system
// fonts)     // Create one per application
//     let mut font_system = FontSystem::new();
//     // A SwashCache stores rasterized glyphs, create one per application
//     let mut swash_cache = SwashCache::new();

//     // Define the text and its attributes
//     let text = "Hello, Cosmic Text!";
//     let attrs = Attrs::new();

//     // Text metrics indicate the font size and line height of a buffer
//     let metrics = Metrics::new(14.0, 20.0);

//     // Create a Buffer provides shaping and layout for a UTF-8 string
//     // Create one per text widget
//     let mut buffer =
//         Buffer::new(&mut font_system, metrics);
//     buffer.push_str(text);
//     buffer.shape_until_last_break();

//     // Render the buffer
//     let mut output = vec![0u8; 800 * 600 * 4];
//     buffer.draw(
//         &mut swash_cache,
//         &mut output,
//         800,
//         600,
//         FontColor::default(),
//     );

//     // Here you would typically display the output as an image,
//     // but for simplicity, we're just printing the buffer contents.
//     println!("Buffer: {output:?}");
// }

pub fn main() {}
// // [finish](https://github.com/john-cd/rust_howto/issues/774)
