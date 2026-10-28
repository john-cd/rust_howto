//! Runs the video example.
//!
//! TODO Reviewed ??-??-2026.

mod video;

/// Runs the video example.
///
/// # Errors
/// Returns an error if encoder setup or frame encoding fails.
fn main() -> anyhow::Result<()> {
    video::run()
}
