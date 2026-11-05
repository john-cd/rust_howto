// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Creates a row-major RGBA pixel buffer with opaque and translucent pixels.

use image::Rgba;
use image::RgbaImage;

/// Creates a two-pixel RGBA image for the example.
fn example_buffer() -> RgbaImage {
    let mut buffer = RgbaImage::new(2, 1);
    buffer.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    buffer.put_pixel(1, 0, Rgba([0, 0, 255, 128]));
    buffer
}

/// Prints the row-major byte storage of the example image.
fn print_example() {
    let buffer = example_buffer();
    println!(
        "dimensions={:?}, bytes={:?}",
        buffer.dimensions(),
        buffer.as_raw()
    );
}

/// Runs the pixel-buffer example from a standalone entry point.
fn main() {
    print_example();
}
// ANCHOR_END: example

/// Runs the pixel-buffer example.
pub fn run() {
    print_example();
}

#[cfg(test)]
mod tests {
    use super::example_buffer;

    /// Verifies that the raw buffer stores RGBA pixels in row-major order.
    #[test]
    fn raw_bytes_preserve_pixel_order_and_alpha() {
        assert_eq!(
            example_buffer().as_raw().as_slice(),
            &[255, 0, 0, 255, 0, 0, 255, 128]
        );
    }

    /// Exercises the example entry point.
    #[test]
    fn test_main_entry_point() {
        super::main();
    }
}
