# Graphics

[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

Crates for graphics libraries and applications, including raster and vector graphics primitives such as geometry, curves, and color.

For most 2D graphics projects, [`raqote`][c-raqote]⮳{{hi:raqote}}, [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}, or a combination of [`pixels`][c-pixels]⮳{{hi:pixels}} and [`lyon`][c-lyon]⮳{{hi:lyon}} will be a good starting point. For 3D or performance-critical 2D, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is the way to go.

If you're using a [[gui | GUI]] framework or [[game-engines | Game Engine]], it will handle a lot of the low-level graphics details for you.

## Low-Level Graphics APIs

- Vulkan
  - `ash`
  - `erupt`
  - [`vulkano`][c-vulkano]⮳{{hi:vulkano}}: Rust bindings for the Vulkan API. Very powerful, but also very complex. Only use this if you absolutely n eed the fine-grained control that Vulkan provides.
- OpenGL
  - gl
  - glow
- DirectX
  - d3d12

## Cross-platform Graphics Frameworks

- [`wgpu`][c-wgpu]⮳{{hi:wgpu}}: A safe and modern graphics API implementing WebGPU (wgpu). While primarily a low-level, cross-platform GPU API, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is capable of rasterization and can be used to draw primitives. It doesn't provide high-level drawing functions directly, but it's the foundation upon which many higher-level graphics libraries are built. It's excellent for performance and control.

{{#include webgpu.incl.md}}

- `gfx-rs`: A low-level, cross-platform graphics abstraction
  - [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}: A lower-level graphics API abstraction layer. More control, but more complex. `wgpu` is often preferred for its ease of use.
  - [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}: Another low-level graphics API abstraction layer. Similar to [`wgpu`][c-wgpu]⮳{{hi:wgpu}} in that it can be used for drawing primitives, but requires more manual setup.

## 2D Raster Graphics (Pixel Manipulation)

Consider:

- [`pixels`][c-pixels]⮳{{hi:pixels}}: A crate for working with pixel buffers directly. Great for 2D raster graphics. You can draw points, lines, and other primitives by directly manipulating the pixel data. Good for simple 2D.
- [`raqote`][c-raqote]⮳{{hi:raqote}}: A fast 2D graphics library focused on rasterization. Provides a canvas-like API for drawing shapes, text, and images.
- [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}: A small, fast, and portable 2D graphics library that can render to bitmaps. Supports paths, gradients, and text. Good for when you need something lightweight.

### 2D Vector Graphics (Paths and Shapes)

[`lyon`][c-lyon]⮳{{hi:lyon}} generates vector paths and shapes. Often used with a rendering library. For basic geometry, you might just use structs and functions within your own code. [`lyon`][c-lyon]⮳{{hi:lyon}} can also generate various types of curves (Bézier curves, etc.).

It's useful for creating the geometry for vector graphics, but you'd typically use a rendering library like [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`raqote`][c-raqote]⮳{{hi:raqote}}, or [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} to actually draw them.

Also consider `femtovg`.

## 3D Graphics

For Rendering, consider: `rend3`, `three-rs`.

### Color Handling

The [`palette`][c-palette]⮳{{hi:palette}} crate is often used for working with colors in Rust.

### Texture Handling

Often handled by the rendering engine itself or directly using a low-level graphics API.

### Shaders

Shaders are usually written in a shading language (like GLSL or WGSL) and then loaded by your Rust code. `wgpu` uses WGSL.

Consider `naga`, `spirv-reflect`.

### Ray Tracing

Often requires specialized libraries or custom implementations using compute shaders.

### Rendering Engines (built on low-level APIs)

Review:

- [`rend3`][c-rend3]⮳{{hi:rend3}}: A 3D rendering engine built on top of `wgpu`. Provides higher-level abstractions for rendering.
- `bevy_render`: Bevy's rendering system (part of the Bevy game engine).

See [[rendering | Rendering]].

## Related Topics

### Compute Shaders (GPGPU)

Compute Shaders are often handled through the low-level graphics APIs like [`wgpu`][c-wgpu]⮳{{hi:wgpu}} or [`vulkano`][c-vulkano]⮳{{hi:vulkano}}.

### Game Engines

Consider:

- [`Bevy`][c-bevy]⮳{{hi:Bevy}}: A data-driven game engine that includes a rendering system.
- [`Amethyst`][c-amethyst]⮳{{hi:Amethyst}}: Another game engine with rendering capabilities.

General Purpose: `bevy`, `ggez`.
Lightweight: `miniquad`, `macroquad`.

See [[game-engines | Game Engines]].

### GUI Frameworks

Often Include 2D Graphics Capabilities. Consider:

- [`iced`][c-iced]⮳{{hi:iced}}: A cross-platform GUI library that uses a renderer (often [`wgpu`][c-wgpu]⮳{{hi:wgpu}} or [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}) to draw its UI elements. This means it can be used for basic 2D graphics as well.
- [`egui`][c-egui]⮳{{hi:egui}}: An immediate mode GUI library that can also be used for simple 2D drawing.

Consult the [[gui | GUI]] pages.

### Image Loading and Manipulation

[`image`][c-image]⮳{{hi:image}} is a widely used crate for loading and manipulating various image formats. See [[multimedia_images | Multimedia: Images]].

### Mathematics / Linear Algebra

[`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} and [`glam`][c-glam]⮳{{hi:glam}} are used for vector and matrix math, which are essential for graphics.

Also consider:

- `cgmath`
- `euclid` for geometry.

See [[mathematics | Mathematics]], [[linear_algebra | Linear Algebra]] and [[game_development | Game Development]].

### Text Rendering

`text-render` can be used for rendering text. Often used with 2D graphics libraries. See also [[text_layout | Text Layout]].

### Visualization, Plotting and Charting

While primarily a plotting library, [`plotters`][c-plotters]⮳{{hi:plotters}} can be used to draw basic 2D vector graphics primitives.
See [[visualization | Visualization]].

### Windowing and Input

Consider:

- [`winit`][c-winit]⮳{{hi:winit}}: A window creation and event handling library. Essential for getting a window on the screen and handling input.
- [`sdl2`][c-sdl2]⮳{{hi:sdl2}}: (Can also be used for windowing and input, but `winit` is often preferred in the Rust ecosystem).

See [[window_creation | Window Creation]].

### Image Formats

The [`image`][c-image]⮳{{hi:image}} crate loads and saves images in various formats.

## Physics Engines

Consider `rapier`, `nphysics`

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / review in depth
need cleanup of the taxonomy
table for related topics
distinguish between graphics and rendering categories
</div>
