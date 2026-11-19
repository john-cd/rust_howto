// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Creating a window with `winit`.
//!
//! A window is how a program receives input and shows pixels. `winit` creates
//! windows on all platforms and delivers events, such as key presses and
//! resizes, through an event loop that runs until the program exits.

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
    /// Closes the native smoke run after its first redraw event.
    smoke: bool,
    /// Number of redraw events handled by this instance.
    redraws: usize,
}

impl ApplicationHandler for App {
    /// Creates the window once when the event loop resumes; exits if creation fails.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attributes = Window::default_attributes().with_title("Winit window");
            match event_loop.create_window(attributes) {
                Ok(window) => self.window = Some(window),
                Err(error) => {
                    eprintln!("Could not create the window: {error}");
                    event_loop.exit();
                }
            }
        }
    }

    /// Exits on a close request and reports redraw requests without drawing pixels.
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if should_exit(&event) {
            event_loop.exit();
        } else if matches!(event, WindowEvent::RedrawRequested) {
            self.redraws += 1;
            println!("Redraw requested");
            if self.smoke {
                event_loop.exit();
            }
        }
    }
}

/// Reports whether a window event requests application shutdown.
fn should_exit(event: &WindowEvent) -> bool {
    matches!(event, WindowEvent::CloseRequested)
}

/// Creates the event loop and runs the window, optionally until its first redraw.
///
/// # Errors
///
/// Returns an error if window/event-loop initialization fails or a smoke run
/// finishes without a redraw.
fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let smoke = std::env::var_os("RUST_HOWTO_SMOKE_TEST").is_some();
    let mut app = App {
        smoke,
        ..Default::default()
    };
    event_loop.run_app(&mut app)?;
    anyhow::ensure!(app.window.is_some(), "window creation did not succeed");
    anyhow::ensure!(
        !smoke || app.redraws > 0,
        "native smoke run received no redraw"
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the window example.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Opens a native window for manual testing.
#[cfg(test)]
mod tests {
    use super::*;

    /// Opens a native window for manual testing.
    #[test]
    #[ignore = "opens a native window"]
    fn test_winit_example_entry_point() {
        main().expect("the Winit example should run successfully");
    }

    /// Closes requested asks the application to exit.
    #[test]
    fn close_requested_asks_the_application_to_exit() {
        assert!(should_exit(&WindowEvent::CloseRequested));
        assert!(!should_exit(&WindowEvent::RedrawRequested));
    }
}
