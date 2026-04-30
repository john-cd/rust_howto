#![allow(dead_code)]
// ANCHOR: example
//! Detect scene changes in a Y4M video clip using the `av-scenechange` crate.
//!
//! This example loads a Y4M file, runs scene-cut analysis, and prints the
//! frame indices where scene changes occur.

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use av_scenechange::DetectionOptions;
use av_scenechange::decoder::Decoder;
use av_scenechange::decoder::Y4mDecoder;
use av_scenechange::detect_scene_changes;
use av_scenechange::v_frame::pixel::U8;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let input_path = video_path_from_args().unwrap_or_else(|| {
        eprintln!("Usage: cargo run --example video -- <path-to-video.y4m>");
        eprintln!("Example: cargo run --example video -- sample_clip.y4m");
        std::process::exit(1);
    });

    let input_file = File::open(&input_path)?;
    let reader = BufReader::new(input_file);
    let y4m_decoder = Y4mDecoder::new(reader)?;

    println!("Analyzing Y4M video: {}", input_path.display());
    println!(
        "Video dimensions: {}x{}",
        y4m_decoder.get_width(),
        y4m_decoder.get_height()
    );
    println!("Frame rate: {:?}", y4m_decoder.get_framerate());

    let mut decoder = Decoder::Y4m(y4m_decoder);
    let opts = DetectionOptions {
        detect_flashes: false,
        lookahead_distance: 16,
        ..Default::default()
    };

    let results = detect_scene_changes::<_, U8>(
        &mut decoder,
        opts,
        None,
        Some(&|frames, keyframes| {
            if frames % 50 == 0 {
                println!(
                    "  progress: analyzed {} frames, {} keyframes",
                    frames, keyframes
                );
            }
        }),
    )?;

    println!("Scene change detection complete.");
    println!("Total frames read: {}", results.frame_count);
    println!("Detected {} scene changes:", results.scene_changes.len());
    println!("{:?}", results.scene_changes);
    println!("Average scan speed: {:.2} FPS", results.speed);

    Ok(())
}

fn video_path_from_args() -> Option<PathBuf> {
    std::env::args_os().nth(1).map(PathBuf::from)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_detection_options_builds() {
    //     let opts: DetectionOptions = Default::default();
    //     assert!(!opts.detect_flashes);
    // }
}
// [review; TODO call main??](https://github.com/john-cd/rust_howto/issues/809)
