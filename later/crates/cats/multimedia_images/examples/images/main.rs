//! Runs the images example.
//!
//! TODO Reviewed ??-??-2026

mod images;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    images::run()
}
