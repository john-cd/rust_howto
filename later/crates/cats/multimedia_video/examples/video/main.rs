//! Runs the video example.
//!
//! TODO Reviewed ??-??-2026.

mod video;

/// Runs the video example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    video::run()
}
