// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use parley::buffer::Buffer;
// use parley::layout::{Alignment, Layout};
// use parley::render::{FontContext, TextRenderer};
// use parley::style::{FontFace, FontMetrics, Style};

// // - The code loads a font from a file. You must replace
// "path/to/your/font.ttf" with the actual path to a .ttf or .otf font file on
// your system. // - The code applies a Style to the text, including setting the
// font, size, and color. // - The Layout is configured with a fixed width and
// infinite height (f32::INFINITY), enabling text wrapping. // - The example
// iterates through the layout's lines and glyphs, printing their positions and
// glyph IDs. This gives you the raw data you need for rendering. // - The
// example now includes a very basic string representation of the rendered text.
// It creates a 2D vector of characters and sets '#' where glyphs are present.
// // - This is a simple way to visualize the layout within the console.

// fn main() {
//     // Initialize font context (you'll need a font file path)
//     let font_path = "path/to/your/font.ttf"; // Replace with your font path
//     let font_context = FontContext::new();
//     let font = font_context.new_font_from_file(font_path, 0).unwrap();
//     let font_metrics = FontMetrics::new(font, 16.0);

//     // Create a text buffer
//     let text = "Hello, Parley! This is a Rust example.\nIt supports multiple
// lines.";     let mut buffer = Buffer::new(&font_context, &font_metrics);
//     buffer.set_text(text);

//     // Create a style
//     let style = Style {
//         font_face: FontFace::from(font),
//         font_size: 16.0,
//         color: parley::color::Color::rgb(255, 255, 255), // White
//         ..Default::default()
//     };
//     buffer.set_style(0..text.len(), style);

//     // Create a layout
//     let mut layout = Layout::new(&font_context, &font_metrics);
//     layout.set_size(200.0, f32::INFINITY); // Set a width, infinite height
// for wrapping     layout.set_alignment(Alignment::Start);
//     layout.add_buffer(&buffer);
//     layout.layout();

//     // Now you have the layout, you can use a renderer to draw it.
//     // This example provides a basic outline; you'll likely integrate
//     // Parley with a graphics library (e.g., wgpu, piet).

//     println!("Layout Bounds: {:?}", layout.bounds());
//     for line in layout.lines() {
//         println!("Line Bounds: {:?}", line.bounds());
//         for glyph in line.glyphs() {
//             println!(
//                 "Glyph: {:?} at ({}, {})",
//                 glyph.glyph_id, glyph.x, glyph.y
//             );
//         }
//     }

//     // Example of rendering to a very basic string representation
//     let mut rendered_text = String::new();
//     let bounds = layout.bounds();
//     let width = bounds.width.ceil() as usize;
//     let height = bounds.height.ceil() as usize;

//     let mut pixels: Vec<Vec<char>> = vec![vec![' '; width]; height];

//     for line in layout.lines() {
//         for glyph in line.glyphs() {
//             let x = glyph.x.floor() as usize;
//             let y = glyph.y.floor() as usize;
//             if y < height && x < width{
//                 pixels[y][x] = '#';
//             }
//         }
//     }
//     for row in pixels {
//         rendered_text.push_str(&row.iter().collect::<String>());
//         rendered_text.push('\n');
//     }
//     println!("{}", rendered_text);
// }

pub fn main() {}
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/783)
