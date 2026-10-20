#![allow(dead_code)]
use std::error::Error;

use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;

// ANCHOR: example
/// A simple example that plays a 440Hz sine wave (A4) for 1 second.
fn main() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("no output device available")?;
    let config = device.default_output_config()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => run_stream::<f32>(&device, &config.into())?,
        cpal::SampleFormat::I16 => run_stream::<i16>(&device, &config.into())?,
        cpal::SampleFormat::U16 => run_stream::<u16>(&device, &config.into())?,
        _ => return Err("unsupported sample format".into()),
    }

    Ok(())
}

fn run_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<(), Box<dyn Error>>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    let sample_rate = config.sample_rate as f32;
    let channels = config.channels as usize;

    // Produce a 440Hz sine wave.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value: T = T::from_sample(next_value());
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    // Play for 1 second.
    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "requires audio device"]
    fn test_main() {
        main().unwrap();
    }
}
