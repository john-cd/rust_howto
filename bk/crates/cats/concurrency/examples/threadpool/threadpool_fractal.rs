//! Renders and saves a Julia set with Rayon and the image crate.

#![allow(dead_code)]
// TODO Reviewed ??-??-2026

use std::fs;
use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;
use image::ImageBuffer;
use image::Rgb;
use num::complex::Complex;
use rayon::prelude::*;

// ANCHOR: example
/// Renders a Julia set into an in-memory RGB image.
///
/// # Errors
///
/// Returns an error if the dimensions or iteration count are zero, the output
/// buffer size overflows, or the buffer cannot be represented by ImageBuffer.
fn render_julia(
    width: u32,
    height: u32,
    max_iterations: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    anyhow::ensure!(width > 0, "image width must be greater than zero");
    anyhow::ensure!(height > 0, "image height must be greater than zero");
    anyhow::ensure!(
        max_iterations > 0,
        "iteration limit must be greater than zero"
    );

    let width_usize =
        usize::try_from(width).context("image width does not fit usize")?;
    let height_usize =
        usize::try_from(height).context("image height does not fit usize")?;
    let row_length = width_usize
        .checked_mul(3)
        .context("image row size overflows usize")?;
    let buffer_length = row_length
        .checked_mul(height_usize)
        .context("image buffer size overflows usize")?;
    let mut pixels = vec![0; buffer_length];
    let julia_constant = Complex::new(-0.8, 0.156);

    pixels
        .par_chunks_mut(row_length)
        .enumerate()
        .for_each(|(y, row)| {
            let y = y as u32;
            let (pixels, remainder) = row.as_chunks_mut::<3>();
            debug_assert!(remainder.is_empty());
            for (x, pixel) in pixels.iter_mut().enumerate() {
                let x = x as u32;
                let iterations = julia_escape_iterations(
                    julia_constant,
                    x,
                    y,
                    width,
                    height,
                    max_iterations,
                );
                let wavelength = 380
                    + ((u64::from(iterations) * 400)
                        / u64::from(max_iterations))
                        as u32;
                *pixel = wavelength_to_rgb(wavelength).0;
            }
        });

    ImageBuffer::from_raw(width, height, pixels)
        .context("rendered pixel buffer has an unexpected length")
}

/// Renders and saves a sample Julia set image.
///
/// # Errors
///
/// Returns an error if the image cannot be rendered, the output directory
/// cannot be created, or the PNG file cannot be written.
fn main() -> Result<()> {
    let image = render_julia(640, 480, 300)?;
    fs::create_dir_all("temp")
        .context("could not create the temp output directory")?;
    image
        .save(Path::new("temp/output.png"))
        .context("could not save the Julia set image")?;
    Ok(())
}
// ANCHOR_END: example

/// Calculates how many iterations a point takes to escape the Julia set.
fn julia_escape_iterations(
    constant: Complex<f32>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    max_iterations: u32,
) -> u32 {
    let width_f32 = width as f32;
    let height_f32 = height as f32;
    let mut point = Complex {
        re: 3.0 * (x as f32 - 0.5 * width_f32) / width_f32,
        im: 2.0 * (y as f32 - 0.5 * height_f32) / height_f32,
    };

    for iteration in 0..max_iterations {
        if point.norm_sqr() >= 4.0 {
            return iteration;
        }
        point = point * point + constant;
    }
    max_iterations
}

/// Maps a visible-light wavelength to an RGB color for the fractal palette.
fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;
    let (red, green, blue) = match wavelength {
        380..=439 => ((440.0 - wave) / 60.0, 0.0, 1.0),
        440..=489 => (0.0, (wave - 440.0) / 50.0, 1.0),
        490..=509 => (0.0, 1.0, (510.0 - wave) / 20.0),
        510..=579 => ((wave - 510.0) / 70.0, 1.0, 0.0),
        580..=644 => (1.0, (645.0 - wave) / 65.0, 0.0),
        645..=780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };
    let factor = match wavelength {
        380..=419 => 0.3 + 0.7 * (wave - 380.0) / 40.0,
        701..=780 => 0.3 + 0.7 * (780.0 - wave) / 80.0,
        _ => 1.0,
    };
    Rgb([
        normalize_color_component(red, factor),
        normalize_color_component(green, factor),
        normalize_color_component(blue, factor),
    ])
}

/// Scales one normalized color component into an 8-bit channel.
fn normalize_color_component(component: f32, factor: f32) -> u8 {
    ((component * factor).powf(0.8) * 255.0) as u8
}

/// Runs the fractal example.
pub fn run() -> Result<()> {
    main()
}

/// Unit tests for the in-memory Julia set renderer.
#[cfg(test)]
mod tests {
    use image::GenericImageView as _;

    use super::*;

    /// Verifies that rendering preserves the image dimensions in memory and in
    /// a temporary PNG.
    #[test]
    fn threadpool_fractal_renders_requested_dimensions_in_memory_and_in_temporary_png()
    -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let output_path = temp_dir.path().join("fractal.png");
        let image = render_julia(64, 64, 128)?;

        assert_eq!(image.dimensions(), (64, 64));
        assert_eq!(image.as_raw().len(), 64 * 64 * 3);

        image.save(&output_path)?;
        let saved_image = image::open(output_path)?;
        assert_eq!(saved_image.dimensions(), (64, 64));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
