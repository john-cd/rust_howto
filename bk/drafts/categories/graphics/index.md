<!-- TODO Reviewed ??-??-2026 -->

# Graphics

[![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

Graphics libraries cover different layers of a rendering pipeline, from drawing 2D shapes to submitting GPU work. Choose by the output you need and how much control you want over the renderer, window, and event loop.

## Choose a Graphics Layer

| Goal                                       | Crates                                                                                    | What they provide                                                                  |
| ------------------------------------------ | ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Draw 2D shapes and paths                   | [`raqote`][c~raqote~docs]↗{{hi:raqote}}, [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} | 2D drawing APIs for paths, fills, strokes, and images.                             |
| Render a pixel buffer                      | [`pixels`][c~pixels~docs]↗{{hi:pixels}}                                                   | A GPU-backed pixel buffer built on [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}.              |
| Turn vector paths into renderable geometry | [`lyon`][c~lyon~docs]↗{{hi:lyon}}                                                         | Tessellates 2D fills and strokes into geometry; a renderer still draws the result. |
| Build a GPU renderer or 3D application     | [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}                                                         | A cross-platform graphics API with native and WebAssembly backends.                |
| Make sketches and generative artwork       | [`nannou`][c~nannou~repo]↗{{hi:nannou}}                                                   | A creative-coding framework with drawing, graphics, windowing, and UI APIs.        |

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

- [Rendering](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering/index.md).
  - [2D Renderers](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering/2d_renderers.md): [2D Raster Graphics](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering/2d_raster_graphics.md) and [2D Vector Graphics](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering/2d_vector_graphics.md).
  - [3D Renderers](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering/3d_renderers.md).
  - [Rendering Engine](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_engine/index.md).
  - [Rendering Data Formats](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_data-formats/index.md).
  - [SVG Rendering](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering/svg_rendering.md).
  - [Text Rendering](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering/text_rendering.md) and [Text Layout](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/gui/text_layout.md).
- Low-level graphics APIs: [Rendering: Graphics API](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_graphics-api/index.md).
  - [GPU Abstraction Layers](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_graphics-api/gpu_abstraction_layers.md).
  - [OpenGL](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_graphics-api/opengl.md).
  - [Vulkan](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_graphics-api/vulkan.md).
  - [Native Graphics APIs](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_graphics-api/native_graphics_apis.md).
  - [Shaders](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_graphics-api/shaders.md).
- [GPU](https://github.com/john-cd/rust_howto/blob/develop/later/src/other/gpu/index.md).
- [Multimedia](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/multimedia/index.md): [Encoding](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/multimedia_encoding/index.md), [Images](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/multimedia_images/index.md) and [Video](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/multimedia_video/index.md).
- [Visualization](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/visualization/index.md): data visualization, plotting and charting.
- [Games](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/games/index.md), [Game Development](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/game-development/index.md) and [Game Engines](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/game-engines/index.md).
- [Simulation](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/simulation/index.md) and [Physics Engines](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/simulation/physics_engines.md).
- [Computer Vision](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/computer-vision/index.md).
- [GUI](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/gui/index.md).

## Application Areas

- Ray tracing, often with specialized libraries or custom code using compute [shaders](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/rendering_graphics-api/shaders.md).
- Real-time rendering and character animation (games).
- Computer-aided design (CAD), 3D modeling, visualization and simulation.
- Visual effects: compositing, particle systems, film and television rendering.
- User interface rendering and typography.
- Scientific visualization, including volume rendering of scientific data.
- Creative coding, procedural generation and generative art.

## Related Topics

Graphics code relies on [linear algebra][p~linear-algebra] for vectors, matrices, and transforms. [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}} and [`glam`][c~glam~docs]↗{{hi:glam}} are widely used for vector and matrix math; also consider [`cgmath`][c~cgmath~docs]↗{{hi:cgmath}} and [`euclid`][c~euclid~docs]↗{{hi:euclid}} for geometry. See also [Mathematics](https://github.com/john-cd/rust_howto/blob/develop/bk/src/categories/mathematics/index.md) and [Linear Algebra](https://github.com/john-cd/rust_howto/blob/develop/bk/src/categories/mathematics/linear_algebra.md). Higher-level GUI and game frameworks can provide the windowing and rendering layers for complete applications.

Further reading: [gfx-rs.github.io][blog~gfx-rs]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
