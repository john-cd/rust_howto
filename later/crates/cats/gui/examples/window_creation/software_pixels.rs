//! Draws software pixels in a native Winit window.

// ANCHOR: example
use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::Context;
use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

/// Owns the window surface and propagates event-handler errors to the caller.
#[derive(Default)]
struct App {
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    error: Option<Box<dyn std::error::Error>>,
}

/// Presents a solid blue frame, skipping zero-sized minimized windows.
fn draw(
    surface: &mut Surface<Rc<Window>, Rc<Window>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let size = surface.window().inner_size();
    let (Some(width), Some(height)) =
        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
    else {
        return Ok(());
    };
    surface.resize(width, height)?;
    let mut buffer = surface.buffer_mut()?;
    // Softbuffer pixels use 0x00RRGGBB; the high byte is unused.
    buffer.fill(0x0020_60C0);
    buffer.present()?;
    Ok(())
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.surface.is_some() {
            return;
        }
        let result = (|| -> Result<_, Box<dyn std::error::Error>> {
            let window = Rc::new(event_loop.create_window(
                Window::default_attributes().with_title("Software pixels"),
            )?);
            let context = Context::new(window.clone())?;
            let surface = Surface::new(&context, window)?;
            surface.window().request_redraw();
            Ok(surface)
        })();
        match result {
            Ok(surface) => self.surface = Some(surface),
            Err(error) => {
                self.error = Some(error);
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => {
                if let Some(surface) = &self.surface {
                    surface.window().request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(surface) = &mut self.surface
                    && let Err(error) = draw(surface)
                {
                    self.error = Some(error);
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}

/// Runs the event loop until the window closes or drawing fails.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    match app.error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}
// ANCHOR_END: example

/// Opens the software-rendered window.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()
}

#[cfg(test)]
mod tests {
    /// Exercises native presentation in an interactive desktop session.
    #[test]
    #[ignore = "opens a native window; close it to finish"]
    fn presents_software_pixels() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
