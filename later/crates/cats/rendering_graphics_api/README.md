<!-- TODO Reviewed ??-??-2026 -->

# Rendering Graphics API

Book code examples for low-level Rendering Graphics APIs like WGPU. It demonstrates integration with key ecosystem crates including `pollster`, `wgpu`.

## Examples Covered

- **Gpu Abstraction Layers** (`examples/gpu_abstraction_layers/`): Demonstrates concepts like `wgpu`.
- **Native Graphics APIS** (`examples/native_graphics_apis/`): Demonstrates concepts like `wgpu_adapter_info`.
- **Opengl** (`examples/opengl/`): Code examples covering opengl.
- **Shaders** (`examples/shaders/`): Code examples covering shaders.
- **Vulkan** (`examples/vulkan/`): Code examples covering vulkan.

## Usage

To check or run examples in this crate:

```console
cargo check --package rendering_graphics_api --examples
cargo test --package rendering_graphics_api
```

Code examples correspond to chapters in `bk/src/categories/rendering_graphics_api` or related book sections.
