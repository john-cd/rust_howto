# Graphics

[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

Crates for graphics libraries and applications, including raster and vector graphics primitives such as geometry, curves, and color.

For most 2D graphics projects, [`raqote`][c-raqote]⮳{{hi:raqote}}, [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}, or a combination of [`pixels`][c-pixels]⮳{{hi:pixels}} and [`lyon`][c-lyon]⮳{{hi:lyon}} will be a good starting point. For 3D or performance-critical 2D, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is the way to go.

If you're using a [[gui | GUI]] framework or [[game-engines | Game Engine]], it will handle a lot of the low-level graphics details for you.

## Rendering

- [[rendering | Rendering]].
- [[2d_renderers | 2D Renderers]].
  - [[2d_raster_graphics | 2D Raster Graphics]].
  - [[2d_vector_graphics | 2D Vector Graphics]].
- [[3d_renderers | 3d Renderers]].
- [[rendering_engine | Rendering Engine]].
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

- [[gpu | GPU]].

## Applications and Fields

- Image Procesing - see [[multimedia_images | Multimedia: Images]].
- Ray Tracing.
- [[computer-vision | Computer Vision]].
- Game Development - see [[games | Games]], [[game-development | Game Development]] and [[game-engines | Game Engines]].
  - Real-Time Rendering.
  - Character Animation.
- Computer-Aided Design (CAD).
  - 3D Modeling.
  - Visualization and Simulation.
- Visual Effects (VFX).
  - Compositing.
  - Particle Systems.
  - Rendering for Film and Television.
- User Interface (UI) Design - consult the [[gui | GUI]] pages.
  - 2D and 3D UI Elements.
  - GUI Rendering.
  - Typography - see [[text_rendering | Text Rendering]] and [[text_layout | Text Layout]].
- [[multimedia | Multimedia]].
  - [[multimedia_encoding | Multimedia: Encoding]].
  - [[multimedia_images | Multimedia: Images]].
  - [[multimedia_video | Multimedia: Video]].
- Scientific Visualization.
  - Data Visualization, Plotting and Charting - see [[visualization | Visualization]].
  - Volume Rendering of Scientific Data.
- Creative Coding and Generative Art.
  - Procedural Generation.
  - Interactive Visuals.

### Ray Tracing

Often requires specialized libraries or custom implementations using compute [[shaders | shaders]].

## Related Topics

- Mathematics / Linear Algebra: [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} and [`glam`][c-glam]⮳{{hi:glam}} are used for vector and matrix math. Also consider [`cgmath`][c-cgmath]⮳{{hi:cgmath}} and `euclid` for geometry. See [[mathematics | Mathematics]], [[linear_algebra | Linear Algebra]] and [[game_development | Game Development]].
- [[simulation | Simulation]] and [[physics_engines | Physics Engines]].

## See also

- [gfx-rs.github.io](https://gfx-rs.github.io).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / organize / review in depth
what applications do we want to cover?
cover `nannou`
</div>
