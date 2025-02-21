# Rendering

[![cat-rendering][cat-rendering-badge]][cat-rendering]{{hi:Rendering}}

Real-time or offline rendering of 2D or 3D graphics, usually with the help of a graphics card.

{{#include rendering.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 review](https://github.com/john-cd/rust_howto/issues/953)

## Rendering in Rust: Topics and Crates

| Topic | Rust Crates | Notes |
|---|---|---|
| 2D Graphics | `raqote`, `tiny-skia`, `cairo-rs`, `image`, `plotters` | `raqote` is a fast, pure Rust 2D graphics library. `tiny-skia` is another good option. `cairo-rs` provides bindings to the Cairo library. `image` is for image manipulation. `plotters` for data visualization. |
| 3D Graphics | `wgpu`, `bevy`, `rend3`, `glium`, `vulkano`, `nalgebra`, `cgmath` | `wgpu` is a cross-platform, low-level graphics API. `bevy` is a data-driven game engine. `rend3` is a rendering engine built on wgpu. `glium` is an OpenGL wrapper. `vulkano` is a Vulkan wrapper. `nalgebra` and `cgmath` provide linear algebra support. |
| Text Rendering | `fontdue`, `glyph_brush`, `text-render` | `fontdue` is a fast, pure Rust font loading and rasterization library. `glyph_brush` is for efficient text layout and caching. `text-render` is another text rendering option. |
| User Interfaces (UI) | `iced`, `egui`, `tauri`, `dioxus`, `slint`, `fltk-rs`, `winit` | `iced` is a cross-platform UI framework focused on simplicity. `egui` is an immediate mode GUI library. `tauri` is for building desktop applications with web technologies. `dioxus` is for building reactive UIs. `slint` (formerly sixtyfps) for embedded and desktop. `fltk-rs` is a binding to the FLTK toolkit. `winit` is for window creation. |
| Windowing/GLFW | `winit`, `glfw` | `winit` is a cross-platform window creation and event handling library. `glfw` provides bindings to the GLFW library. `winit` is generally preferred now. |
| Game Development | `bevy`, `amethyst`, `ggez`, `piston` | `bevy` is a data-driven game engine. `amethyst` is another game engine. `ggez` is a simple game framework. `piston` is a modular game engine library. |
| Data Visualization | `plotters`, `chartist-rs` | `plotters` is a plotting library. `chartist-rs` is another option. |
| Shader Development | `glsl-to-spirv`, `shaderc` | `glsl-to-spirv` compiles GLSL to SPIR-V. `shaderc` is another shader compiler. |
| Image Manipulation | `image`, `imageproc` | `image` is a general-purpose image processing library. `imageproc` provides more advanced image processing algorithms. |
| Font Loading | `fontdue`, `ttf-parser`, `opentype` | `fontdue` is a fast font rasterizer. `ttf-parser` and `opentype` provide lower-level font parsing capabilities. |
| SVG Rendering | `resvg`, `usvg` | `resvg` is an SVG rendering library. `usvg` is another option. |
| Hardware Acceleration | Often depends on the 3D API or rendering engine used | Libraries like `wgpu`, `vulkano`, and `glium` provide access to hardware acceleration through graphics APIs like Vulkan, Metal, and DirectX. |

</div>
