# WebGPU

{{#include webgpu.incl.md}}

For WebGPU development in Rust, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is the essential crate. You'll use it to interact with the WebGPU API, manage resources, configure pipelines, and write shaders in WGSL.

## Key Concepts

- WebGPU API: The API for interacting with the GPU.
- Shaders: Programs that run on the GPU.
- Pipelines: The sequence of steps involved in rendering.
- Buffers: Memory on the GPU.
- Textures: [Images][p-images] used in rendering.
- Bind Groups: How shaders access resources.

## WebGPU Bindings with `wgpu` {#wgpu}

[![wgpu-website][c-wgpu-website-badge]][c-wgpu-website] [![wgpu][c-wgpu-badge]][c-wgpu] [![wgpu-crates.io][c-wgpu-crates.io-badge]][c-wgpu-crates.io] [![wgpu-github][c-wgpu-github-badge]][c-wgpu-github] [![wgpu-lib.rs][c-wgpu-lib.rs-badge]][c-wgpu-lib.rs]{{hi:wgpu}}{{hi:Graphics}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

[`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a Rusty [WebGPU][wikipedia-webgpu]{{hi:WebGPU}}⮳ API wrapper. It is a cross-platform, safe, pure-Rust [graphics][p-graphics] API. It runs natively on Vulkan, Metal, D3D12, and OpenGL; and on top of WebGL2 and WebGPU on WASM. The API is based on the [WebGPU standard][webgpu-website]⮳. It serves as the core of the WebGPU integration in Firefox and Deno.

```rust,editable
{{#include ../../../crates/cats/graphics/tests/wgpu.rs:example}}
```

### Shader Language

WGSL (WebGPU Shading Language) is the shader language used by WebGPU. You'll write your shaders in WGSL. There aren't specific Rust crates for WGSL itself, but you'll use it in your shader code (which is then loaded by [`wgpu`][c-wgpu]⮳{{hi:wgpu}}).

### Compute Shaders

You write compute shaders in WGSL and then use the [`wgpu`][c-wgpu]⮳{{hi:wgpu}} API to dispatch them.

### Graphics Pipeline

You configure the [graphics][p-graphics] pipeline using the [`wgpu`][c-wgpu]⮳{{hi:wgpu}} API.

### Texture Handling

[`wgpu`][c-wgpu]⮳{{hi:wgpu}} provides ways to create and manage textures.

### Debugging

WebGPU has built-in debugging capabilities that can be accessed through browser developer tools (if you're targeting the web) or through debugging tools specific to your platform (if you're targeting native).

## Related Topics

### Mathematics (Linear Algebra)

See [[mathematics | Mathematics]]. The key crates are:

- [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}: A popular [linear algebra][p-linear-algebra] library. Essential for graphics programming.
- [`glam`][c-glam]⮳{{hi:glam}}: Another [linear algebra][p-linear-algebra] library, often used in game and graphics development.

## Windowing and Event Handling

[`winit`][c-winit]⮳{{hi:winit}}, a [window creation][p-window-creation] and event handling library, is often used with [`wgpu`][c-wgpu]⮳{{hi:wgpu}}.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[webgpu: write](https://github.com/john-cd/rust_howto/issues/375)
review [rust-gpu-ecosystem-website]: https://rust-gpu.github.io/ecosystem
review in depth
table for related topics?
</div>
