// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// Trait that handles the application events:
use winit::application::ApplicationHandler;
// An event from a `Window`: Resized, Moved, etc.
use winit::event::WindowEvent;
// Allow you to create new windows while Winit executes your callback:
use winit::event_loop::ActiveEventLoop;
// Indicates the desired behavior of the event loop after
// `Event::AboutToWait` is emitted.
use winit::event_loop::ControlFlow;
// Provides a way to retrieve events from the system and from the windows
// that were registered to the events loop:
use winit::event_loop::EventLoop;
use winit::window::Window;
// Identifier, unique for each window:
use winit::window::WindowId;

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    // The application has been resumed.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes =
            Window::default_attributes().with_title("Test window");
        self.window =
            Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render
                // continuously to render in this event rather
                // than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the
                // OS.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you
                // need to redraw in applications which do not always
                // need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    // `ControlFlow::Poll` continuously runs the event loop, even if the OS
    // hasn't dispatched any events. This is ideal for games and
    // similar applications:
    // event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available
    // to process. This is ideal for non-game applications that only
    // update in response to user input, and uses significantly less
    // power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    // Run the application with the event loop on the calling thread.
    event_loop.run_app(&mut app)?;
    Ok(())
}

// // [WIP review](https://github.com/john-cd/rust_howto/issues/794)
// https://github.com/rust-windowing/winit/tree/master/examples
