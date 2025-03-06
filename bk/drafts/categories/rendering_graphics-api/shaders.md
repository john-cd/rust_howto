# Shaders

{{#include shaders.incl.md}}

Languages and tools for writing, compiling, and using shaders.

Shaders are programs that run on the GPU (Graphics Processing Unit). There are several types of shaders, including:

- Vertex Shaders: These shaders process the attributes of each vertex in a 3D model. They can transform vertex positions, apply lighting calculations, and manipulate vertex data before it is rasterized into pixels.
- Fragment Shaders (or Pixel Shaders): These shaders handle the color and other attributes of individual pixels. They determine the final color of each pixel by applying textures, lighting effects, and other visual effects.
- Geometry Shaders: These shaders work with entire primitives (points, lines, or triangles) and can generate new geometry or modify existing geometry before it is rasterized.
- Compute Shaders: These shaders are used for general-purpose computing tasks that are not directly related to rendering. They can perform complex calculations and data processing on the GPU.

Shaders are written in specialized programming languages like GLSL (OpenGL Shading Language), HLSL (High-Level Shading Language for DirectX), and Metal Shading Language (for Apple's Metal API). `wgpu` uses WGSL.

## `naga` {#naga}

Shader translator and validator. Part of the `wgpu` project.

## `rust-gpu` {#rust-gpu}

Making Rust a first-class language and ecosystem for GPU shaders

## `cubecl` {#cubecl}

Multi-platform high-performance compute language extension for Rust.

## `spirv-reflect` {#spirv-reflect}

## Compute Shaders (GPGPU)

Compute Shaders are often handled through the low-level graphics APIs like [`wgpu`][c-wgpu]⮳{{hi:wgpu}} or [`vulkano`][c-vulkano]⮳{{hi:vulkano}}.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

## See also

- [[typecasts | Typecasts]] and especially `bytemuck` - A crate for mucking around with piles of bytes.
- [Are we Game e=yet? - Shaders](https://arewegameyet.rs/ecosystem/shader/)

<div class="hidden">
TODO write
</div>
