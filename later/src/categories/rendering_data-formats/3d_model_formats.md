<!-- TODO Reviewed ??-??-2026 -->

# 3D Model Formats

[![cat~rendering::data-formats][cat~rendering::data-formats~badge]][cat~rendering::data-formats]{{hi:3D models}}{{hi:Rendering data formats}}

glTF is a portable format for 3D scenes, meshes, materials, and animation. A `.gltf` file stores JSON plus referenced assets; `.glb` packages the JSON and binary data together.

## Parse a glTF document {#data-format}

[![gltf][c~gltf~docs~badge]][c~gltf~docs]{{hi:gltf}}

The [`gltf`][c~gltf~docs]↗ crate parses and validates glTF 2.0 documents. This example parses a small in-memory document and verifies that its default scene is available.

```rust,editable
{{#include ../../../crates/cats/rendering_data_formats/examples/data_formats/gltf_example.rs:example}}
```

## Other 3D mesh formats {#other-mesh-formats}

- [![tobj][c~tobj~docs~badge]][c~tobj~docs] [`tobj`][c~tobj~docs]↗ reads Wavefront OBJ meshes and is suitable for applications that need a small, focused mesh loader.
- [![stl_io][c~stl_io~docs~badge]][c~stl_io~docs] [`stl_io`][c~stl_io~docs]↗ reads and writes STL triangle meshes, a format commonly used for 3D printing.
- [![ply-rs][c~ply-rs~docs~badge]][c~ply-rs~docs] [`ply-rs`][c~ply-rs~docs]↗ parses PLY files, which can represent polygon meshes and point clouds.

See the [rendering data formats index](index.md) for image, font, vector, and texture formats.

## Related Topics {#related-topics .skip}

- [[rendering | Rendering]].
- [[rendering_engine | Rendering Engine]].
- [[3d_renderers | 3D Renderers]].
- [[multimedia_images | Multimedia Images]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
