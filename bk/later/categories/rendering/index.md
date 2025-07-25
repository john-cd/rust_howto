# Rendering

[![cat~rendering][cat~rendering~badge]][cat~rendering]{{hi:Rendering}}

Real-time or offline rendering of 2D or 3D graphics, usually with the help of a graphics card.

## Definitions

The terms "rendering" and [[graphics | "graphics"]] are related but refer to distinct aspects of visual representation.

Rendering is the process of converting the assets and abstract data defined within graphic design (models, geometry, shapes, textures, lighting, etc.) into a visual image or animation. It's the "final step" where all the pieces come together to produce the image displayed on the screen, whether in real-time (like a video game) or offline (like a CGI scene in a movie).

Examples:

- A game frame being drawn on a screen.
- Generating a ray-traced image for realistic lighting.
- Converting 3D models into 2D images through shading and projection.

## Topics

- [[rendering_engine | Rendering Engine]].
- [[2d_renderers | 2D Renderers]].
  - [[2d_raster_graphics | 2D Raster Graphics]].
  - [[2d_vector_graphics | 2D Vector Graphics]].
- [[3d_renderers | 3d Renderers]].
- [[rendering_data-formats | Rendering Data Formats]].
- [[svg_rendering | SVG Rendering]].
- [[text_rendering | Text Rendering]].
- Low-level Graphics API: [[rendering_graphics-api | Rendering: Graphics API]].
  - [[gpu_abstraction_layers | Gpu Abstraction Layers]].
  - [[opengl | OpenGL]].
  - [[native_graphics_apis | Native Graphics APIs]].
  - [[vulkan | Vulkan]].
  - [[shaders | Shaders]].

See also:

- [[graphics | Graphics]].
- [[gpu | GPU]].

## Code Examples

## 2D Raster Graphics

{{#include 2d_raster_graphics.incl.md}}

## 2D Vector Graphics

{{#include 2d_vector_graphics.incl.md}}

## 3D Renderers

{{#include 3d_renderers.incl.md}}

## SVG Rendering

{{#include svg_rendering.incl.md}}

## Font Loading & Text Rendering

{{#include text_rendering.incl.md}}

## Related Topics and Use Cases

| Topic | Rust Crates | Notes |
|---|---|---|
| [[game_development | Game Development]] | [`bevy`][c~bevy~docs]⮳{{hi:bevy}}, [`amethyst`][c~amethyst~docs]⮳{{hi:amethyst}}, [`ggez`][c~ggez~docs]⮳{{hi:ggez}}, [`piston`][c~piston~docs]⮳{{hi:piston}} | [`bevy`][c~bevy~docs]⮳{{hi:bevy}} is a data-driven game engine. [`amethyst`][c~amethyst~docs]⮳{{hi:amethyst}} is another game engine. [`ggez`][c~ggez~docs]⮳{{hi:ggez}} is a simple game framework. [`piston`][c~piston~docs]⮳{{hi:piston}} is a modular game engine library. |
| Graphical User Interfaces ([[gui | GUI]]) | [`iced`][c~iced~docs]⮳{{hi:iced}}, [`egui`][c~egui~docs]⮳{{hi:egui}}, [`tauri`][c~tauri~docs]⮳{{hi:tauri}}, [`dioxus`][c~dioxus~docs]⮳{{hi:dioxus}}, [`slint`][c~slint~docs]⮳{{hi:slint}}, [`fltk-rs`][c~fltk~docs]⮳{{hi:fltk-rs}} | [`iced`][c~iced~docs]⮳{{hi:iced}} is a cross-platform UI framework focused on simplicity. [`egui`][c~egui~docs]⮳{{hi:egui}} is an immediate mode GUI library. [`tauri`][c~tauri~docs]⮳{{hi:tauri}} is for building desktop applications with web technologies. [`dioxus`][c~dioxus~docs]⮳{{hi:dioxus}} is for building reactive UIs. [`slint`][c~slint~docs]⮳{{hi:slint}} (formerly sixtyfps) for embedded and desktop. [`fltk-rs`][c~fltk~docs]⮳{{hi:fltk-rs}} is a binding to the FLTK toolkit. |
| [[multimedia_images | Image]] Manipulation | [`image`][c~image~docs]⮳{{hi:image}}, [`imageproc`][c~imageproc~docs]⮳{{hi:imageproc}} | [`image`][c~image~docs]⮳{{hi:image}} is a general-purpose image processing library. [`imageproc`][c~imageproc~docs]⮳{{hi:imageproc}} provides more advanced image processing algorithms. |
| [[multimedia_video | Multimedia: Video]] | | |
| [[visualization | Visualization]] | [`plotters`][c~plotters~docs]⮳{{hi:plotters}} | [`plotters`][c~plotters~docs]⮳{{hi:plotters}} is a plotting library. |
| Windowing | [`winit`][c~winit~docs]⮳{{hi:winit}}, [`glfw`][c~glfw~docs]⮳{{hi:glfw}} | [`winit`][c~winit~docs]⮳{{hi:winit}} is a cross-platform window creation and event handling library. [`glfw`][c~glfw~docs]⮳{{hi:glfw}} provides bindings to the GLFW library. [`winit`][c~winit~docs]⮳{{hi:winit}} is generally preferred now. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/953)
</div>
