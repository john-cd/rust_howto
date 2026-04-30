// ANCHOR: example
//! Create a simple `baseview` window and respond to basic window events.
//!
//! The window stays open until the close button is requested.
//!
//! `baseview` is a specialized, low-level window creation library targetting windows
//! to be embedded in other applications (e.g. audio plugin UIs).
//!
//! `baseview` abstracts the platform-specific windowing APIs (`winapi`, `cocoa`,
//! `xcb`) into a platform-independent API, but otherwise gets out of your way so
//! we can write plugin UIs.
//!
//! Requirements (Linux):
//! ```sh
//! sudo apt-get install libx11-dev libxcb1-dev libx11-xcb-dev libgl1-mesa-dev
//! ```
use anyhow::Result;
use baseview::{ControlFlow, Size, Window, WindowDelegate, WindowEvent, WindowOpenOptions, WindowScalePolicy};

struct BaseviewExample;

impl WindowDelegate for BaseviewExample {
    fn on_frame(&mut self, _window: &mut Window<'_>) {
        // No drawing is performed in this example; just keep the window responsive.
    }

    fn on_window_event(&mut self, event: WindowEvent<'_>, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(size) => {
                println!("Window resized to: {}x{}", size.width, size.height);
            }
            WindowEvent::Moved(position) => {
                println!("Window moved to: {}x{}", position.x, position.y);
            }
            _ => (),
        }
    }
}

pub fn main() -> Result<()> {
    let size = Size::new(640.0, 480.0);
    let mut open_options = WindowOpenOptions::new("Baseview Example".to_string(), size);
    open_options.scale = WindowScalePolicy::ScaleFactor(1.0);

    Window::open_parentless(open_options, Box::new(BaseviewExample));
    Ok(())
}
// ANCHOR_END: example
