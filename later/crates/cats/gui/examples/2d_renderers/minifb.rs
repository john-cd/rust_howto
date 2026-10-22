// TODO Reviewed ??-??-2026
// ANCHOR: example
//! A minimal native graphics example using `minifb`.
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

/// Opens a window and draws into a pixel buffer until the window is closed.
fn main() -> Result<()> {
    let mut window = Window::new(
        "minifb native graphics example",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut offset = 0u32;
    let mut last_frame = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let r = ((x as u32).wrapping_add(offset)) & 0xFF;
                let g = ((y as u32).wrapping_add(offset)) & 0xFF;
                let b = (((x + y) as u32).wrapping_add(offset)) & 0xFF;
                buffer[y * WIDTH + x] = (0xFF << 24) | (r << 16) | (g << 8) | b;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;

        offset = offset.wrapping_add(1);

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

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_main() -> Result<()> {
        main()
    }
}
// TODO review
