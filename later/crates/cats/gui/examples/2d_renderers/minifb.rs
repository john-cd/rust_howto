// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A minimal native graphics example using `minifb`.
//! TODO explain minifb usage and its purpose in this example.
//!
//! This example opens a native window and renders a simple animated pixel
//! gradient by writing directly to a pixel buffer.

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

/// Opens a window and draws into a pixel buffer until the window is closed.
fn run_window() -> Result<()> {
    // TODO explain why/how we create the window and buffer here
    let mut window = Window::new(
        "minifb native graphics example",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut offset = 0u32;
    let mut last_frame = Instant::now();

    // TODO explain the main rendering loop and its purpose
    while window.is_open() && !window.is_key_down(Key::Escape) {
        fill_gradient(&mut buffer, offset);

        // TODO explain why we update the window with the buffer here
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;

        offset = offset.wrapping_add(1);

        // TODO explain why we increment the offset here to animate the gradient
        let elapsed = last_frame.elapsed();
        if elapsed < Duration::from_millis(16) {
            std::thread::sleep(
                Duration::from_millis(16).saturating_sub(elapsed),
            );
        }
        last_frame = Instant::now();
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

    /// Runs the minifb example through its entry point.
    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_minifb_example_entry_point() -> Result<()> {
        run_window()
    }
}
