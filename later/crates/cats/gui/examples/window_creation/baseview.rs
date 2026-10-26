// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Create a simple `baseview` window and respond to basic window events.
//!
//! The window stays open until the close button is requested.
//!
//! `baseview` is a specialized, low-level window creation library targetting
//! windows to be embedded in other applications (e.g. audio plugin UIs).
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
    fn draw(&self) -> Result<(), HandlerError> {
        // No drawing is performed in this example; just keep the window
        // responsive.
        Ok(())
    }

    fn resized(&self, new_size: WindowSize) -> Result<(), HandlerError> {
        println!(
            "Window resized to: {}x{}",
            new_size.physical.width, new_size.physical.height
        );
        Ok(())
    }

    fn on_event(&self, event: Event) -> EventStatus {
        println!("Window event: {event:?}");
        EventStatus::Captured
    }
}

/// Opens a window.
fn main() -> Result<(), baseview::Error> {
    let settings = WindowSettings::new()
        .with_title("Baseview Example")
        .with_size(LogicalSize::new(640.0, 480.0));
    let window = Window::create(settings, |_context: WindowContext| {
        Ok(BaseviewExample)
    })?;
    window.run_until_closed()
}
// ANCHOR_END: example

pub fn run() -> Result<(), baseview::Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_main() -> Result<(), baseview::Error> {
        main()
    }
}
