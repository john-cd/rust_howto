//! Runs the data formats example.
//!
//! TODO Reviewed ??-??-2026.

mod inline_gltf_triangle;

/// Runs the inline glTF triangle example.
fn main() -> Result<(), ::gltf::Error> {
    inline_gltf_triangle::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
