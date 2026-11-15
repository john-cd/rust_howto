//! Runs the data formats example.
//!
//! TODO Reviewed ??-??-2026.

/// Parses an inline glTF triangle and reports its vertex and index counts.
mod inline_gltf_triangle;

/// Runs the inline glTF triangle example.
fn main() -> Result<(), ::gltf::Error> {
    inline_gltf_triangle::run()
}
