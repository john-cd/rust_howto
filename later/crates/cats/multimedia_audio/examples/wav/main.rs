//! Runs the in-memory WAV example.

mod wav;

/// Runs the in-memory WAV example.
fn main() {
    wav::run().expect("writing and reading a WAV buffer should succeed");
}
