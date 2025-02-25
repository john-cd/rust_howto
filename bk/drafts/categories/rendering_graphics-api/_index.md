# Graphics APIs

[![cat-rendering::graphics-api][cat-rendering::graphics-api-badge]][cat-rendering::graphics-api]{{hi:Rendering graphics api}}

Provide direct access to the hardware's or the operating system's rendering capabilities.

{{#include graphics_apis.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 review](https://github.com/john-cd/rust_howto/issues/956)

Direct hardware access is generally considered unsafe and requires careful consideration of memory safety and concurrency. Many crates provide safe abstractions *over* these unsafe operations.

| Topic/Area | Rust Crates (Examples) | Notes |
|---|---|---|
| Low-Level Graphics (Vulkan) | [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, `ash` | [`ash`][c-ash]⮳{{hi:ash}} is a lower-level, more direct binding to Vulkan. [`vulkano`][c-vulkano]⮳{{hi:vulkano}} provides a safer, higher-level abstraction. These are closer to the metal than many other options. |
| Low-Level Graphics (Direct3D 12 - Windows) | [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (with a D3D12 backend), `d3d12` (more raw bindings) | Direct3D 12 access is usually through [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}'s backend or lower level bindings like `d3d12`. Windows-specific. |
| Low-Level Graphics (Metal - macOS/iOS) | [`metal`][c-metal]⮳{{hi:metal}}, [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (with a Metal backend) | Metal access is provided by the [`metal`][c-metal]⮳{{hi:metal}} crate or through [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}. macOS/iOS specific. |
| Window Management & Input (Cross-Platform) | [`winit`][c-winit]⮳{{hi:winit}}, `raw-window-handle` | These crates are essential for interacting with the window system, which is a prerequisite for any rendering. `raw-window-handle` is used to get the window handle to pass to graphics APIs. [`winit`][c-winit]⮳{{hi:winit}} provides a higher-level abstraction for window creation and event handling. |
| Abstracted Graphics (Cross-Platform) | [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} | These provide a common API that can target multiple backends (Vulkan, D3D12, Metal, etc.). While not *directly* accessing hardware, they provide a lower level than game engines or higher-level frameworks. [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is focused on WebGPU compatibility. |
| Image Loading/Manipulation | [`image`][c-image]⮳{{hi:image}}, [`imageproc`][c-imageproc]⮳{{hi:imageproc}} | Essential for working with image data, often used in conjunction with graphics APIs. |
| Shader Compilation | [`shaderc`][c-shaderc]⮳{{hi:shaderc}}, [`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}} | Needed for compiling shaders (GLSL, HLSL, etc.) into formats like SPIR-V, which is used by Vulkan and other APIs. |
| Memory Management (GPU) | (Often handled by the graphics API wrappers like [`vulkano`][c-vulkano]⮳{{hi:vulkano}} or [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}) | Direct memory management is usually within the scope of the chosen graphics API. |

## Important Considerations

* Safety: Direct hardware access is inherently unsafe. Rust's ownership and borrowing system helps, but you must be extremely cautious. Crates like [`vulkano`][c-vulkano]⮳{{hi:vulkano}} provide safer abstractions.
* Portability: Direct hardware access is often platform-specific. Crates like [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} and [`wgpu`][c-wgpu]⮳{{hi:wgpu}} provide cross-platform abstractions, but they still rely on platform-specific backends.
* Complexity: Working directly with graphics APIs is complex. Consider using a higher-level engine or framework (e.g., Bevy, Amethyst, or even a game engine like Unreal Engine or Unity via Rust bindings) if your needs allow.
* Driver Dependence: Graphics drivers are essential. Issues with drivers can greatly impact rendering.

</div>
