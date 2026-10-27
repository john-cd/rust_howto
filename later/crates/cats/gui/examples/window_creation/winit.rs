// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Creating a window with `winit`.

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

/// Application state for the example window.
#[derive(Default)]
struct App {
    /// The window created after the event loop resumes.
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attributes =
                Window::default_attributes().with_title("Winit window");
            match event_loop.create_window(attributes) {
                Ok(window) => self.window = Some(window),
                Err(error) => {
                    eprintln!("Could not create the window: {error}");
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if should_exit(&event) {
            event_loop.exit();
        } else if matches!(event, WindowEvent::RedrawRequested) {
            println!("Redraw requested");
        }
    }
}

/// Reports whether a window event requests application shutdown.
fn should_exit(event: &WindowEvent) -> bool {
    matches!(event, WindowEvent::CloseRequested)
}

/// Creates the event loop and runs the application.
fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut App::default())?;
    Ok(())
}
// ANCHOR_END: example

/// Runs the window example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Opens a native window for manual testing.
    #[test]
    #[ignore = "opens a native window"]
    fn test_winit_example_entry_point() {
        main().expect("the Winit example should run successfully");
    }

    /// Checks that only a close request asks the application to exit.
    #[test]
    fn close_requested_asks_the_application_to_exit() {
        assert!(should_exit(&WindowEvent::CloseRequested));
        assert!(!should_exit(&WindowEvent::RedrawRequested));
    }
}
