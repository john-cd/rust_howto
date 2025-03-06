# Graphics APIs

{{#include graphics_apis.incl.md}}

| | [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, [`glium`][c-glium]⮳{{hi:glium}} | [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a cross-platform, low-level graphics API that exposes modern GPU capabilities. [`vulkano`][c-vulkano]⮳{{hi:vulkano}} is a Rust wrapper around the Vulkan API. These are lower-level and require more detailed graphics programming knowledge. |

## OpenGL {#opengl}

- gl
- glow

[`glium`][c-glium]⮳{{hi:glium}} is an older, but simpler OpenGL wrapper.

## Low-Level Graphics (Direct3D 12 - Windows) {#d3d12}

[`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (with a D3D12 backend), `d3d12` (more raw bindings) |

Direct3D 12 access is usually through [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}'s backend or lower level bindings like `d3d12`. Windows-specific.

```rust,editable
{{#include ../../../crates/cats/rendering_graphics_api/tests/graphics.rs:example}}
```

## Low-Level Graphics (Metal) {#metal}

Metal access is provided by the [`metal`][c-metal]⮳{{hi:metal}} crate or through [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (with a Metal backend). macOS/iOS specific.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[graphics_apis: write](https://github.com/john-cd/rust_howto/issues/457)
</div>
