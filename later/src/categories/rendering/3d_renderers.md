<!-- TODO Reviewed ??-??-2026 -->

# 3D Renderers

{{#include 3d_renderers.incl.md}}

Graphics APIs, wrappers for and backends to [Vulkan][p~vulkan] and OpenGL.

3D [rendering][p~rendering] is the 3D computer graphics process of converting 3D models into 2D images.

## Topics {#topics}

- Forward Rendering.
- Deferred Rendering.
- Physically Based Rendering (PBR).
- Ray Tracing.
- Global Illumination.
- Shadow Mapping.
- Ambient Occlusion.
- Level of Detail (LOD).

Many algorithms are implemented within engines like [`rend3`][c~rend3~docs]↗{{hi:rend3}} or game engines like [`bevy`][c~bevy~docs]↗{{hi:bevy}}.

## Project a Vertex to the Screen {#project-a-vertex}

Every 3D renderer transforms model-space vertices by a model, view, and projection matrix, divides by `w`, and maps the result to pixels. This example performs that vertex stage on the CPU with [`glam`][c~glam~docs]↗{{hi:glam}}. It needs no window or GPU, and the same matrices are what you would upload to a shader. The camera helpers live in `glam::camera`; pick the module that matches your graphics API's handedness and depth range.

```rust,editable
{{#include ../../../crates/cats/rendering/examples/3d_renderers/main.rs:example}}
```

## `rend3` {#rend3}

[`rend3`][c~rend3~docs]↗{{hi:rend3}} is a 3D rendering engine built on top of [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}.

## `three-d` {#threed}

A OpenGL/WebGL/OpenGL ES renderer which seeks to make graphics simple but still have the power to efficiently draw exactly what you want.

## `kiss3d` {#kiss3d}

Keep It Simple, Stupid 3d graphics engine.

This library is born from the frustration that today's 3D graphics library are either:

- Too low level: you have to write your own [shaders][p~shaders] and opening a window takes 8 hours, 300 lines of code and 10L of coffee.
- High level, but too hard to understand/use: these libraries are made to create beautiful photoreal (or close to it) animations or games. They have many features; too many, in fact, if you just want to draw a few objects on the screen with as little friction as possible.
  kiss3d is not designed to be feature-complete or fast. It is designed to let you draw simple geometric figures and play with them with as little friction as possible.

## Engine Renderers {#engine-renderers}

[`bevy_render`][c~bevy_render~docs]↗{{hi:bevy_render}} is the rendering system of the [`bevy`][c~bevy~docs]↗{{hi:bevy}} game engine. `three-rs` is an older, unmaintained three.js-style renderer; prefer [`three-d`](#threed) or [`bevy_render`][c~bevy_render~docs]↗.

## Hardware Acceleration {#hardware-acceleration}

Often depends on the 3D API or [rendering engine][p~rendering-engine] used. Libraries like [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}}, and [`glium`][c~glium~docs]↗{{hi:glium}} provide access to hardware acceleration through graphics APIs like WebGPU, OpenGL, Vulkan, Metal, and DirectX.

See:

- [[rendering_graphics-api | Rendering: Graphics API]].
- [[gpu_abstraction_layers | GPU Abstraction Layers]].
- [[native_graphics_apis | Native Graphics APIs]].
- [[vulkan | Vulkan]].
- [[opengl | OpenGL]].

### Applications {#applications}

- [[game-development | Game Development]].
- [[game_engines | Game Engines]].
- [[gui | GUI]].

## See Also {#see-also .skip}

- [Are we Game yet? 3D Rendering][are-we-game-yet?-3d-rendering~website]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
