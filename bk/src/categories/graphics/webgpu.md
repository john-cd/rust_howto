# WebGPU

{{#include webgpu.incl.md}}

## `wgpu` {#wgpu}

[![wgpu-website][c-wgpu-website-badge]][c-wgpu-website] [![wgpu][c-wgpu-badge]][c-wgpu] [![wgpu-crates.io][c-wgpu-crates.io-badge]][c-wgpu-crates.io] [![wgpu-github][c-wgpu-github-badge]][c-wgpu-github] [![wgpu-lib.rs][c-wgpu-lib.rs-badge]][c-wgpu-lib.rs]{{hi:wgpu}}{{hi:Graphics}}
[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

`wgpu` is a Rusty WebGPU API wrapper.

[wgpu][c-wgpu-github]⮳ is a cross-platform, safe, pure-Rust graphics API. It runs natively on Vulkan, Metal, D3D12, and OpenGL; and on top of WebGL2 and WebGPU on wasm. The API is based on the [WebGPU standard][webgpu-website]⮳. It serves as the core of the WebGPU integration in Firefox and Deno.

[WebGPU][wikipedia-webgpu]{{hi:WebGPU}}⮳

```rust,editable
{{#include ../../../crates/cats/graphics/tests/wgpu.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[webgpu: write (P2)](https://github.com/john-cd/rust_howto/issues/375)

- review [rust-gpu-ecosystem-website]: [https://rust-gpu.github.io/ecosystem](https://rust-gpu.github.io/ecosystem)

</div>
