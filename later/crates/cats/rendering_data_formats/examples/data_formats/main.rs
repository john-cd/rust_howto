//! Runs the data formats example.
//!
//! TODO Reviewed ??-??-2026.

mod gltf_example;

/// Runs the glTF example.
fn main() -> Result<(), ::gltf::Error> {
    gltf_example::run()
}
