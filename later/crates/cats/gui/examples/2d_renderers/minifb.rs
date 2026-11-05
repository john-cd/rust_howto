//! TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A minimal native graphics example using `minifb`.
//!
//! `minifb` opens a native window and gives you a plain `u32` pixel buffer to
//! present, with no GPU API or scene graph in between. That makes it a good fit
//! for small software renderers, emulators and demos, and for seeing what
//! the higher-level renderers do underneath.
//!
//! This example renders a simple animated pixel gradient by writing directly to
//! the pixel buffer.

use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use minifb::Key;
use minifb::Window;
use minifb::WindowOptions;

/// The width of the window in pixels.
const WIDTH: usize = 320;

/// The height of the window in pixels.
const HEIGHT: usize = 240;

/// Target frame duration capping the loop at roughly 60 frames per second.
const TARGET_FRAME_DURATION: Duration = Duration::from_millis(16);

/// Returns the `0xAARRGGBB` color of the pixel at (`x`, `y`) for the
/// animation `offset`: each channel is a position plus the offset, wrapped to
/// one byte.
fn pixel(x: usize, y: usize, offset: u32) -> u32 {
    let r = (x as u32).wrapping_add(offset) & 0xFF;
    let g = (y as u32).wrapping_add(offset) & 0xFF;
    let b = ((x + y) as u32).wrapping_add(offset) & 0xFF;
    (0xFF << 24) | (r << 16) | (g << 8) | b
}

/// Fills the buffer (`WIDTH` x `HEIGHT` pixels) with the gradient.
fn fill_gradient(buffer: &mut [u32], offset: u32) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = pixel(x, y, offset);
        }
    }
}

/// Creates and configures the native window for the example.
///
/// # Errors
///
/// Returns an error if the window cannot be created.
fn create_window() -> Result<Window> {
    Ok(Window::new(
        "minifb native graphics example",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?)
}

/// Renders a single frame: fills the pixel buffer, blits it to the window,
/// and advances the animation offset.
///
/// # Errors
///
/// Returns an error if blitting the buffer to the window fails.
fn render_frame(window: &mut Window, buffer: &mut [u32], offset: &mut u32) -> Result<()> {
    fill_gradient(buffer, *offset);

    // Copy the finished frame to the window; this also processes input
    // events.
    window.update_with_buffer(buffer, WIDTH, HEIGHT)?;

    // Shifting every channel by one each frame makes the gradient scroll.
    *offset = offset.wrapping_add(1);

    Ok(())
}

/// Throttles execution if the frame completed faster than the target duration.
fn throttle_frame_rate(last_frame: Instant, target_duration: Duration) -> Instant {
    let elapsed = last_frame.elapsed();
    if elapsed < target_duration {
        std::thread::sleep(target_duration.saturating_sub(elapsed));
    }
    Instant::now()
}

/// Opens a window and draws into a pixel buffer until the window is closed.
fn run_window() -> Result<()> {
    // The window owns the OS surface; the buffer is our own CPU-side
    // framebuffer with one `0xAARRGGBB` value per pixel.
    let mut window = create_window()?;
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut offset = 0u32;
    let mut last_frame = Instant::now();

    // Redraw every frame until the user closes the window or presses Escape.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        render_frame(&mut window, &mut buffer, &mut offset)?;
        last_frame = throttle_frame_rate(last_frame, TARGET_FRAME_DURATION);
    }

    Ok(())
}

/// Starts the standalone minifb example target.
fn main() -> Result<()> {
    run_window()
}

// ANCHOR_END: example

/// Opens the minifb window.
pub fn run() -> Result<()> {
    run_window()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the color of a few pixels.
    #[test]
    fn pixel_colors() {
        assert_eq!(pixel(0, 0, 0), 0xFF00_0000);
        assert_eq!(pixel(1, 2, 0), 0xFF01_0203);
        // Channels wrap around at 256.
        assert_eq!(pixel(255, 0, 1), 0xFF00_0100);
    }

    /// Checks that the gradient fills the whole buffer and animates.
    #[test]
    fn gradient_fills_and_animates() {
        let mut buffer = vec![0_u32; WIDTH * HEIGHT];
        fill_gradient(&mut buffer, 0);
        assert!(buffer.iter().all(|p| p >> 24 == 0xFF));
        assert_eq!(buffer[WIDTH + 1], pixel(1, 1, 0));
        let first = buffer.clone();
        fill_gradient(&mut buffer, 1);
        assert_ne!(first, buffer);
    }

    /// Checks that throttling returns an instant after the elapsed time.
    #[test]
    fn frame_rate_throttling_advances_instant() {
        let start = Instant::now();
        let target = Duration::from_millis(5);
        let next = throttle_frame_rate(start, target);
        assert!(next >= start);
    }

    /// Runs the minifb example through its entry point.
    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_minifb_example_entry_point() -> Result<()> {
        run_window()
    }
}
