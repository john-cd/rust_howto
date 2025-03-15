# Shaders (GLSL, HLSL, SPIR-V)

{{#include shaders.incl.md}}

Languages and tools for writing, compiling, and using shaders.

Shaders are programs that run on the GPU (Graphics Processing Unit). There are several types of shaders, including:

- Vertex Shaders: These shaders process the attributes of each vertex in a 3D model. They can transform vertex positions, apply lighting calculations, and manipulate vertex data before it is rasterized into pixels.
- Fragment Shaders (or Pixel Shaders): These shaders handle the color and other attributes of individual pixels. They determine the final color of each pixel by applying textures, lighting effects, and other visual effects.
- Geometry Shaders: These shaders work with entire primitives (points, lines, or triangles) and can generate new geometry or modify existing geometry before it is rasterized.
- Compute Shaders: These shaders are used for general-purpose computing tasks that are not directly related to rendering. They can perform complex calculations and data processing on the GPU.

Shaders are written in specialized programming languages like GLSL (OpenGL Shading Language), HLSL (High-Level Shading Language for DirectX), and Metal Shading Language (for Apple's Metal API). [`wgpu`][c-wgpu]⮳{{hi:wgpu}} uses WGSL.

## Topics {#skip}

- SPIR-V compilation.
- GLSL/HLSL compilation.
- Compute Shaders.
- GPU Compute.

## `naga` {#naga}

[`naga`][c-naga]⮳{{hi:naga}} is a general-purpose shader translation and analysis tool.

Shader translator and validator. Part of the [`wgpu`][c-wgpu]⮳{{hi:wgpu}} project.

## `rust-gpu` {#rust-gpu}

Making Rust a first-class language and ecosystem for GPU shaders

## `cubecl` {#cubecl}

Multi-platform high-performance compute language extension for Rust.

## GLSL/HLSL compilation {#skip}

TODO

## SPIR-V {#skip}

### `glsl-to-spirv` {#skip}

[`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}} compiles GLSL to SPIR-V.

### `spirv-builder` {#skip}

For building spirv modules.

### `spirv-reflect` {#spirv-reflect}

For inspecting SPIR-V shaders

### `shaderc` {#shaderc}

[`shaderc`][c-shaderc]⮳{{hi:shaderc}} is a Rust wrapper for the shaderc library (compiles GLSL/HLSL to SPIR-V).

## Compute Shaders and GPU Compute (GPGPU) {#skip}

Compute Shaders are often handled through the low-level graphics APIs like [`wgpu`][c-wgpu]⮳{{hi:wgpu}} or [`vulkano`][c-vulkano]⮳{{hi:vulkano}}.

## Related Topics {#skip}

- [[gpu | GPU]].
- [[gpu_abstraction_layers | GPU Abstraction Layers]].

## See Also {#skip}

- [[typecasts | Typecasts]] and especially [`bytemuck`][c-bytemuck]⮳{{hi:bytemuck}} - A crate for mucking around with piles of bytes.
- [Are we Game yet? - Shaders](https://arewegameyet.rs/ecosystem/shader/).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO decide what to cover, write
review `gpu-allocator`
</div>
