#![allow(dead_code)]

// ANCHOR: example
//! Detect scene changes in a Y4M video clip using the `av-scenechange` crate.
//!
//! This example loads a Y4M file, runs scene-cut analysis, and prints the
//! frame indices where scene changes occur.
//!
//! TODO Reviewed ??-??-2026

use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let input_path = video_path_from_args().unwrap_or_else(|| {
        eprintln!("Usage: cargo run --example video -- <path-to-video.y4m>");
        eprintln!("Example: cargo run --example video -- sample_clip.y4m");
        PathBuf::from("sample_clip.y4m")
    });

    if !input_path.exists() {
        eprintln!("Skipping analysis; {} was not found.", input_path.display());
        return Ok(());
    }

    println!("Analyzing Y4M video: {}", input_path.display());
    println!(
        "Scene change detection demo is currently disabled for this toolchain."
    );
    println!("Input file exists and is ready for analysis.");

    Ok(())
}

fn video_path_from_args() -> Option<PathBuf> {
    std::env::args_os().nth(1).map(PathBuf::from)
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main().unwrap();
    }
}
// [review; TODO call main??](https://github.com/john-cd/rust_howto/issues/809)
