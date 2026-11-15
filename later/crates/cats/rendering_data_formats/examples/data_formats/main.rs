//! Runs the data formats example.
//!
//! TODO Reviewed ??-??-2026.

/// Parses an inline glTF triangle and reports its vertex and index counts.
mod inline_gltf_triangle;

/// Runs the inline glTF triangle example.
fn main() -> Result<(), ::gltf::Error> {
    inline_gltf_triangle::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
