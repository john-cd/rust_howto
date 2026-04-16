// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! # Femtovg Example
// //! This example demonstrates how to use the `femtovg` crate for 2D
// //! rendering. It showcases drawing shapes, text, images, and applying
// //! transformations.
// //!
// //! ## Features
// //! - Drawing basic shapes (triangle, rectangle).
// //! - Rendering text with custom fonts.
// //! - Displaying images.
// //! - Applying transformations (translation, rotation).

// use femtovg::Align;
// use femtovg::Baseline;
// use femtovg::Canvas;
// use femtovg::Color;
// use femtovg::FillStyle;
// use femtovg::FontId;
// use femtovg::ImageFlags;
// use femtovg::ImageId;
// use femtovg::Paint;
// use femtovg::Path;
// use femtovg::Renderer;
// use femtovg::TextOptions;
// use femtovg::Transform;
// use minifb::Key;
// use minifb::Window;
// use minifb::WindowOptions;

// fn main() {
//     const WIDTH: usize = 600;
//     const HEIGHT: usize = 400;

//     let mut window =
//         Window::new("femtovg example", WIDTH, HEIGHT,
// WindowOptions::default())             .unwrap();

//     let mut fb = vec![0u32; WIDTH * HEIGHT];

//     let mut renderer = Renderer::new().unwrap();
//     let mut canvas = Canvas::new(renderer); // Create a new canvas

//     // Load a font from a file.
//     let font = include_bytes!("./Roboto-Regular.ttf");
//     let font_id = canvas.add_font(font).unwrap();

//     // Load an image from a file.
//     let mut img: Option<ImageId> = None;
//     let image_data = include_bytes!("./rust-logo-512x512.png");
//     if let Ok(image) = image::load_from_memory(image_data) {
//         let rgba = image.to_rgba8();
//         img = Some(
//             canvas
//                 .create_image(
//                     rgba.width() as usize,
//                     rgba.height() as usize,
//                     rgba.as_raw(),
//                     ImageFlags::empty(),
//                 )
//                 .unwrap(),
//         );
//     } else {
//         println!("Failed to load image");
//     }

//     // Main loop:
//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         canvas.clear_rect(
//             0,
//             0,
//             WIDTH as u32,
//             HEIGHT as u32,
//             Color::rgb(220, 220, 220),
//         );

//         // Draw a triangle.
//         let mut path = Path::new();
//         path.move_to(50.0, 50.0);
//         path.line_to(200.0, 50.0);
//         path.line_to(100.0, 150.0);
//         path.close();

//         let paint = Paint::new(Color::rgb(0, 160, 192));
//         canvas.fill_path(&path, &paint);

//         // Draw some text.
//         canvas.set_font_size(24.0);
//         canvas.set_font(&[font_id]);
//         canvas.set_fill_style(FillStyle::Color(Color::rgb(0, 0, 0)));
//         canvas
//             .fill_text(
//                 10.0,
//                 30.0,
//                 "Hello, Femtovg!",
//                 TextOptions {
//                     align: Align::Left,
//                     baseline: Baseline::Top,
//                     ..Default::default()
//                 },
//             )
//             .unwrap();

//         // Draw an image.
//         if let Some(image_id) = img {
//             canvas.draw_image(
//                 image_id,
//                 250.0,
//                 50.0,
//                 100.0,
//                 100.0,
//                 0.0,
//                 1.0f32,
//                 ImageFlags::empty(),
//             );
//         }

//         // Draw a transformed rectangle.
//         canvas.save();
//         canvas.translate(350.0, 250.0);
//         canvas.rotate(std::f32::consts::PI / 6.0);
//         let mut path2 = Path::new();
//         path2.rect(-50.0, -50.0, 100.0, 100.0);
//         canvas.fill_path(&path2, &Paint::new(Color::rgba(255, 0, 0, 128)));
//         canvas.restore();

//         // Update the window:
//         canvas.flush(&mut fb, WIDTH as u32, HEIGHT as u32);

//         window.update_with_buffer(&fb, WIDTH, HEIGHT).unwrap();
//     }
// }

pub fn main() {}
// // [finish](https://github.com/john-cd/rust_howto/issues/777)
