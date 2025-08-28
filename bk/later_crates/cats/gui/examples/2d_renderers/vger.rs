// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use vger::canvas::Canvas;
// use vger::canvas::Color;
// use vger::geom::Point;
// use vger::geom::Rect;
// use vger::geom::Size;
// use vger::platform::run;

// fn main() {
//     run(
//         |canvas: &mut Canvas| {
//             let bounds = canvas.bounds();

//             // Background
//             canvas.draw_rect(bounds, Color::DARK_GRAY);

//             // Use Rect::from_center_size to center the red rectangle.
//             let red_rect = Rect::from_center_size(
//                 bounds.center(),
//                 Size::new(200.0, 100.0),
//             );
//             canvas.draw_rect(red_rect, Color::RED);

//             // Blue Circle:
//             canvas.draw_circle(
//                 bounds.center() + Point::new(150.0, -50.0),
//                 50.0,
//                 Color::BLUE,
//             );

//             // Green Line:
//             canvas.draw_line(
//                 bounds.center() + Point::new(-100.0, 50.0),
//                 bounds.center() + Point::new(0.0, 150.0),
//                 Color::GREEN,
//                 5.0, // Line thickness
//             );

//             // Text (requires a font, see below):
//             // canvas.draw_text("Hello, Vger!", bounds.center(),
//             // Color::WHITE);

//             // Load a font (replace with your font path)
//             let font_data = include_bytes!("../assets/Roboto-Regular.ttf");
//             let font = Font::new(font_data).unwrap();

//             // Draw text
//             canvas.draw_text(
//                 "Hello, Vger!",
//                 bounds.center() + Point::new(0.0, -100.0), // Position.
//                 &font,                                     // Font.
//                 32.0,                                      // Font size.
//                 Color::WHITE,
//             );

//             // Path example (triangle)
//             let path = vger::path::Path::new(vec![
//                 bounds.center() + Point::new(-50.0, -50.0),
//                 bounds.center() + Point::new(50.0, -50.0),
//                 bounds.center() + Point::new(0.0, 50.0),
//             ]);
//             canvas.draw_path(&path, Color::YELLOW, 3.0);
//         },
//         Size::new(800.0, 600.0), // Initial window size.
//         "Vger Example",          // Window title.
//     );
// }

pub fn main() {}
// // [finish](https://github.com/john-cd/rust_howto/issues/792)
