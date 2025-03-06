# Graphics

[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

Crates for graphics libraries and applications, including raster and vector graphics primitives such as geometry, curves, and color.

For most 2D graphics projects, [`raqote`][c-raqote]⮳{{hi:raqote}}, [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}, or a combination of [`pixels`][c-pixels]⮳{{hi:pixels}} and [`lyon`][c-lyon]⮳{{hi:lyon}} will be a good starting point. For 3D or performance-critical 2D, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is the way to go.

If you're using a [[gui | GUI]] framework or [[game-engines | Game Engine]], it will handle a lot of the low-level graphics details for you.

## Cross-platform Graphics Frameworks

See [[rendering | Rendering]] and [[rendering_engine | Rendering Engine]].

## 2D Raster Graphics (Pixel Manipulation)

See [[rendering | Rendering]].

### 2D Vector Graphics (Paths and Shapes)

See [[2d_vector_graphics | 2D Vector Graphics]].

## 3D Graphics

See [[webgpu | WebGPU]].

## Text Rendering

See [[text_rendering | Text Rendering]].

### Texture Handling

Often handled by the rendering engine itself or directly using a low-level graphics API.

## Low-Level Graphics APIs

See [[rendering_graphics-api | Rendering Graphics API]].

### Shaders

See [[shaders | Shaders]].

### Ray Tracing

Often requires specialized libraries or custom implementations using compute [[shaders | shaders]].

### Rendering Engines (built on low-level APIs)

See [[rendering_engine | Rendering Engines]].

## Related Topics

- [[color_handling | Color Handling]].
- [[game-engines | Game Engines]].
- GUI Frameworks - like [`iced`][c-iced]⮳{{hi:iced}} and [`egui`][c-egui]⮳{{hi:egui}}, often include 2D Graphics Capabilities. Consult the [[gui | GUI]] pages.
- Images (loading, saving, and manipulating) - see [[multimedia_images | Multimedia: Images]].
- Mathematics / Linear Algebra: [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} and [`glam`][c-glam]⮳{{hi:glam}} are used for vector and matrix math. Also consider `cgmath` and `euclid` for geometry. See [[mathematics | Mathematics]], [[linear_algebra | Linear Algebra]] and [[game_development | Game Development]].
- [[physics_engines | Physics Engines]].
- [[shaders | Shaders]] (GPU).
- Visualization, Plotting and Charting - see [[visualization | Visualization]].

## See also

- [gfx-rs.github.io](https://gfx-rs.github.io).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / organize / review in depth
need cleanup of the taxonomy
distinguish between graphics and rendering categories
</div>
