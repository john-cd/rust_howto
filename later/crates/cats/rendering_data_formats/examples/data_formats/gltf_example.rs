#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parses a small glTF document with the `gltf` crate.

/// Parses a minimal glTF document that contains one empty default scene.
fn parse_minimal_gltf() -> Result<::gltf::Gltf, ::gltf::Error> {
    ::gltf::Gltf::from_slice(
        br#"{"asset":{"version":"2.0"},"scene":0,"scenes":[{"nodes":[]}]}"#,
    )
}

/// Parses a glTF document and reports how many scenes it contains.
fn main() -> Result<(), ::gltf::Error> {
    let document = parse_minimal_gltf()?;
    let scene_count = document.document.scenes().count();
    println!("Parsed a glTF document with {scene_count} scene(s).");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), ::gltf::Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that parsing preserves the document's default scene.
    #[test]
    fn parses_minimal_gltf_with_default_scene() -> Result<(), ::gltf::Error> {
        let document = parse_minimal_gltf()?;

        assert_eq!(document.document.scenes().count(), 1);
        assert!(document.document.default_scene().is_some());

        Ok(())
    }
}
