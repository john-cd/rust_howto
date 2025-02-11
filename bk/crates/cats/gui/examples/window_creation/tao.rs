// // ANCHOR: example
// use tao::event::Event;
// use tao::event::WindowEvent;
// use tao::event_loop::ControlFlow;
// use tao::event_loop::EventLoop;
// use tao::window::WindowBuilder;

// fn main() {
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .with_title("Tao Example")
//         .build(&event_loop)
//         .unwrap();

//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Wait;

//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 ..
//             } => *control_flow = ControlFlow::Exit,
//             Event::WindowEvent {
//                 event: WindowEvent::Resized(physical_size),
//                 ..
//             } => {
//                 println!("Resized to: {:?}", physical_size);
//                 // Handle resize event here (e.g., redraw graphics)
//             }
//             Event::WindowEvent {
//                 event: WindowEvent::Moved(position),
//                 ..
//             } => {
//                 println!("Moved to: {:?}", position);
//             }
//             Event::WindowEvent {
//                 event: WindowEvent::KeyboardInput { input, .. },
//                 ..
//             } => {
//                 if let Some(keycode) = input.virtual_keycode {
//                     println!("Key pressed: {:?}", keycode);
//                     // Handle keyboard input here
//                 }
//             }
//             Event::WindowEvent {
//                 event: WindowEvent::MouseInput { state, button, .. },
//                 ..
//             } => {
//                 println!("Mouse {:?} pressed: {:?}", button, state);
//             }
//             Event::MainEventsCleared => {
//                 // Application update logic goes here.
//                 // println!("Main Events Cleared");
//                 window.request_redraw();
//             }
//             Event::RedrawRequested(_window_id) => {
//                 // Redraw the application.
//                 // println!("Redraw Requested");
//             }
//             _ => (),
//         }
//     });
// }
// // ANCHOR_END: example

pub fn main() {}
// // [P1](https://github.com/john-cd/rust_howto/issues/789)
