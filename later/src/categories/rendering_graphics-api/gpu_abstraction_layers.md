<!-- TODO Reviewed ??-??-2026 -->

# GPU Abstraction Layers

{{#include webgpu.incl.md}}

Libraries that provide a [cross-platform][p~cross-platform] abstraction over native graphics APIs.

For most Rust graphics applications, start with [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}. Its safe Rust API follows WebGPU and targets native backends as well as browsers, so one renderer can reach multiple platforms without application code selecting a different graphics API for each one. Choose a lower-level option when you need a different trade-off or are maintaining an existing integration.

## Key Concepts {#key-concepts .skip}

- [Shaders][p~shaders]: Programs that run on the GPU.
- Pipelines: The sequence of steps involved in rendering.
- Buffers: Memory on the GPU.
- Textures: [Images][p~images] used in rendering.
- Bind Groups: How [shaders][p~shaders] access resources.

## `wgpu` {#wgpu}

[![wgpu~website][c~wgpu~website~badge]][c~wgpu~website] [![wgpu][c~wgpu~docs~badge]][c~wgpu~docs] [![wgpu~crates.io][c~wgpu~crates.io~badge]][c~wgpu~crates.io] [![wgpu~repo][c~wgpu~repo~badge]][c~wgpu~repo] [![wgpu~lib.rs][c~wgpu~lib.rs~badge]][c~wgpu~lib.rs]{{hi:wgpu}}{{hi:Graphics}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

[`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} is a safe, portable Rust API based on the [WebGPU standard][webgpu~website]↗{{hi:WebGPU}}. On native targets it can use Vulkan, Metal, Direct3D 12, or OpenGL; in browsers, WebAssembly applications can use WebGPU or WebGL2. Backend availability depends on the target platform and device.

The same API covers graphics and compute workloads while leaving resource creation, pipelines, command submission, and shader dispatch explicit. This makes `wgpu` a practical default: it handles backend differences while preserving a consistent Rust interface, and it can target browser builds as well as native applications.

```rust,editable
{{#include ../../../crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs:example}}
```

## Blade {#blade}

[`Blade`][c~blade~repo]↗{{hi:Blade}} is a lean, low-level GPU abstraction by `kvark`, one of the authors of `wgpu`. Its graphics library supports Vulkan and GLES on Linux, Vulkan on Windows, Metal on Apple platforms, Vulkan on Android, and WebGL2 on the web. The full Blade Engine has narrower requirements: its ray-traced renderer needs Vulkan hardware ray-tracing support. Choose Blade when its lower-level design fits your renderer and target platforms.

## `gfx-hal` {#gfx-hal}

[`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}} is in maintenance mode; its development moved to the `wgpu-hal` abstraction used internally by `wgpu`. Treat `gfx-hal` as deprecated for new projects. Use `wgpu` by default, and use `gfx-hal` when maintaining existing code that depends on it.

### Related Topics {#related-topics .skip}

- For Rust shader authoring, see [Shaders][p~shaders]; for GPU programming and compute options, see [GPU programming][p~gpu].
- [[graphics | Graphics]].
- [[mathematics | Mathematics]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
