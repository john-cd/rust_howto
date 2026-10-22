//! Runs the video example.
//!
//! TODO Reviewed ??-??-2026

mod video;

/// Runs the example from the `video` module.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    video::run()
}
