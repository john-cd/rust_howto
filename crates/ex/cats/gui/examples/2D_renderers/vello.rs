// // ANCHOR: example
// use vello::Renderer;
// use vello::peniko::Brush;
// use vello::peniko::Color;
// use vello::peniko::Fill;
// use vello::peniko::Stroke;
// // Use winit for window creation and event handling
// use winit::dpi::PhysicalSize;
// use winit::event::Event;
// use winit::event::WindowEvent;
// use winit::event_loop::ControlFlow;
// use winit::event_loop::EventLoop;
// use winit::window::WindowBuilder;

// fn main() -> Result<(), winit::error::OsError> {
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .with_title("Vello Example")
//         .with_inner_size(PhysicalSize::new(512, 512))
//         .build(&event_loop)?;

//     // Initialize the Renderer using pollster::block_on.
//     let mut renderer = pollster::block_on(Renderer::new(&window))?;

//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Poll;
//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 ..
//             } => *control_flow = ControlFlow::Exit,
//             Event::RedrawRequested(_) => {
//                 let width = window.inner_size().width;
//                 let height = window.inner_size().height;

//                 // Use vello::SceneBuilder to create a scene containing a red
// rectangle with a blue stroke and a green circle.                 let mut
// scene = vello::SceneBuilder::new();

//                 // Draw a red rectangle with a blue outline
//                 scene.push_rect(
//                     vello::kurbo::Rect::new(50.0, 50.0, 200.0, 150.0),
//                     Fill::NonZero,
//                     &Brush::Solid(Color::RED),
//                 );
//                 scene.stroke(
//                     &vello::kurbo::Rect::new(50.0, 50.0, 200.0, 150.0),
//                     &Stroke::new(5.0),
//                     &Brush::Solid(Color::BLUE),
//                 );

//                 // Draw a green circle
//                 scene.push_circle(
//                     vello::kurbo::Circle::new((350.0, 100.0), 50.0),
//                     Fill::NonZero,
//                     &Brush::Solid(Color::GREEN),
//                 );

//                 let scene = scene.build();

//                 // Sets RenderParams including width, height, base color, and
// antialiasing.                 let render_params = vello::RenderParams {
//                     base_color: Color::WHITE,
//                     width,
//                     height,
//                     antialiasing_method: vello::AaConfig::Msaa8,
//                 };

//                 // Sets DeviceParams for performance tuning.
//                 let device_params = vello::DeviceParams {
//                     power_preference: wgpu::PowerPreference::HighPerformance,
//                     ..Default::default()
//                 };

//                 pollster::block_on(renderer.render_to_window(
//                     &window,
//                     &scene,
//                     &render_params,
//                     &device_params,
//                 ))
//                 .unwrap();
//             }
//             Event::MainEventsCleared => window.request_redraw(), // See the
// RedrawRequested event above.             _ => (),
//         }
//     });
// }
// // ANCHOR_END: example

pub fn main() {}
// // [P1](https://github.com/john-cd/rust_howto/issues/791)
