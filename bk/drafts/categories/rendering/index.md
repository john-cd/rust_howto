# Rendering

[![cat-rendering][cat-rendering-badge]][cat-rendering]{{hi:Rendering}}

Real-time or offline rendering of 2D or 3D graphics, usually with the help of a graphics card.

## Definitions

The terms "rendering" and [[graphics | "graphics"]] are related but refer to distinct aspects of visual representation.

Rendering is the process of converting the assets and abstract data defined within graphic design (models, geometry, shapes, textures, lighting, etc.) into a visual image or animation. It’s the "final step" where all the pieces come together to produce the image displayed on the screen, whether in real-time (like a video game) or offline (like a CGI scene in a movie).

Examples:

- A game frame being drawn on a screen.
- Generating a ray-traced image for realistic lighting.
- Converting 3D models into 2D images through shading and projection.

## Rendering in Rust: Topics and Crates

| [[graphics | Graphics]] | | [`wgpu`][c-wgpu]⮳{{hi:wgpu}}: A cross-platform, safe, and portable GPU API. Often used with `winit` or game engines. [`rend3`][c-rend3]⮳{{hi:rend3}}: A 3D rendering engine built on top of wgpu. [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}: A low-level graphics API abstraction layer. [`image`][c-image]⮳{{hi:image}} for image loading and manipulation. |

| Topic | Rust Crates | Notes |
|---|---|---|
| 2D Graphics | [`raqote`][c-raqote]⮳{{hi:raqote}}, [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}, [`cairo-rs`][c-cairo]⮳{{hi:cairo-rs}}, [`image`][c-image]⮳{{hi:image}}, [`plotters`][c-plotters]⮳{{hi:plotters}} | [`raqote`][c-raqote]⮳{{hi:raqote}} is a fast, pure Rust 2D graphics library. [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} is another good option. [`cairo-rs`][c-cairo]⮳{{hi:cairo-rs}} provides bindings to the Cairo library. [`image`][c-image]⮳{{hi:image}} is for image manipulation. [`plotters`][c-plotters]⮳{{hi:plotters}} for data visualization. |
| 3D Graphics | [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`bevy`][c-bevy]⮳{{hi:bevy}}, [`rend3`][c-rend3]⮳{{hi:rend3}}, [`glium`][c-glium]⮳{{hi:glium}}, [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}, [`cgmath`][c-cgmath]⮳{{hi:cgmath}} | [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a cross-platform, low-level graphics API. [`bevy`][c-bevy]⮳{{hi:bevy}} is a data-driven game engine. [`rend3`][c-rend3]⮳{{hi:rend3}} is a rendering engine built on wgpu. [`glium`][c-glium]⮳{{hi:glium}} is an OpenGL wrapper. [`vulkano`][c-vulkano]⮳{{hi:vulkano}} is a Vulkan wrapper. [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} and [`cgmath`][c-cgmath]⮳{{hi:cgmath}} provide linear algebra support. |
| Windowing/GLFW | [`winit`][c-winit]⮳{{hi:winit}}, [`glfw`][c-glfw]⮳{{hi:glfw}} | [`winit`][c-winit]⮳{{hi:winit}} is a cross-platform window creation and event handling library. [`glfw`][c-glfw]⮳{{hi:glfw}} provides bindings to the GLFW library. [`winit`][c-winit]⮳{{hi:winit}} is generally preferred now. |
| Shader Development | [`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}}, [`shaderc`][c-shaderc]⮳{{hi:shaderc}} | [`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}} compiles GLSL to SPIR-V. [`shaderc`][c-shaderc]⮳{{hi:shaderc}} is another shader compiler. |
| Hardware Acceleration | Often depends on the 3D API or rendering engine used | Libraries like [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, and [`glium`][c-glium]⮳{{hi:glium}} provide access to hardware acceleration through graphics APIs like Vulkan, Metal, and DirectX. |

## Cross-platform Graphics Frameworks

- [`wgpu`][c-wgpu]⮳{{hi:wgpu}}: A safe and modern graphics API implementing WebGPU (wgpu). While primarily a low-level, cross-platform GPU API, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is capable of rasterization and can be used to draw primitives. It doesn't provide high-level drawing functions directly, but it's the foundation upon which many higher-level graphics libraries are built. It's excellent for performance and control.

{{#include webgpu.incl.md}}

- `gfx-rs`: A low-level, cross-platform graphics abstraction
  - [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}: A lower-level graphics API abstraction layer. More control, but more complex. `wgpu` is often preferred for its ease of use. Similar to [`wgpu`][c-wgpu]⮳{{hi:wgpu}} in that it can be used for drawing primitives, but requires more manual setup.

## Font Loading, Text Rendering

{{#include text_rendering.incl.md}}

## SVG Rendering

{{#include svg_rendering.incl.md}}

## Related Topics

| Topic | Rust Crates | Notes |
|---|---|---|
| [[game_development | Game Development]] | [`bevy`][c-bevy]⮳{{hi:bevy}}, [`amethyst`][c-amethyst]⮳{{hi:amethyst}}, [`ggez`][c-ggez]⮳{{hi:ggez}}, [`piston`][c-piston]⮳{{hi:piston}} | [`bevy`][c-bevy]⮳{{hi:bevy}} is a data-driven game engine. [`amethyst`][c-amethyst]⮳{{hi:amethyst}} is another game engine. [`ggez`][c-ggez]⮳{{hi:ggez}} is a simple game framework. [`piston`][c-piston]⮳{{hi:piston}} is a modular game engine library. |
| Graphical User Interfaces ([[gui | GUI]]) | [`iced`][c-iced]⮳{{hi:iced}}, [`egui`][c-egui]⮳{{hi:egui}}, [`tauri`][c-tauri]⮳{{hi:tauri}}, [`dioxus`][c-dioxus]⮳{{hi:dioxus}}, [`slint`][c-slint]⮳{{hi:slint}}, [`fltk-rs`][c-fltk]⮳{{hi:fltk-rs}} | [`iced`][c-iced]⮳{{hi:iced}} is a cross-platform UI framework focused on simplicity. [`egui`][c-egui]⮳{{hi:egui}} is an immediate mode GUI library. [`tauri`][c-tauri]⮳{{hi:tauri}} is for building desktop applications with web technologies. [`dioxus`][c-dioxus]⮳{{hi:dioxus}} is for building reactive UIs. [`slint`][c-slint]⮳{{hi:slint}} (formerly sixtyfps) for embedded and desktop. [`fltk-rs`][c-fltk]⮳{{hi:fltk-rs}} is a binding to the FLTK toolkit. |
| [[multimedia_images | Image]] Manipulation | [`image`][c-image]⮳{{hi:image}}, [`imageproc`][c-imageproc]⮳{{hi:imageproc}} | [`image`][c-image]⮳{{hi:image}} is a general-purpose image processing library. [`imageproc`][c-imageproc]⮳{{hi:imageproc}} provides more advanced image processing algorithms. |
| [[visualization | Visualization]] | [`plotters`][c-plotters]⮳{{hi:plotters}} | [`plotters`][c-plotters]⮳{{hi:plotters}} is a plotting library. |

See also:

- [[graphics | Graphics]].
- [[gui | GUI]].
- [[multimedia_images | Multimedia: Images]].
- [[multimedia_video | Multimedia: Video]].
- [[rendering_data-formats | Rendering Data Formats]].
- [[rendering_engine | Rendering Engine]].
- [[rendering_graphics-api | Rendering: Graphics API]]
- [[visualization | Visualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/953)
ORGANIZE
move high level stuff in rendering engine
move low level stuff in graphics API
</div>
