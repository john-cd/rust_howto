# Rendering: Graphics APIs

[![cat~rendering::graphics-api][cat~rendering::graphics-api~badge]][cat~rendering::graphics-api]{{hi:Rendering graphics api}}

Provide direct access to the hardware's or the operating system's rendering capabilities.

## GPU Abstraction Layers

Libraries that provide a cross-platform abstraction over native graphics APIs.

| Topic/Area | Rust Crates (Examples) | Notes |
|---|---|---|
| Abstracted [[graphics | Graphics]] (Cross-Platform) | [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}} | These provide a common API that can target multiple backends (Vulkan, D3D12, Metal, etc.). They provide a lower level than game engines or higher-level frameworks. [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} is focused on WebGPU compatibility. |

{{#include gpu_abstraction_layers.incl.md}}

## Low-level Native Graphics APIs

| Topic/Area | Rust Crates (Examples) | Notes |
|---|---|---|
| Low-Level Graphics (Vulkan) | | |
| Low-Level Graphics (Direct3D 12 - Windows) | [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}} (with a D3D12 backend), [`d3d12`](https://crates.io/crates/d3d12)↗{{hi:d3d12}} (more raw bindings) | Direct3D 12 access is usually through [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}}'s backend or lower level bindings like [`d3d12`](https://crates.io/crates/d3d12)↗{{hi:d3d12}}. Windows-specific. |
| Low-Level Graphics (Metal - macOS/iOS) | [`metal`][c~metal~docs]↗{{hi:metal}}, [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}} (with a Metal backend) | Metal access is provided by the [`metal`][c~metal~docs]↗{{hi:metal}} crate or through [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}}. macOS/iOS specific. |

Note the following:

- Direct hardware access is often platform-specific. Crates like [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}} and [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} provide cross-platform abstractions, but they still rely on platform-specific backends.
- Working directly with low-level graphics APIs is complex. Consider using a higher-level [[gui | GUI]] framework or game engine (e.g., [`Bevy`][c~bevy~docs]↗{{hi:Bevy}}, [`Amethyst`][c~amethyst~docs]↗{{hi:Amethyst}}, or even [`Unreal Engine`](https://www.unrealengine.com)↗{{hi:Unreal Engine}} or [`Unity`](https://unity.com)↗{{hi:Unity}} via Rust bindings) if your needs allow.

{{#include native_graphics_apis.incl.md}}

{{#include vulkan.incl.md}}

## OpenGL

{{#include opengl.incl.md}}

## Shaders and GPU Compute

| Topic/Area | Rust Crates (Examples) | Notes |
|---|---|---|
| Shader Compilation | [`shaderc`][c~shaderc~docs]↗{{hi:shaderc}}, [`glsl-to-spirv`][c~glsl-to-spirv~docs]↗{{hi:glsl-to-spirv}} | Needed for compiling shaders (GLSL, HLSL, etc.) into formats like SPIR-V, which is used by Vulkan and other APIs. |

{{#include shaders.incl.md}}

## Related Topics

| Topic/Area | Rust Crates (Examples) | Notes |
|---|---|---|
| [[memory-management | Memory Management]] (GPU) | Often handled by the graphics API wrappers like [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} or [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}} | Direct memory management is usually within the scope of the chosen graphics API. |
| Window Management & Input (Cross-Platform) | [`winit`][c~winit~docs]↗{{hi:winit}}, `raw-window-handle` | These crates interact with the window system, which is a prerequisite for any rendering. `raw-window-handle` is used to get the window handle to pass to graphics APIs. [`winit`][c~winit~docs]↗{{hi:winit}} provides a higher-level abstraction for window creation and event handling. |
| [[multimedia_images | Image]] Loading/Manipulation | [`image`][c~image~docs]↗{{hi:image}}, [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}} | See also [[images | Images]]. |

- [[graphics | Graphics]].
- [[rendering | Rendering]].
- [[rendering_data-formats | Rendering: Data Formats]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/956)
</div>
