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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
