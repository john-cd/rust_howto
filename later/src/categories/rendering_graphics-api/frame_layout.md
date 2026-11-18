<!-- TODO Reviewed ??-??-2026. -->

# Viewports and GPU Readback Layout

A rendered frame crosses several coordinate and memory boundaries: logical UI points,
physical pixels, texture coordinates, and staging-buffer bytes. State each boundary
explicitly. Use physical target dimensions for rendering, and convert pointer positions
using the window scale factor before comparing them with a physical-pixel viewport.

## Fit the Image and Unpack Its Rows

The example centers a source image inside a target without stretching its aspect ratio.
It also computes checked RGBA8 readback strides and strips padding from mapped rows.
A 65-pixel row uses 260 image bytes but needs a 512-byte staging stride for a buffer-texture
copy. Copying the whole mapped buffer into an image would introduce spurious padding.

```rust,ignore,noplayground
{{#include ../../../crates/cats/rendering_graphics_api/examples/frame_layout.rs:example}}
```

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p rendering_graphics_api --features wgpu --example frame_layout
just cargo run --manifest-path later/crates/Cargo.toml --locked -p rendering_graphics_api --features wgpu --example frame_layout
```

These are CPU layout tests; they do not request an adapter. The separate
[offscreen workflow](../rendering_engine/rendering_engines.md#offscreen) performs the
actual render, map and pixel checks. Its 64-pixel rows happen to need no padding.
The helper here covers arbitrary nonzero widths within a 64 MiB demonstration budget.
For a real copy, use the padded stride as `bytes_per_row`, allocate the padded size,
wait for mapping completion, and then unpack the image rows before encoding/comparison.
See [wgpu row alignment](https://docs.rs/wgpu/30.0.1/wgpu/constant.COPY_BYTES_PER_ROW_ALIGNMENT.html).
This layout is specifically RGBA8, one layer, one mip; compressed block textures need
a format-aware calculation rather than multiplying width by four.

Zero-sized surfaces should suspend presentation rather than trigger a zero-sized texture
allocation. On restoration, recompute the surface configuration and viewport. A pointer
in the letterbox bars is outside the image; reject it before converting to image-local
coordinates. Keep model/view/projection and input hit-testing transforms consistent.

## Choose the Rendering Layer

| Layer | Common crates | Coverage and next boundary |
| --- | --- | --- |
| Cross-platform GPU | `wgpu`, `naga` | Shader validation, compute, offscreen rendering and CPU layout here. A visible surface needs window/lifetime and resize handling. |
| Native API access | `ash`, `glow`, `glutin` | [Vulkan](vulkan.md), [OpenGL](opengl.md), [native APIs](native_graphics_apis.md). OpenGL calls require a current compatible context; loading functions alone is insufficient. |
| Math and vertex data | `glam`, `nalgebra`, `bytemuck` | Projection and buffer preparation; document handedness, depth range, alignment and shader layout. A castable Rust struct is not proof of a matching WGSL layout. |
| CPU/vector drawing | `tiny-skia`, `lyon`, `vello` | Published [Graphics](https://john-cd.com/rust_howto/categories/graphics/index.html) and staged [vector rendering](../rendering/2d_vector_graphics.md). Separate tessellation from drawing. |
| Scene/engine ownership | `bevy`, `three-d`, `kiss3d` | [Rendering Engines](../rendering_engine/rendering_engines.md); the direct wgpu example does not test these engines. |
| Assets and images | `gltf`, `image`, `resvg`, `usvg` | [Rendering Data Formats](../rendering_data-formats/index.md); decoding an asset is separate from uploading or presenting it. |

Apply color-space and alpha conventions consistently when UI textures, decoded images,
and render targets meet. Compare test pixels away from antialiased edges, and use
appropriate tolerances for the format rather than assuming all drivers emit identical
edge bytes. Surface/device loss and native presentation remain separate platform checks.

## Related Topics

- [Rendering Engines](../rendering_engine/rendering_engines.md).
- [Window Creation](../gui/window_creation.md).
- [Rendering Data Formats](../rendering_data-formats/index.md).
