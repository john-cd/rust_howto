//! Compares a parallel CPU image blur with GPU compute including host transfers.

mod cpu;
mod gpu;

use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use anyhow::ensure;

const SAMPLES: usize = 7;
const WIDTHS: [u32; 4] = [130, 512, 1024, 2048];

/// Generates deterministic grayscale pixels with edges and texture.
fn pixels(width: u32) -> Vec<u32> {
    (0..width * width)
        .map(|index| {
            let x = index % width;
            let y = index / width;
            (13 * x + 19 * y + ((x ^ y) & 31) * 11) & 255
        })
        .collect()
}

/// Sorts a nonempty odd number of samples and returns their middle value.
fn median(samples: &mut [Duration; SAMPLES]) -> Duration {
    samples.sort_unstable();
    samples[SAMPLES / 2]
}

/// Verifies each blur and reports median CPU and end-to-end GPU timings.
///
/// # Errors
///
/// Returns an error when adapter setup, execution, or result comparison fails.
fn main() -> Result<()> {
    let gpu = pollster::block_on(gpu::Gpu::new())?;
    println!("Rayon threads: {}", rayon::current_num_threads());
    println!("width,CPU ms,GPU upload+compute+readback ms,GPU resident ms,CPU/end-to-end GPU");

    for width in WIDTHS {
        let input = pixels(width);
        let mut scratch = vec![0; input.len()];
        let mut reference = vec![0; input.len()];
        let mut actual = vec![0; input.len()];
        let image = gpu.prepare(width);

        // Warm both paths before starting the samples.
        cpu::blur(width as usize, &input, &mut scratch, &mut reference);
        gpu.blur(&image, &input, &mut actual)?;
        ensure!(
            actual == reference,
            "warm-up mismatch for {width} pixels per row"
        );
        gpu.blur_resident(&image)?;

        let mut cpu_times = [Duration::ZERO; SAMPLES];
        let mut gpu_times = [Duration::ZERO; SAMPLES];
        let mut resident_times = [Duration::ZERO; SAMPLES];
        for ((cpu_time, gpu_time), resident_time) in cpu_times
            .iter_mut()
            .zip(&mut gpu_times)
            .zip(&mut resident_times)
        {
            let start = Instant::now();
            cpu::blur(width as usize, &input, &mut scratch, &mut reference);
            *cpu_time = start.elapsed();

            let start = Instant::now();
            gpu.blur(&image, &input, &mut actual)?;
            *gpu_time = start.elapsed();
            ensure!(
                actual == reference,
                "output mismatch for {width} pixels per row"
            );

            let start = Instant::now();
            gpu.blur_resident(&image)?;
            *resident_time = start.elapsed();
        }

        let cpu_ms = median(&mut cpu_times).as_secs_f64() * 1000.0;
        let gpu_ms = median(&mut gpu_times).as_secs_f64() * 1000.0;
        let resident_ms = median(&mut resident_times).as_secs_f64() * 1000.0;
        println!(
            "{width},{cpu_ms:.3},{gpu_ms:.3},{resident_ms:.3},{:.2}",
            cpu_ms / gpu_ms
        );
    }
    Ok(())
}
