<!-- TODO Reviewed ??-??-2026 -->

# Rendering: Graphics APIs

[![cat~rendering::graphics-api][cat~rendering::graphics-api~badge]][cat~rendering::graphics-api]{{hi:Rendering graphics api}}

Provide direct access to the hardware's or the operating system's rendering capabilities.

Portable APIs (WebGPU through [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}) run on every major platform and the web. Native APIs (Vulkan, OpenGL, Metal, DirectX 12) give finer control or access to platform-only features, at the cost of more code per platform. Start with `wgpu` unless you need something it does not expose.

## Choosing a Graphics API

| API        | Crate(s)                                                                                                 | Platforms                                | When to use                                                                             |
| ---------- | -------------------------------------------------------------------------------------------------------- | ---------------------------------------- | --------------------------------------------------------------------------------------- |
| WebGPU     | [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}                                                                        | Windows, macOS, Linux, iOS, Android, web | The default choice. One code base, backends for Vulkan, Metal, DX12, OpenGL and WebGPU. |
| Vulkan     | [`ash`][c~ash~docs]↗{{hi:ash}} (raw bindings), [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} (safe wrapper) | Windows, Linux, Android                  | Maximum control on Vulkan-capable platforms.                                            |
| OpenGL     | [`glow`][c~glow~docs]↗{{hi:glow}}, [`glium`][c~glium~docs]↗{{hi:glium}}                                  | Desktop, web (WebGL)                     | Legacy hardware and existing OpenGL code.                                               |
| Metal      | [`objc2-metal`][c~objc2-metal~docs]↗{{hi:objc2-metal}}                                                   | macOS, iOS                               | Apple-only features. The `metal` crate is deprecated.                                   |
| DirectX 12 | [`windows`][c~windows~docs]↗{{hi:windows}}                                                               | Windows                                  | Windows-only features.                                                                  |

[`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}} is no longer maintained; use `wgpu` instead.

## Pages

- [[native_graphics_apis | Native Graphics APIs]], [[vulkan | Vulkan]], [[opengl | OpenGL]], [[shaders | Shaders]], and [[gpu_abstraction_layers | GPU Abstraction Layers]].

{{#include gpu_abstraction_layers.incl.md}}

{{#include native_graphics_apis.incl.md}}

{{#include vulkan.incl.md}}

{{#include opengl.incl.md}}

{{#include shaders.incl.md}}

## Related Topics

| Topic                                                           | Rust Crates (Examples)                                                                 | Notes                                                                                          |
| --------------------------------------------------------------- | -------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| GPU memory management                                           | [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}}                                             | Memory management depends on the chosen graphics API.                                          |
| Window management and input                                     | [`winit`][c~winit~docs]↗{{hi:winit}}, `raw-window-handle`                              | Window handles connect a window system to graphics APIs; `winit` also provides event handling. |
| [Image loading and manipulation](../multimedia_images/index.md) | [`image`][c~image~docs]↗{{hi:image}}, [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}} | Load and process image data for rendering.                                                     |

- [[graphics | Graphics]].
- [[rendering | Rendering]].
- [[rendering_data-formats | Rendering: Data Formats]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
