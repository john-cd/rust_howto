# Rendering: Graphics APIs

[![cat-rendering::graphics-api][cat-rendering::graphics-api-badge]][cat-rendering::graphics-api]{{hi:Rendering graphics api}}

Provide direct access to the hardware's or the operating system's rendering capabilities.

Low-Level Rendering Graphics APIs:

- Direct3D (DirectX)
- OpenGL
- Vulkan
- Metal
- WebGL
- WebGPU

Note the following:

- Direct hardware access is often platform-specific. Crates like [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} and [`wgpu`][c-wgpu]⮳{{hi:wgpu}} provide cross-platform abstractions, but they still rely on platform-specific backends.
- Working directly with low-level graphics APIs is complex. Consider using a higher-level [[gui | GUI]] framework or game engine (e.g., Bevy, Amethyst, or even Unreal Engine or Unity via Rust bindings) if your needs allow.

## Low-Level Graphics APIs

| Topic/Area | Rust Crates (Examples) | Notes |
|---|---|---|
| Low-Level Graphics (Vulkan) | [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, [`ash`][c-ash]⮳{{hi:ash}} | [`ash`][c-ash]⮳{{hi:ash}} is a lower-level, more direct binding to Vulkan. [`vulkano`][c-vulkano]⮳{{hi:vulkano}} provides a safer, higher-level abstraction. These are closer to the metal than many other options. |
| Low-Level Graphics (Direct3D 12 - Windows) | [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (with a D3D12 backend), `d3d12` (more raw bindings) | Direct3D 12 access is usually through [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}'s backend or lower level bindings like `d3d12`. Windows-specific. |
| Low-Level Graphics (Metal - macOS/iOS) | [`metal`][c-metal]⮳{{hi:metal}}, [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (with a Metal backend) | Metal access is provided by the [`metal`][c-metal]⮳{{hi:metal}} crate or through [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}. macOS/iOS specific. |
| Shader Compilation | [`shaderc`][c-shaderc]⮳{{hi:shaderc}}, [`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}} | Needed for compiling shaders (GLSL, HLSL, etc.) into formats like SPIR-V, which is used by Vulkan and other APIs. |

## Code Examples

{{#include graphics_apis.incl.md}}

## Vulkan

{{#include vulkan.incl.md}}

## Related Topics

| Topic/Area | Rust Crates (Examples) | Notes |
|---|---|---|
| Abstracted [[graphics | Graphics]] (Cross-Platform) | [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} | These provide a common API that can target multiple backends (Vulkan, D3D12, Metal, etc.). While not *directly* accessing hardware, they provide a lower level than game engines or higher-level frameworks. [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is focused on WebGPU compatibility. |
| Memory Management (GPU) | Often handled by the graphics API wrappers like [`vulkano`][c-vulkano]⮳{{hi:vulkano}} or [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} | Direct memory management is usually within the scope of the chosen graphics API. |
| Window Management & Input (Cross-Platform) | [`winit`][c-winit]⮳{{hi:winit}}, `raw-window-handle` | These crates interact with the window system, which is a prerequisite for any rendering. `raw-window-handle` is used to get the window handle to pass to graphics APIs. [`winit`][c-winit]⮳{{hi:winit}} provides a higher-level abstraction for window creation and event handling. |
| Image Loading/Manipulation | [`image`][c-image]⮳{{hi:image}}, [`imageproc`][c-imageproc]⮳{{hi:imageproc}} | See [[multimedia_images | Multimedia Images]] and [[images | Images]]. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/956)
</div>
