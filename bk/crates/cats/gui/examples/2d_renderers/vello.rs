// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use the `vello` crate for 2D rendering
// //! with GPU acceleration. It sets up a window using `winit` and draws
// //! a simple scene containing a rectangle and a circle using Vello's API.
// //!
// //! ## Key Concepts:
// //! - **Renderer:** The core Vello object responsible for rendering scenes to
// //!   a surface (like a window).
// //! - **SceneBuilder:** Used to construct a scene by adding shapes, paths,
// //!   and other drawing commands.
// //! - **Peniko:** Vello uses the `peniko` crate for defining colors, brushes,
// //!   and fill/stroke styles.
// //! - **Kurbo:** Vello uses the `kurbo` crate for defining geometric shapes
// //!   (like Rect, Circle).
// //! - **Winit:** Used for creating the window and handling user input and
// //!   window events.
// //! - **Pollster:** Used to block on asynchronous operations during
// //!   initialization and rendering, simplifying the example for synchronous
// //!   contexts.

// // Import necessary items from the vello crate.
// use vello::kurbo; // For defining geometric shapes.
// use vello::peniko::{Brush, Color, Fill, Stroke}; // For defining drawing
// styles. use vello::{AaConfig, DeviceParams, RenderParams, Renderer,
// SceneBuilder}; // Core Vello types. // Use winit for window creation and
// event handling. use winit::dpi::PhysicalSize;
// use winit::event::Event;
// use winit::event::WindowEvent;
// use winit::event_loop::ControlFlow;
// use winit::event_loop::EventLoop;
// use winit::window::WindowBuilder;

// /// Entry point for the Vello example application.
// ///
// /// Initializes winit, creates a window, sets up the Vello renderer,
// /// and runs the event loop to draw a scene.
// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // --- 1. Winit Setup ---
//     // Create an event loop for handling window events.
//     let event_loop = EventLoop::new();
//     // Build the application window.
//     let window = WindowBuilder::new()
//         .with_title("Vello Example")
//         .with_inner_size(PhysicalSize::new(600, 400))
//        // Set initial window size.
//        .build(&event_loop)?;

//     // --- 2. Vello Renderer Initialization ---
//     // Create a Vello Renderer instance associated with the window.
//     // `Renderer::new` is async, so `pollster::block_on` is used here to wait
//     // for it to complete in this synchronous `main` function context.
//     // This might block the main thread briefly during startup.
//     let mut renderer = pollster::block_on(Renderer::new(&window))?;

//     // --- 3. Winit Event Loop ---
//     event_loop.run(move |event, _, control_flow| {
//         // Set the default control flow to Poll. This means the loop will run
//         // continuously, checking for events and redrawing as needed.
//         *control_flow = ControlFlow::Poll;

//         match event {
//             // --- Event Handling ---
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 // User clicked the close button.
//                 ..
//             } => {
//                 // Set control flow to Exit to break out of the event loop
//                 // and close the application.
//                 *control_flow = ControlFlow::Exit;
//             }
//             Event::WindowEvent {
//                 event: WindowEvent::Resized(physical_size),
//                 // Window was resized.
//                 ..
//             } => {
//                 // Optional: Handle window resize events if needed.
//                 // Vello's `render_to_window` handles surface reconfiguration
//                 // automatically based on the render params width/height.
//                 println!("Window resized to: {physical_size:?}");
//                 // Request a redraw to render the scene with the new size.
//                 window.request_redraw();
//             }

//             // --- Redrawing ---
//             Event::RedrawRequested(_) => {
//                 // This event is triggered when the window needs to be
//                 // redrawn, either by the OS or by `window.request_redraw()`.

//                 // Get the current inner dimensions of the window.
//                 let width = window.inner_size().width;
//                 let height = window.inner_size().height;

//                 // --- 4. Scene Building ---
//                 // Create a SceneBuilder to define what to draw.
//                 let mut scene_builder = SceneBuilder::new();

//                 // Define a red rectangle with a blue outline.
//                 let rect = kurbo::Rect::new(50.0, 50.0, 250.0, 150.0);
//                 // Fill the rectangle with solid red color.
//                 scene_builder.fill(
//                     Fill::NonZero, // Fill rule (NonZero is common).
//                     kurbo::Affine::IDENTITY, // No transformation.
//                     &Brush::Solid(Color::RED), // Solid red brush.
//                     None, // No alpha mask.
//                     &rect, // The shape to fill.
//                 );
//                 // Stroke the rectangle's outline with a 5px blue line.
//                 scene_builder.stroke(
//                     &Stroke::new(5.0), // 5px stroke width.
//                     kurbo::Affine::IDENTITY, // No transformation.
//                     &Brush::Solid(Color::BLUE), // Solid blue brush.
//                     None, // No alpha mask.
//                     &rect, // The shape to stroke.
//                 );

//                 // Define and draw a green circle.
//                 let circle = kurbo::Circle::new((350.0, 100.0), 50.0);
// // Center (350, 100), radius 50.
//                  scene_builder.fill(
//                     Fill::NonZero,
//                     kurbo::Affine::IDENTITY,
//                     &Brush::Solid(Color::GREEN), // Solid green brush.
//                     None,
//                     &circle, // The shape to fill.
//                 );

//                 // Finalize the scene.
//                 let scene = scene_builder.build();

//                 // --- 5. Rendering ---
//                 // Define parameters for rendering.
//                 let render_params = RenderParams {
//                   base_color: Color::WHITE,
//                   // Background color when clearing.
//                   width,
//                   // Render width matches window width.
//                   height,
//                   // Render height matches window height.
//                   antialiasing_method: AaConfig::Msaa8,
//                   // Use 8x MSAA for smoother edges.
//                 };

//                 // Define device parameters (optional, defaults are often
//                 // fine).
//                 let device_params = DeviceParams {
//                     power_preference: wgpu::PowerPreference::HighPerformance,
//                  // Request high performance GPU if available.
//                  ..Default::default()                 };

//                 // Render the scene to the window's surface.
//                 // `render_to_window` is async, so `pollster::block_on` is
//                 // used. In a more complex application, you might integrate
//                 // this with an async runtime.
//                 match pollster::block_on(renderer.render_to_window(
//                     &window,
//                     &scene,
//                     &render_params,
//                     &device_params,
//                 )) {
//                     Ok(_) => {} // Render successful.
//                     Err(e) => {
//                         // Handle rendering errors (e.g., surface lost).
//                         eprintln!("Error rendering to window: {e}");
//                         // You might want to recreate the renderer or exit
//                         // here depending on the error.
//                         *control_flow = ControlFlow::Exit;
//                     }
//                 }
//             }

//             // --- Continuous Rendering ---
//             Event::MainEventsCleared => {
//                 // This event is fired after all window events have been
//                 // processed. Requesting a redraw here ensures the
// // application continuously renders frames, which is useful
// // for animation or if the scene changes over time. For
// // static scenes, we might only request redraws when
// // necessary (e.g., after resize).
// window.request_redraw();
//             }
//             _ => (), // Ignore other events.
//         }
//     });

// // Note: The event loop runs indefinitely until `ControlFlow::Exit` is
// // set. `event_loop.run` never returns in the normal case, so code here
// // is unreachable. However, returning Ok(()) satisfies the function
// // signature.
// Ok(())
// }

pub fn main() {}
