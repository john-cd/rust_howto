// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parses an inline glTF triangle and reports its vertex and index counts.
//!
//! glTF is the "JPEG of 3D": an open format for 3D models used by engines and
//! web viewers. A mesh is stored as vertices (points in space) and indices
//! (which vertices form each triangle).

const INLINE_GLTF_TRIANGLE: &[u8] = br#"{
  "asset": {"version": "2.0"},
  "scene": 0,
  "scenes": [{"nodes": [0]}],
  "nodes": [{"mesh": 0}],
  "meshes": [{"primitives": [{"attributes": {"POSITION": 0}, "indices": 1}]}],
  "buffers": [{"uri": "data:application/octet-stream;base64,AAAAAAAAAAAAAAAAAACAPwAAAAAAAAAAAAAAAAAAgD8AAAAAAAABAAIA", "byteLength": 42}],
  "bufferViews": [
    {"buffer": 0, "byteOffset": 0, "byteLength": 36},
    {"buffer": 0, "byteOffset": 36, "byteLength": 6}
  ],
  "accessors": [
    {"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3", "min": [0, 0, 0], "max": [1, 1, 0]},
    {"bufferView": 1, "componentType": 5123, "count": 3, "type": "SCALAR"}
  ]
}"#;

/// Parses the inline glTF triangle and returns its vertex and index counts.
fn parse_inline_triangle() -> Result<(usize, usize), ::gltf::Error> {
    let document = ::gltf::Gltf::from_slice(INLINE_GLTF_TRIANGLE)?;
    let primitives: Vec<_> = document
        .document
        .meshes()
        .flat_map(|mesh| mesh.primitives())
        .collect();

    let vertex_count = primitives
        .iter()
        .filter_map(|primitive| primitive.get(&::gltf::Semantic::Positions))
        .map(|accessor| accessor.count())
        .sum();
    #[allow(
        clippy::redundant_closure_for_method_calls,
        reason = "the method path does not accept the iterator's lifetime"
    )]
    let index_count = primitives
        .iter()
        .filter_map(|primitive| primitive.indices())
        .map(|accessor| accessor.count())
        .sum();

    Ok((vertex_count, index_count))
}

/// Runs the example and prints the parsed geometry counts.
fn main() -> Result<(), ::gltf::Error> {
    let (vertex_count, index_count) = parse_inline_triangle()?;
    println!(
        "Parsed a glTF triangle with {vertex_count} vertices and {index_count} indices."
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), ::gltf::Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::main;
    use super::parse_inline_triangle;

    /// Verifies the inline triangle has three vertices and three indices.
    #[test]
    fn inline_triangle_has_three_vertices_and_indices()
    -> Result<(), ::gltf::Error> {
        assert_eq!(parse_inline_triangle()?, (3, 3));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
