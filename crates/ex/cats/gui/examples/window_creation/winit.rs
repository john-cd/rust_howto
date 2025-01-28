// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use winit::event::Event;
// use winit::event::WindowEvent;
// use winit::event_loop::ControlFlow;
// use winit::event_loop::EventLoop;
// use winit::window::WindowBuilder;

// fn main() {
//     // Create an event loop
//     let event_loop = EventLoop::new();

//     // Build the window
//     let window = WindowBuilder::new()
//         .with_title("Hello, winit!")
//         .build(&event_loop)
//         .unwrap();

//     // Run the event loop
//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Wait;

//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 ..
//             } => *control_flow = ControlFlow::Exit,
//             _ => (),
//         }
//     });
// }

pub fn main() {}
// // [P1](https://github.com/john-cd/rust_howto/issues/794)
