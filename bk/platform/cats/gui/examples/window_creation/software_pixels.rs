// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Draws software pixels in a native Winit window.
//!
//! Software rendering means the CPU computes every pixel into a buffer that is
//! then shown in the window, with no GPU programming. It is slower, but simple,
//! portable and enough for tools and retro-style games.

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
    /// Optional surface used by app; `None` means the value is not configured or unavailable.
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,

    /// Optional error used by app; `None` means the value is not configured or unavailable.
    error: Option<Box<dyn std::error::Error>>,
    /// Stops the smoke check after a successful nonzero frame presentation.
    smoke: bool,
    /// Number of frames successfully submitted to the native surface.
    presented: usize,
}

/// The solid color of the frame, in the 0x00RRGGBB format.
const BLUE: u32 = 0x0020_60C0;

/// Returns the size as non-zero numbers, or `None` for a zero-sized
///
/// (minimized) window.
///
/// # Returns
///
/// `Some` when the requested value is available; otherwise returns `None`.
fn non_zero_size(width: u32, height: u32) -> Option<(NonZeroU32, NonZeroU32)> {
    Some((NonZeroU32::new(width)?, NonZeroU32::new(height)?))
}

/// Presents a solid blue frame, skipping zero-sized minimized windows.
///
/// Returns `true` only after a nonzero buffer is successfully presented.
///
/// # Errors
///
/// Returns an error if resizing, mapping, or presenting the native surface fails.
fn draw(surface: &mut Surface<Rc<Window>, Rc<Window>>) -> Result<bool, Box<dyn std::error::Error>> {
    let size = surface.window().inner_size();
    let Some((width, height)) = non_zero_size(size.width, size.height) else {
        return Ok(false);
    };
    surface.resize(width, height)?;
    let mut buffer = surface.buffer_mut()?;
    // Softbuffer pixels use 0x00RRGGBB; the high byte is unused.
    buffer.fill(BLUE);
    buffer.present()?;
    Ok(true)
}

impl ApplicationHandler for App {
    /// Reports whether the suspended operation has resumed.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.surface.is_some() {
            return;
        }
        let result = (|| -> Result<_, Box<dyn std::error::Error>> {
            let window = Rc::new(
                event_loop
                    .create_window(Window::default_attributes().with_title("Software pixels"))?,
            );
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

    /// Schedules repaint after resizing and exits while preserving any drawing error.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) => {
                if let Some(surface) = &self.surface {
                    surface.window().request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(surface) = &mut self.surface {
                    match draw(surface) {
                        Ok(true) => {
                            self.presented += 1;
                            if self.smoke {
                                event_loop.exit();
                            }
                        }
                        Ok(false) => {}
                        Err(error) => {
                            self.error = Some(error);
                            event_loop.exit();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Runs the event loop until the window closes or drawing fails.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let smoke = std::env::var_os("RUST_HOWTO_SMOKE_TEST").is_some();
    let mut app = App {
        smoke,
        ..Default::default()
    };
    event_loop.run_app(&mut app)?;
    if smoke && app.error.is_none() && app.presented == 0 {
        return Err(std::io::Error::other("no native frame was presented").into());
    }
    println!("Presented {} softbuffer frame(s).", app.presented);
    match app.error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}
// ANCHOR_END: example

/// Opens the software-rendered window.
/// Draws software pixels in a native Winit window.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main()
}

/// Checks that a zero-sized window is skipped.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that a zero-sized window is skipped.
    #[test]
    fn zero_sized_windows_are_skipped() {
        assert_eq!(non_zero_size(0, 10), None);
        assert_eq!(non_zero_size(10, 0), None);
        let (width, height) = non_zero_size(640, 480).expect("non-zero size");
        assert_eq!((width.get(), height.get()), (640, 480));
    }

    /// Checks that the unused high byte of the color is zero.
    #[test]
    fn blue_has_no_alpha_byte() {
        assert_eq!(BLUE >> 24, 0);
    }

    /// Exercises native presentation in an interactive desktop session.
    #[test]
    #[ignore = "opens a native window; close it to finish"]
    fn presents_software_pixels() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
