# Rendering

[![cat-rendering][cat-rendering-badge]][cat-rendering]{{hi:Rendering}}

Real-time or offline rendering of 2D or 3D graphics, usually with the help of a graphics card.

{{#include rendering.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/953)

## Rendering in Rust: Topics and Crates

| Topic | Rust Crates | Notes |
|---|---|---|
| 2D Graphics | [`raqote`][c-raqote]⮳{{hi:raqote}}, [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}, [`cairo-rs`][c-cairo]⮳{{hi:cairo-rs}}, [`image`][c-image]⮳{{hi:image}}, [`plotters`][c-plotters]⮳{{hi:plotters}} | [`raqote`][c-raqote]⮳{{hi:raqote}} is a fast, pure Rust 2D graphics library. [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} is another good option. [`cairo-rs`][c-cairo]⮳{{hi:cairo-rs}} provides bindings to the Cairo library. [`image`][c-image]⮳{{hi:image}} is for image manipulation. [`plotters`][c-plotters]⮳{{hi:plotters}} for data visualization. |
| 3D Graphics | [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`bevy`][c-bevy]⮳{{hi:bevy}}, [`rend3`][c-rend3]⮳{{hi:rend3}}, [`glium`][c-glium]⮳{{hi:glium}}, [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}, [`cgmath`][c-cgmath]⮳{{hi:cgmath}} | [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a cross-platform, low-level graphics API. [`bevy`][c-bevy]⮳{{hi:bevy}} is a data-driven game engine. [`rend3`][c-rend3]⮳{{hi:rend3}} is a rendering engine built on wgpu. [`glium`][c-glium]⮳{{hi:glium}} is an OpenGL wrapper. [`vulkano`][c-vulkano]⮳{{hi:vulkano}} is a Vulkan wrapper. [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} and [`cgmath`][c-cgmath]⮳{{hi:cgmath}} provide linear algebra support. |
| Text Rendering | [`fontdue`][c-fontdue]⮳{{hi:fontdue}}, [`glyph_brush`][c-glyph_brush]⮳{{hi:glyph_brush}}, `text-render` | [`fontdue`][c-fontdue]⮳{{hi:fontdue}} is a fast, pure Rust font loading and rasterization library. [`glyph_brush`][c-glyph_brush]⮳{{hi:glyph_brush}} is for efficient text layout and caching. `text-render` is another text rendering option. |
| User Interfaces (UI) | [`iced`][c-iced]⮳{{hi:iced}}, [`egui`][c-egui]⮳{{hi:egui}}, [`tauri`][c-tauri]⮳{{hi:tauri}}, [`dioxus`][c-dioxus]⮳{{hi:dioxus}}, [`slint`][c-slint]⮳{{hi:slint}}, [`fltk-rs`][c-fltk]⮳{{hi:fltk-rs}}, [`winit`][c-winit]⮳{{hi:winit}} | [`iced`][c-iced]⮳{{hi:iced}} is a cross-platform UI framework focused on simplicity. [`egui`][c-egui]⮳{{hi:egui}} is an immediate mode GUI library. [`tauri`][c-tauri]⮳{{hi:tauri}} is for building desktop applications with web technologies. [`dioxus`][c-dioxus]⮳{{hi:dioxus}} is for building reactive UIs. [`slint`][c-slint]⮳{{hi:slint}} (formerly sixtyfps) for embedded and desktop. [`fltk-rs`][c-fltk]⮳{{hi:fltk-rs}} is a binding to the FLTK toolkit. [`winit`][c-winit]⮳{{hi:winit}} is for window creation. |
| Windowing/GLFW | [`winit`][c-winit]⮳{{hi:winit}}, [`glfw`][c-glfw]⮳{{hi:glfw}} | [`winit`][c-winit]⮳{{hi:winit}} is a cross-platform window creation and event handling library. [`glfw`][c-glfw]⮳{{hi:glfw}} provides bindings to the GLFW library. [`winit`][c-winit]⮳{{hi:winit}} is generally preferred now. |
| Game Development | [`bevy`][c-bevy]⮳{{hi:bevy}}, [`amethyst`][c-amethyst]⮳{{hi:amethyst}}, [`ggez`][c-ggez]⮳{{hi:ggez}}, [`piston`][c-piston]⮳{{hi:piston}} | [`bevy`][c-bevy]⮳{{hi:bevy}} is a data-driven game engine. [`amethyst`][c-amethyst]⮳{{hi:amethyst}} is another game engine. [`ggez`][c-ggez]⮳{{hi:ggez}} is a simple game framework. [`piston`][c-piston]⮳{{hi:piston}} is a modular game engine library. |
| Data Visualization | [`plotters`][c-plotters]⮳{{hi:plotters}}, `chartist-rs` | [`plotters`][c-plotters]⮳{{hi:plotters}} is a plotting library. `chartist-rs` is another option. |
| Shader Development | [`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}}, [`shaderc`][c-shaderc]⮳{{hi:shaderc}} | [`glsl-to-spirv`][c-glsl_to_spirv]⮳{{hi:glsl-to-spirv}} compiles GLSL to SPIR-V. [`shaderc`][c-shaderc]⮳{{hi:shaderc}} is another shader compiler. |
| Image Manipulation | [`image`][c-image]⮳{{hi:image}}, [`imageproc`][c-imageproc]⮳{{hi:imageproc}} | [`image`][c-image]⮳{{hi:image}} is a general-purpose image processing library. [`imageproc`][c-imageproc]⮳{{hi:imageproc}} provides more advanced image processing algorithms. |
| Font Loading | [`fontdue`][c-fontdue]⮳{{hi:fontdue}}, [`ttf-parser`][c-ttf_parser]⮳{{hi:ttf-parser}}, [`opentype`][c-opentype]⮳{{hi:opentype}} | [`fontdue`][c-fontdue]⮳{{hi:fontdue}} is a fast font rasterizer. [`ttf-parser`][c-ttf_parser]⮳{{hi:ttf-parser}} and [`opentype`][c-opentype]⮳{{hi:opentype}} provide lower-level font parsing capabilities. |
| SVG Rendering | [`resvg`][c-resvg]⮳{{hi:resvg}}, [`usvg`][c-usvg]⮳{{hi:usvg}} | [`resvg`][c-resvg]⮳{{hi:resvg}} is an SVG rendering library. [`usvg`][c-usvg]⮳{{hi:usvg}} is another option. |
| Hardware Acceleration | Often depends on the 3D API or rendering engine used | Libraries like [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`vulkano`][c-vulkano]⮳{{hi:vulkano}}, and [`glium`][c-glium]⮳{{hi:glium}} provide access to hardware acceleration through graphics APIs like Vulkan, Metal, and DirectX. |

</div>
