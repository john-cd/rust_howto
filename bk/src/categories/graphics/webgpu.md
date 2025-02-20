# WebGPU

{{#include webgpu.incl.md}}

## `wgpu` {#wgpu}

[![wgpu-website][c-wgpu-website-badge]][c-wgpu-website] [![wgpu][c-wgpu-badge]][c-wgpu] [![wgpu-crates.io][c-wgpu-crates.io-badge]][c-wgpu-crates.io] [![wgpu-github][c-wgpu-github-badge]][c-wgpu-github] [![wgpu-lib.rs][c-wgpu-lib.rs-badge]][c-wgpu-lib.rs]{{hi:wgpu}}{{hi:Graphics}}
[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

`wgpu` is a Rusty WebGPU API wrapper.

[wgpu][c-wgpu-github]⮳ is a cross-platform, safe, pure-Rust [graphics][p-graphics] API. It runs natively on Vulkan, Metal, D3D12, and OpenGL; and on top of WebGL2 and WebGPU on wasm. The API is based on the [WebGPU standard][webgpu-website]⮳. It serves as the core of the WebGPU integration in Firefox and Deno.

[WebGPU][wikipedia-webgpu]{{hi:WebGPU}}⮳

```rust,editable
{{#include ../../../crates/cats/graphics/tests/wgpu.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[webgpu: write (P2)](https://github.com/john-cd/rust_howto/issues/375)

- review [rust-gpu-ecosystem-website]: [https://rust-gpu.github.io/ecosystem](https://rust-gpu.github.io/ecosystem)

## Key Concepts

- WebGPU API: The API for interacting with the GPU.
- Shaders: Programs that run on the GPU.
- Pipelines: The sequence of steps involved in rendering.
- Buffers: Memory on the GPU.
- Textures: Images used in rendering.
- Bind Groups: How shaders access resources.

## WebGPU Bindings

- `wgpu`: This is the primary crate for WebGPU in Rust. It provides safe and portable bindings to the WebGPU API. It's the -most important- crate for WebGPU development in Rust.

## See also

## Windowing and Event Handling

- `winit`: A window creation and event handling library. Essential for getting a window on the screen and handling input. Often used with `wgpu`.

## Shader Language

- WGSL (WebGPU Shading Language): This is the shader language used by WebGPU. You'll write your shaders in WGSL. There aren't specific Rust crates for WGSL itself, but you'll use it in your shader code (which is then loaded by `wgpu`).

### Compute Shaders

Handled through `wgpu`. You write compute shaders in WGSL and then use the `wgpu` API to dispatch them.

### Graphics Pipeline

Managed through `wgpu`. You configure the graphics pipeline using the `wgpu` API.

### Texture Handling

Handled through `wgpu`. `wgpu` provides ways to create and manage textures.

### Mathematics (Linear Algebra)

- `nalgebra`: A popular linear algebra library. Essential for graphics programming.
- `glam`: Another linear algebra library, often used in game and graphics development.

### Debugging

- WebGPU has built-in debugging capabilities that can be accessed through browser developer tools (if you're targeting the web) or through debugging tools specific to your platform (if you're targeting native).

### Build Tools

`cargo`: The standard Rust build tool.

For WebGPU development in Rust, `wgpu` is the essential crate. You'll use it to interact with the WebGPU API, manage resources, configure pipelines, and write shaders in WGSL. `winit` is needed for window management. Linear algebra libraries are important for graphics calculations. Debugging is typically done through browser developer tools or platform-specific tools.

</div>
