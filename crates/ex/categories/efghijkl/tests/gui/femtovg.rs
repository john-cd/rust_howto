// // ANCHOR: example
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
//         Window::new("femtovg example", WIDTH, HEIGHT, WindowOptions::default())
//             .unwrap();

//     let mut fb = vec![0u32; WIDTH * HEIGHT];

//     let mut renderer = Renderer::new().unwrap();
//     let mut canvas = Canvas::new(renderer);

//     let font = include_bytes!("../assets/Roboto-Regular.ttf"); // Replace with your font path
//     let font_id = canvas.add_font(font).unwrap();

//     let mut img: Option<ImageId> = None;
//     let image_data = include_bytes!("../assets/rust-logo-512x512.png"); // Replace with your image path
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

//     while window.is_open() && !window.is_key_down(Key::Escape) {
//         canvas.clear_rect(
//             0,
//             0,
//             WIDTH as u32,
//             HEIGHT as u32,
//             Color::rgb(220, 220, 220),
//         );

//         // Example drawing
//         let mut path = Path::new();
//         path.move_to(50.0, 50.0);
//         path.line_to(200.0, 50.0);
//         path.line_to(100.0, 150.0);
//         path.close();

//         let paint = Paint::new(Color::rgb(0, 160, 192));
//         canvas.fill_path(&path, &paint);

//         // Example text
//         canvas.set_font_size(24.0);
//         canvas.set_font(&[font_id]);
//         canvas.set_fill_style(FillStyle::Color(Color::rgb(0, 0, 0)));
//         canvas
//             .fill_text(10.0, 30.0, "Hello, Femtovg!", TextOptions {
//                 align: Align::Left,
//                 baseline: Baseline::Top,
//                 ..Default::default()
//             })
//             .unwrap();

//         // Example Image
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

//         // Example transformed drawing
//         canvas.save();
//         canvas.translate(350.0, 250.0);
//         canvas.rotate(std::f32::consts::PI / 6.0);
//         let mut path2 = Path::new();
//         path2.rect(-50.0, -50.0, 100.0, 100.0);
//         canvas.fill_path(&path2, &Paint::new(Color::rgba(255, 0, 0, 128)));
//         canvas.restore();

//         canvas.flush(&mut fb, WIDTH as u32, HEIGHT as u32);

//         window.update_with_buffer(&fb, WIDTH, HEIGHT).unwrap();
//     }
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/777)
