<!-- TODO Reviewed ??-??-2026 -->

# Native Graphics APIs (Bindings)

{{#include native_graphics_apis.incl.md}}

Low-Level [Rendering][p~rendering] Graphics APIs include:

- Direct3D (DirectX).
- Metal.
- OpenGL.
- Vulkan.
- WebGL and WebGPU.

## Identify the Native Backend with `wgpu` {#adapter-info}

[`wgpu`][c~wgpu~docs]↗ selects an adapter and exposes the underlying native graphics API through `get_info()`. This headless example reports the adapter name, backend (Vulkan, Metal, or Direct3D 12), and device type. It requests no device or window and returns an error if no compatible adapter is available.

From `later/crates`, run:

```sh
cargo run --locked -p rendering_graphics_api --example native_graphics_apis --features wgpu
```

```rust,editable
{{#include ../../../crates/cats/rendering_graphics_api/examples/native_graphics_apis/wgpu_adapter_info.rs:example}}
```

## Vulkan {#vulkan .skip}

[`ash`][c~ash~docs] provides low-level Vulkan bindings. Choose native Vulkan for Vulkan-specific extensions or direct control over synchronization and resource management. See [Vulkan](vulkan.md) for API setup and examples.

## OpenGL {#opengl .skip}

[`glow`][c~glow~docs] wraps OpenGL and OpenGL ES function loading and also supports WebGL. It is useful when integrating with an existing OpenGL renderer or context. See [OpenGL](opengl.md) for context creation and rendering.

## Low-Level Graphics (Direct3D 12 - Windows) {#d3d12}

The [`windows`][c~windows~docs] crate exposes Direct3D 12 through `Win32::Graphics::Direct3D12`; enable the corresponding Cargo features for the interfaces your application uses. Native Direct3D 12 is appropriate for Windows-specific integration and extensions. The [`d3d12`][c~d3d12~crates.io] crate offers another low-level binding. `gfx-hal` is unmaintained and should not be the starting point for a new application.

## Low-Level Graphics (Metal) {#metal}

[`objc2-metal`][c~objc2-metal~docs] provides generated bindings to Apple's Metal framework on macOS and iOS. The older `metal` crate is deprecated in its favor. Choose native Metal for Apple-only features or integration with an existing Metal application.

Use [`wgpu`][c~wgpu~docs] for a portable API unless these platform-specific requirements justify managing native resources and synchronization yourself. See [GPU Abstraction Layers](gpu_abstraction_layers.md) and [Shaders](shaders.md).

## Related Topics {#related-topics .skip}

- [[gpu | GPU]].
- [[gpu_abstraction_layers | GPU Abstraction Layers]].
- [[rendering | Rendering]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
