<!-- TODO Reviewed ??-??-2026 -->

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

## See Also {#see-also}

- [[rendering_data-formats | Rendering Data Formats]]: 3D model and scene formats.
- [[rendering_engine | Rendering Engine]]: engines built on the APIs below.
- [[rendering_graphics-api | Rendering: Graphics API]]: OpenGL, Vulkan, `wgpu` and shaders.
- [[game_development | Game Development]], [[gui | GUI]], [[visualization | Visualization]] and [[multimedia_images | Multimedia: Images]] / [[multimedia_video | Multimedia: Video]].
- Windowing: [`winit`][c~winit~docs]↗{{hi:winit}}, the preferred cross-platform window and event library.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
