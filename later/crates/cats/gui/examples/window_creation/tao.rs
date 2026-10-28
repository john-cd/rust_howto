// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Creates and manages a desktop window with Tao.

use std::error::Error;

use tao::event::Event;
use tao::event::WindowEvent;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;
use tao::window::WindowBuilder;

/// Returns whether a window event requests that the application exit.
fn should_close(event: &WindowEvent<'_>) -> bool {
    matches!(event, WindowEvent::CloseRequested)
}

/// Creates a window and runs its event loop.
///
/// # Errors
///
/// Returns an error if the operating system cannot create the window.
fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Tao Example")
        .build(&event_loop)?;

    // Tao is the Tauri project's winit fork, used with desktop app
    // integrations such as system menus and tray support.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } if should_close(&event) => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                println!("Resized to: {physical_size:?}");
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
// ANCHOR_END: example

/// Runs the Tao window example.
///
/// # Errors
///
/// Returns an error if the operating system cannot create the window.
pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that close requests exit while unrelated window events do not.
    #[test]
    fn close_request_is_the_only_event_that_requests_exit() {
        assert!(should_close(&WindowEvent::CloseRequested));
        assert!(!should_close(&WindowEvent::Focused(false)));
    }

    /// Runs the Tao entry point in a native window.
    #[test]
    #[ignore = "opens a window"]
    fn test_tao_example_entry_point() -> Result<(), Box<dyn Error>> {
        run()
    }
}
