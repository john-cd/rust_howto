// ANCHOR: example
use baseview::{Event, EventStatus, Size, Window, WindowHandler, WindowOpenOptions, WindowScalePolicy};

// Baseview is a low-level windowing system for audio plugin UIs.

struct MyHandler;

impl WindowHandler for MyHandler {
    fn on_frame(&mut self, _window: &mut Window) {
        // Handle frame updates
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) -> EventStatus {
        match event {
            Event::Mouse(_) => println!("Mouse event"),
            Event::Keyboard(_) => println!("Keyboard event"),
            Event::Window(baseview::WindowEvent::WillClose) => {
                println!("Window will close");
            }
            _ => {}
        }
        EventStatus::Captured
    }
}

pub fn main() -> anyhow::Result<()> {
    let options = WindowOpenOptions {
        title: "Baseview Example".into(),
        size: Size::new(800.0, 600.0),
        scale: WindowScalePolicy::SystemScaleFactor,
    };

    // This will block the current thread until the window is closed.
    Window::open_blocking(options, |_window| MyHandler);

    Ok(())
}
// ANCHOR_END: example
