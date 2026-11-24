// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Create a simple `baseview` window and respond to basic window events.
//!
//! The window stays open until the user requests that it close.
//!
//! `baseview` can create windows embedded in a host application, such as an
//! audio plug-in editor.
//!
//! `baseview` abstracts the platform-specific windowing APIs (`winapi`,
//! `cocoa`, `xcb`) into a platform-independent API, but otherwise gets out of
//! your way so we can write plugin UIs.
//!
//! Requirements (Linux):
//! ```sh
//! sudo apt-get install libx11-dev libxcb1-dev libx11-xcb-dev libgl1-mesa-dev
//! ```

use baseview::Event;
use baseview::EventStatus;
use baseview::HandlerError;
use baseview::Window;
use baseview::WindowContext;
use baseview::WindowHandler;
use baseview::WindowSettings;
use baseview::WindowSize;
use baseview::dpi::LogicalSize;

/// A window delegate that handles the window events.
struct BaseviewExample;

impl WindowHandler for BaseviewExample {
    /// Completes a draw callback without rendering; this example only demonstrates events.
    fn draw(&self) -> Result<(), HandlerError> {
        // No drawing is performed in this example; just keep the window
        // responsive.
        Ok(())
    }

    /// Prints the new physical pixel dimensions reported by the window backend.
    fn resized(&self, new_size: WindowSize) -> Result<(), HandlerError> {
        println!(
            "Window resized to: {}x{}",
            new_size.physical.width, new_size.physical.height
        );
        Ok(())
    }

    /// Logs each native event and marks it captured by this handler.
    fn on_event(&self, event: Event) -> EventStatus {
        println!("Window event: {event:?}");
        EventStatus::Captured
    }
}

/// The title of the window.
const TITLE: &str = "Baseview Example";

/// The initial size of the window in logical pixels.
const SIZE: (f64, f64) = (640.0, 480.0);

/// Opens a window.
fn main() -> Result<(), baseview::Error> {
    let settings = WindowSettings::new()
        .with_title(TITLE)
        .with_size(LogicalSize::new(SIZE.0, SIZE.1));
    let window = Window::create(settings, |_context: WindowContext| Ok(BaseviewExample))?;
    window.run_until_closed()
}
// ANCHOR_END: example

/// Opens a window.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> Result<(), baseview::Error> {
    main()
}

/// Checks the window defaults.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the window defaults.
    #[test]
    fn window_defaults() {
        assert_eq!(TITLE, "Baseview Example");
        assert!(SIZE.0 > 0.0 && SIZE.1 > 0.0);
    }

    /// Checks that the handler accepts the draw call without a window.
    #[test]
    fn draw_succeeds() {
        assert!(BaseviewExample.draw().is_ok());
    }

    /// Runs the baseview example through its entry point.
    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_baseview_example_entry_point() -> Result<(), baseview::Error> {
        main()
    }
}
