//! Runs the gpu abstraction layers example.
//!
//! TODO Reviewed ??-??-2026

#[cfg(feature = "wgpu")]
mod wgpu;

/// Runs the example from the `wgpu` module where it is supported.
fn main() {
    #[cfg(feature = "wgpu")]
    {
        wgpu::run();
    }
}
