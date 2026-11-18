<!-- TODO Reviewed ??-??-2026. -->

# Rendering Graphics API

Book code examples for low-level Rendering Graphics APIs like WGPU. It demonstrates integration with key ecosystem crates including `pollster`, `wgpu`.

## Examples Covered

- **Gpu Abstraction Layers** (`examples/gpu_abstraction_layers/`): Demonstrates concepts like `wgpu`.
- **Native Graphics APIS** (`examples/native_graphics_apis/`): Demonstrates concepts like `wgpu_adapter_info`.
- **Opengl** (`examples/opengl/`): Demonstrates loading `glow` function pointers and issuing a clear call when a current context is supplied.
- **Shaders** (`examples/shaders/`): Code examples covering shaders.
- **Vulkan** (`examples/vulkan/`): Code examples covering vulkan.

## Usage

To check or run examples in this crate:

```console
cargo check --package rendering_graphics_api --examples
cargo test --package rendering_graphics_api
```

Code examples correspond to chapters in `later/src/categories/rendering_graphics-api` or related book sections.

## Viewport and Readback Contracts

The `frame_layout` example uses wgpu's alignment constant but does not request a GPU:

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p rendering_graphics_api --features wgpu --example frame_layout
```

It tests checked RGBA8 row padding/unpacking, aspect-preserving viewports, and pointer
mapping. See `later/src/categories/rendering_graphics-api/frame_layout.md`. The separate
`offscreen_render` example performs real GPU render/map/pixel checks.
