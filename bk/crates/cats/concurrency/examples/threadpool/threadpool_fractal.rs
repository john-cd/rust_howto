#![allow(dead_code)]
// ANCHOR: example
use std::fs;
use std::sync::mpsc::channel;

use anyhow::Result;
use image::ImageBuffer;
use image::Rgb;
use num::complex::Complex;
use threadpool::ThreadPool;

/// Converts a wavelength (in nanometers) to an RGB color.
///
/// This function maps a given wavelength to a corresponding RGB color,
/// simulating the colors of the visible light spectrum. It uses a piecewise
/// linear function to determine the red, green, and blue components.
fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;
    let (r, g, b) = match wavelength {
        380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
        440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
        490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
        510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
        580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
        645..=780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };
    let factor = match wavelength {
        380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
        701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
        _ => 1.0,
    };
    let (r, g, b) = (
        normalize(r, factor),
        normalize(g, factor),
        normalize(b, factor),
    );
    Rgb([r, g, b])
}

/// Calculates the number of iterations required for a point to escape the Julia
/// set.
///
/// This function determines whether a given point (x, y) belongs to the Julia
/// set for a given complex number `c`. It does this by iteratively applying the
/// function z = z^2 + c, starting with z as the complex representation of the
/// point (x, y). If the magnitude of z exceeds 2.0 within `max_iter`
/// iterations, the point is considered to have escaped the set. The number of
/// iterations required for escape is returned.
///
/// # Arguments
/// * `c` - The complex number defining the Julia set.
/// * `x`, `y` - The coordinates of the point to test.
/// * `width`, `height` - The dimensions of the image.
fn julia(
    c: Complex<f32>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    max_iter: u32,
) -> u32 {
    let width = width as f32;
    let height = height as f32;
    let mut z = Complex {
        // Scale and translate the point to image coordinates:
        re: 3.0 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };
    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

/// Normalizes a color component value.
///
/// This function takes a color component (r, g, or b) and a factor, and
/// normalizes the color component to an 8-bit value (0-255). It applies a
/// power function to adjust the intensity and then scales it to the range
/// of 0-255.
fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}

fn main() -> Result<()> {
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);
    let iterations = 300;

    let c = Complex::new(-0.8, 0.156);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                let i = julia(c, x, y, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("Could not send data!");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    img.save("temp/output.png")?;
    println!("Image saved!");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review; threadpool_fractal.rs is noplayground - linking with cc](https://github.com/john-cd/rust_howto/issues/268)
