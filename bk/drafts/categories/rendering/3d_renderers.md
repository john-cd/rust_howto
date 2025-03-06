# 3D Renderers

{{#include 3d_renderers.incl.md}}

Graphics APIs, wrappers for and backends to Vulkan and OpenGL.

3D rendering is the 3D computer graphics process of converting 3D models into 2D images.

## Topics

- Forward Rendering
- Deferred Rendering
- Physically Based Rendering (PBR)
- Ray Tracing
- Global Illumination
- Shadow Mapping
- Ambient Occlusion
- Level of Detail (LOD)

Many algorithms are implemented within engines like `rend3` or game engines like `bevy`.

## `rend3` {#rend3}

[`rend3`][c-rend3]⮳{{hi:rend3}} is a 3D rendering engine built on top of `wgpu`.

## `three-d` {#threed}

A OpenGL/WebGL/OpenGL ES renderer which seeks to make graphics simple but still have the power to efficiently draw exactly what you want.

## `kiss3d` {#kiss3d}

Keep It Simple, Stupid 3d graphics engine.

This library is born from the frustration that today’s 3D graphics library are either:

- Too low level: you have to write your own shaders and opening a window takes 8 hours, 300 lines of code and 10L of coffee.
- High level, but too hard to understand/use: these libraries are made to create beautiful photoreal (or close to it) animations or games. They have many features; too many, in fact, if you just want to draw a few objects on the screen with as little friction as possible.
kiss3d is not designed to be feature-complete or fast. It is designed to let you draw simple geometric figures and play with them with as little friction as possible.

## Hardware Acceleration

Often depends on the 3D API or rendering engine used. Libraries like [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, and [`glium`][c-glium]⮳{{hi:glium}} provide access to hardware acceleration through graphics APIs like WebGPU, OpenGL, Vulkan, Metal, and DirectX.

See:

- [[rendering_graphics-api | Rendering: Graphics API]].
- [[gpu_abstraction_layers | GPU Abstraction Layers]].
- [[native_graphics_apis | Native Graphics APIs]].
- [[vulkan | Vulkan]].
- [[opengl | OpenGL]].

### Applications

- [[game-development | Game Development]].
- [[game_engines | Game Engines]].
- [[gui | GUI]].

## See also

- [Are we Game yet? 3D Rendering](https://arewegameyet.rs/ecosystem/3drendering).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write. decide what to cover
See also `fast-poisson-disk` for generating point distributions.
</div>
