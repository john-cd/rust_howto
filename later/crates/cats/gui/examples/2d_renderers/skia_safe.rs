// #![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! # Skia Setup
// //!
// //! This section initializes the Skia graphics library, creating a context,
// //! a render target, and a surface. The surface is the drawing area where
// //! Skia will render graphics. It uses OpenGL as the backend.
// //!
// //! # Font Handling
// //!
// //! Loads a font and uses it to render text.
// //!
// //! # Counter and Conditional Rendering
// //!
// //! Implements the counter logic and conditional text rendering.
// //!
// //! # Winit Integration
// //!
// //! Uses winit for window management and event handling.
// //!
// //! # Resizing
// //!
// //! Handles window resizing by recreating the backend render target and
// surface. //!
// //! # Input Handling
// //!
// //! Uses the Up and Down arrow keys to increment and decrement the counter.
// //!
// //! # Redraw Loop
// //!
// //! Uses `request_redraw` to trigger redraws, ensuring the display is
// updated. //!
// //! # Clear Canvas
// //!
// //! Clears the canvas before each redraw to prevent drawing artifacts.
// //!
// //! # Error Handling
// //!
// //! Includes basic error handling for Skia operations.
// //!
// //! # Color Handling
// //!
// //! Uses `skia_safe`'s `Color` type for setting colors.

// use skia_safe::Canvas;
// use skia_safe::Color;
// use skia_safe::Font;
// use skia_safe::FontMgr;
// use skia_safe::Paint;
// use skia_safe::Path;
// use skia_safe::Point;
// use skia_safe::Rect;
// use skia_safe::SurfaceProps;
// use skia_safe::TextBlob;
// use skia_safe::gpu::BackendRenderTarget;
// use skia_safe::gpu::Context;
// use skia_safe::gpu::Surface;
// use winit::dpi::PhysicalSize;
// use winit::event::Event;
// use winit::event::WindowEvent;
// use winit::event_loop::ControlFlow;
// use winit::event_loop::EventLoop;
// use winit::window::WindowBuilder;

// fn main() {
//     // Create a new even loop and window.
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .with_title("Skia Counter")
//         .build(&event_loop)
//         .unwrap();

//     let size = window.inner_size();
//     let (width, height) = (size.width, size.height);

//     let mut gr_context = Context::new_gl(None).unwrap();

//     let mut backend_render_target = BackendRenderTarget::new_gl(
//         (width as i32, height as i32),
//         0,
//         8,
//         &Default::default(),
//     );

//     let surface_props =
//         SurfaceProps::new(Default::default(),
// skia_safe::ColorType::RGBA_8888);     let mut surface =
// Surface::from_backend_render_target(         &mut gr_context,
//         &backend_render_target,
//         skia_safe::SurfaceOrigin::BottomLeft,
//         skia_safe::ColorType::RGBA_8888,
//         None,
//         Some(&surface_props),
//     )
//     .unwrap();

//     let font_manager = FontMgr::new();
//     let font = Font::from_name(
//         "Arial", // Replace with a font available on your system if needed
//         16.0,
//         &font_manager,
//     );
//     let mut counter_value: i32 = 0;

//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Poll;

//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 ..
//             } => *control_flow = ControlFlow::Exit,
//             Event::WindowEvent {
//                 event: WindowEvent::Resized(physical_size),
//                 ..
//             } => {
//                 let (width, height) =
//                     (physical_size.width, physical_size.height);
//                 backend_render_target = BackendRenderTarget::new_gl(
//                     (width as i32, height as i32),
//                     0,
//                     8,
//                     &Default::default(),
//                 );
//                 surface = Surface::from_backend_render_target(
//                     &mut gr_context,
//                     &backend_render_target,
//                     skia_safe::SurfaceOrigin::BottomLeft,
//                     skia_safe::ColorType::RGBA_8888,
//                     None,
//                     Some(&surface_props),
//                 )
//                 .unwrap();
//             }
//             Event::RedrawRequested(_) => {
//                 let canvas = surface.canvas();
//                 canvas.clear(Color::WHITE);

//                 let mut paint = Paint::default();
//                 paint.set_color(Color::BLACK);

//                 let counter_text = format!("Counter: {}", counter_value);
//                 let text_blob =
//                     TextBlob::from_str(&counter_text, &font).unwrap();
//                 canvas.draw_text_blob(&text_blob, (10, 20), &paint);

//                 if counter_value % 2 == 0 {
//                     paint.set_color(Color::GREEN);
//                     let conditional_text = "Counter is even!";
//                     let text_blob =
//                         TextBlob::from_str(conditional_text, &font).unwrap();
//                     canvas.draw_text_blob(&text_blob, (10, 40), &paint);
//                 } else {
//                     paint.set_color(Color::YELLOW);
//                     let conditional_text = "Counter is odd!";
//                     let text_blob =
//                         TextBlob::from_str(conditional_text, &font).unwrap();
//                     canvas.draw_text_blob(&text_blob, (10, 40), &paint);
//                 }
//                 surface.flush_and_submit();
//                 window.request_redraw();
//             }
//             Event::Input(input) => {
//                 use winit::event::KeyboardInput;
//                 if let winit::event::Input::Keyboard(KeyboardInput {
//                     virtual_keycode: Some(key),
//                     state: winit::event::ElementState::Pressed,
//                     ..
//                 }) = input
//                 {
//                     match key {
//                         winit::event::VirtualKeyCode::Up => counter_value +=
// 1,                         winit::event::VirtualKeyCode::Down => {
//                             counter_value -= 1
//                         }
//                         _ => (),
//                     }
//                     window.request_redraw();
//                 }
//             }
//             _ => (),
//         }
//     });
// }

pub fn main() {}
// // [finish](https://github.com/john-cd/rust_howto/issues/786)
