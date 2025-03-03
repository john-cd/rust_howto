# Rendering Engines

[![cat-rendering::engine][cat-rendering::engine-badge]][cat-rendering::engine]{{hi:Rendering engine}}

High-level solutions for rendering on the screen.

{{#include rendering_engines.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/955)

## Rendering Engines (Built on Low-Level APIs)

[`rend3`][c-rend3]⮳{{hi:rend3}}: A 3D rendering engine built on top of wgpu. Provides higher-level abstractions for rendering.
[`bevy_render`][c-bevy_render]⮳{{hi:bevy_render}}: Bevy's rendering system (part of the Bevy game engine).

| Rendering Engine/Library | Rust Crates/Ecosystem | Focus/Features |
| 2D/3D - General Purpose Engines | [`bevy`][c-bevy]⮳{{hi:bevy}}| Data-driven, modular game engine built on top of WGPU. Focuses on performance and developer experience. Supports 2D and 3D rendering. |
| 2D/3D - Lower Level Graphics| [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, [`glium`][c-glium]⮳{{hi:glium}} | [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a cross-platform, low-level graphics API that exposes modern GPU capabilities. [`vulkano`][c-vulkano]⮳{{hi:vulkano}} is a Rust wrapper around the Vulkan API. [`glium`][c-glium]⮳{{hi:glium}} is an older, but simpler OpenGL wrapper.These are lower-level and require more detailed graphics programming knowledge. |

</div>
