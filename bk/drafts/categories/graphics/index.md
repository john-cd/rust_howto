<!-- TODO Reviewed ??-??-2026 -->

# Graphics

[![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

Graphics libraries cover different layers of a rendering pipeline, from drawing 2D shapes to submitting GPU work. Choose by the output you need and how much control you want over the renderer, window, and event loop.

## Choose a Graphics Layer

| Goal | Crates | What they provide |
|---|---|---|
| Draw 2D shapes and paths | [`raqote`][c~raqote~docs]↗{{hi:raqote}}, [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} | 2D drawing APIs for paths, fills, strokes, and images. |
| Render a pixel buffer | [`pixels`][c~pixels~docs]↗{{hi:pixels}} | A GPU-backed pixel buffer built on [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}. |
| Turn vector paths into renderable geometry | [`lyon`][c~lyon~docs]↗{{hi:lyon}} | Tessellates 2D fills and strokes into geometry; a renderer still draws the result. |
| Build a GPU renderer or 3D application | [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} | A cross-platform graphics API with native and WebAssembly backends. |
| Make sketches and generative artwork | [`nannou`][c~nannou~repo]↗{{hi:nannou}} | A creative-coding framework with drawing, graphics, windowing, and UI APIs. |

These tools are not interchangeable. For example, `lyon` prepares path geometry while a renderer such as `wgpu` submits it to a graphics backend. `pixels` starts from a pixel buffer, while 2D drawing APIs expose higher-level path and paint operations. A GUI framework or game engine may provide window management and rendering as part of a larger application framework.

## Rendering Topics

Graphics work commonly involves:

- Raster drawing, pixel buffers, and image surfaces.
- Vector paths, fills, strokes, SVG, and text rendering.
- 3D meshes, materials, real-time rendering, and scene organization.
- GPU APIs, shaders, and graphics data formats.
- Visualization, creative coding, and generative art.

Image processing, computer vision, simulation, and physics use some of the same image, geometry, and GPU concepts, but are separate application areas.

## Subtopics

- [[rendering | Rendering]].
  - [[2d_renderers | 2D Renderers]]: [[2d_raster_graphics | 2D Raster Graphics]] and [[2d_vector_graphics | 2D Vector Graphics]].
  - [[3d_renderers | 3D Renderers]].
  - [[rendering_engine | Rendering Engine]].
  - [[rendering_data-formats | Rendering Data Formats]].
  - [[svg_rendering | SVG Rendering]].
  - [[text_rendering | Text Rendering]] and [[text_layout | Text Layout]].
- Low-level graphics APIs: [[rendering_graphics-api | Rendering: Graphics API]].
  - [[gpu_abstraction_layers | GPU Abstraction Layers]].
  - [[opengl | OpenGL]].
  - [[vulkan | Vulkan]].
  - [[native_graphics_apis | Native Graphics APIs]].
  - [[shaders | Shaders]].
- [[gpu | GPU]].
- [[multimedia | Multimedia]]: [[multimedia_encoding | Encoding]], [[multimedia_images | Images]] and [[multimedia_video | Video]].
- [[visualization | Visualization]]: data visualization, plotting and charting.
- [[games | Games]], [[game-development | Game Development]] and [[game-engines | Game Engines]].
- [[simulation | Simulation]] and [[physics_engines | Physics Engines]].
- [[computer-vision | Computer Vision]].
- [[gui | GUI]].

## Application Areas

- Ray tracing, often with specialized libraries or custom code using compute [[shaders | shaders]].
- Real-time rendering and character animation (games).
- Computer-aided design (CAD), 3D modeling, visualization and simulation.
- Visual effects: compositing, particle systems, film and television rendering.
- User interface rendering and typography.
- Scientific visualization, including volume rendering of scientific data.
- Creative coding, procedural generation and generative art.

## Related Topics

Graphics code relies on [linear algebra][p~linear-algebra] for vectors, matrices, and transforms. [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} and [`glam`][c~glam~docs]↗{{hi:glam}} are widely used for vector and matrix math; also consider [`cgmath`][c~cgmath~docs]↗{{hi:cgmath}} and [`euclid`][c~euclid~docs]↗{{hi:euclid}} for geometry. See also [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]]. Higher-level GUI and game frameworks can provide the windowing and rendering layers for complete applications.

Further reading: [gfx-rs.github.io][blog~gfx-rs]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
