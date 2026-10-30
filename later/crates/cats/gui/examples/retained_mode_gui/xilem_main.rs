//! Standalone entry point for the Xilem counter example.

#[path = "xilem.rs"]
mod xilem;

/// Runs the native Xilem counter window.
fn main() -> Result<(), winit::error::EventLoopError> {
    xilem::run()
}
