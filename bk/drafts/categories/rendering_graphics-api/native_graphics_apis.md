# Native Graphics APIs (Bindings)

{{#include native_graphics_apis.incl.md}}

Low-Level Rendering Graphics APIs include:

- Direct3D (DirectX).
- Metal.
- OpenGL.
- Vulkan.
- WebGL and WebGPU.

## Vulkan {#skip}

See [[vulkan | Vulkan]].

## OpenGL {#skip}

See [[opengl | OpenGL]].

## Low-Level Graphics (Direct3D 12 - Windows) {#d3d12}

Direct3D 12 access is usually through [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}'s backend or lower level bindings like `d3d12`. Windows-specific.

[`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (with a D3D12 backend), `d3d12` (more raw bindings)

```rust,editable
{{#include ../../../crates/cats/rendering_graphics_api/tests/graphics.rs:example}}
```

## Low-Level Graphics (Metal) {#metal}

Metal access is provided by the [`metal`][c-metal]⮳{{hi:metal}} crate or through [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (with a Metal backend). macOS/iOS specific.

See also [`metal-rs`][c-metal]⮳{{hi:metal-rs}}: 'Metal' bindings.

## Related Topics {#skip}

- [[gpu | GPU]].
- [[gpu_abstraction_layers | GPU Abstraction Layers]].
- [[rendering | Rendering]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[graphics_apis: write](https://github.com/john-cd/rust_howto/issues/457)
</div>
