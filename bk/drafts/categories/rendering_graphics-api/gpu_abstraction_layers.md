# GPU Abstraction Layers

{{#include webgpu.incl.md}}

Libraries that provide a cross-platform abstraction over native graphics APIs.

## Key Concepts {#skip}

- Shaders: Programs that run on the GPU.
- Pipelines: The sequence of steps involved in rendering.
- Buffers: Memory on the GPU.
- Textures: [Images][p-images] used in rendering.
- Bind Groups: How shaders access resources.

## Key Rust Crates {#skip}

- [`wgpu-rs`][c-wgpu]⮳{{hi:wgpu-rs}}.
- `gfx-rs`: A low-level, cross-platform graphics abstraction (historically used, less active now).
  - [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}: A lower-level graphics API abstraction layer. More control, but more complex. Similar to [`wgpu`][c-wgpu]⮳{{hi:wgpu}} in that it can be used for drawing primitives, but requires more manual setup.

[`wgpu`][c-wgpu]⮳{{hi:wgpu}} is often preferred for its ease of use.

## WebGPU {#skip}

WebGPU is a JavaScript, C++, Rust, and C API that allows portably and efficiently utilizing a device's graphics processing unit (GPU). This is achieved with the underlying Vulkan, Metal, or Direct3D 12 system APIs.

WebGPU enables 3D graphics within an HTML canvas. It also has robust support for general-purpose GPU computations.

For WebGPU development in Rust, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is the essential crate. You'll use it to interact with the WebGPU API, manage resources, configure pipelines, and write shaders in WGSL.

## WebGPU Bindings with `wgpu` {#wgpu}

[![wgpu-website][c-wgpu-website-badge]][c-wgpu-website] [![wgpu][c-wgpu-badge]][c-wgpu] [![wgpu-crates.io][c-wgpu-crates.io-badge]][c-wgpu-crates.io] [![wgpu-github][c-wgpu-github-badge]][c-wgpu-github] [![wgpu-lib.rs][c-wgpu-lib.rs-badge]][c-wgpu-lib.rs]{{hi:wgpu}}{{hi:Graphics}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

[`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a safe and portable Rusty [WebGPU][wikipedia-webgpu]{{hi:WebGPU}}⮳ API wrapper. It is a cross-platform, safe, pure-Rust [graphics][p-graphics] API that exposes modern GPU capabilities. It runs natively on Vulkan, Metal, D3D12, and OpenGL; and on top of WebGL2 and WebGPU on WASM. The API is based on the [WebGPU standard][webgpu-website]⮳. It is suitable for general purpose graphics and compute on the GPU. It serves as the core of the WebGPU integration in Firefox and Deno.

- Shader Language: WGSL (WebGPU Shading Language) is the shader language used by WebGPU. You'll write your shaders in WGSL. You'll use it in your shader code (which is then loaded by [`wgpu`][c-wgpu]⮳{{hi:wgpu}}).
- Graphics Pipeline: You configure the [graphics][p-graphics] pipeline using the [`wgpu`][c-wgpu]⮳{{hi:wgpu}} API.
- Texture Handling: [`wgpu`][c-wgpu]⮳{{hi:wgpu}} provides ways to create and manage textures.
- While primarily a low-level, cross-platform GPU API, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is capable of rasterization and can be used to draw primitives. It doesn't provide high-level drawing functions directly, but it's the foundation upon which many higher-level graphics libraries are built. It's excellent for performance and control.
- Compute Shaders: You write compute shaders in WGSL and then use the [`wgpu`][c-wgpu]⮳{{hi:wgpu}} API to dispatch them.

- Debugging: WebGPU has built-in debugging capabilities that can be accessed through browser developer tools (if you're targeting the web) or through debugging tools specific to your platform (if you're targeting native).

```rust,editable
{{#include ../../../crates/cats/graphics/tests/wgpu.rs:example}}
```

### See also {#skip}

- `web-sys`: Web bindings, used for WebGPU in the browser.

## Related Topics {#skip}

- [[graphics | Graphics]].
- [[mathematics | Mathematics]].
- [[shaders | Shaders]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[webgpu: write](https://github.com/john-cd/rust_howto/issues/375)
review [rust-gpu-ecosystem-website]: https://rust-gpu.github.io/ecosystem
review in depth
</div>
