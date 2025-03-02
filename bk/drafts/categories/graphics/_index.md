# Graphics

[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

Crates for graphics libraries and applications, including raster and vector graphics primitives such as geometry, curves, and color.

For modern graphics development in Rust, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a great starting point. If you're using a game engine, it will likely handle a lot of the low-level graphics details for you. [`image`][c-image]⮳{{hi:image}} is essential for image handling. Linear algebra libraries like [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} or [`glam`][c-glam]⮳{{hi:glam}} are crucial. And depending on your needs, you might need to explore UI libraries or other specialized crates.

## Low-Level Graphics APIs

- [`pixels`][c-pixels]⮳{{hi:pixels}}: A crate for working with pixel buffers, often used for rendering graphics.
- [`sdl2`][c-sdl2]⮳{{hi:sdl2}}: Bindings to the SDL library, which can be used for window management, input, and graphics.
- [`wgpu`][c-wgpu]⮳{{hi:wgpu}}: A cross-platform, safe, and portable GPU API. It's a popular choice for modern graphics development in Rust.
- [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}: A lower-level graphics API abstraction layer. More control, but more complex. `wgpu` is often preferred for its ease of use.
- [`vulkano`][c-vulkano]⮳{{hi:vulkano}}: Rust bindings for the Vulkan API. Very powerful, but also very complex. Only use this if you absolutely need the fine-grained control that Vulkan provides.

### WebGPU

{{#include webgpu.incl.md}}

### 2D Raster Graphics (Pixel Manipulation)

[`pixels`][c-pixels]⮳{{hi:pixels}}: Working with pixel buffers directly. Good for simple 2D.
[`raqote`][c-raqote]⮳{{hi:raqote}}: Fast 2D graphics library with a canvas-like API.
[`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}: Small, fast, portable 2D graphics library.

## Related Topics

### Color Handling

- [`palette`][c-palette]⮳{{hi:palette}}: Working with colors.

### Compute Shaders (GPGPU)

Often handled through the low-level graphics APIs like [`wgpu`][c-wgpu]⮳{{hi:wgpu}} or [`vulkano`][c-vulkano]⮳{{hi:vulkano}}.

### Game Engines (High-Level)

[`Bevy`][c-bevy]⮳{{hi:Bevy}}: A data-driven game engine that includes a rendering system.
[`Amethyst`][c-amethyst]⮳{{hi:Amethyst}}: Another game engine with rendering capabilities.

### GUI Libraries (Often Include 2D Graphics Capabilities)

- [`iced`][c-iced]⮳{{hi:iced}}: A cross-platform GUI library that uses a renderer (often [`wgpu`][c-wgpu]⮳{{hi:wgpu}} or [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}) to draw its UI elements. This means it can be used for basic 2D graphics as well.
- [`egui`][c-egui]⮳{{hi:egui}}: An immediate mode GUI library that can also be used for simple 2D drawing.

### Image Loading and Manipulation

[`image`][c-image]⮳{{hi:image}}: A widely used crate for loading and manipulating various image formats.

### Mathematics / Linear Algebra

[`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}: A popular linear algebra library.
[`glam`][c-glam]⮳{{hi:glam}}: Another linear algebra library, often used in game and graphics development.

### Rendering Engines (built on low-level APIs)

- [`rend3`][c-rend3]⮳{{hi:rend3}}: A 3D rendering engine built on top of `wgpu`. Provides higher-level abstractions for rendering.
- bevy_render: Bevy's rendering system (part of the Bevy game engine).

### Text Rendering

- `text-render`: Rendering text. Often used with 2D graphics libraries.

### Visualization

While primarily a plotting library, [`plotters`][c-plotters]⮳{{hi:plotters}} can be used to draw basic 2D vector graphics primitives.

### Windowing and Input

[`winit`][c-winit]⮳{{hi:winit}}: A window creation and event handling library. Essential for getting a window on the screen and handling input.
[`sdl2`][c-sdl2]⮳{{hi:sdl2}}: (Can also be used for windowing and input, but `winit` is often preferred in the Rust ecosystem).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO review below

## Key Concepts

Graphics pipeline: The sequence of steps involved in rendering an image.
Shaders: Programs that run on the GPU.
Textures: Images used in rendering.
Meshes: 3D models.
Transformations: How objects are moved, rotated, and scaled.
Lighting: How light interacts with objects.

Let's break down the graphics libraries and applications in Rust, focusing on raster and vector graphics primitives:

## Libraries (Crates)

- [`wgpu`][c-wgpu]⮳{{hi:wgpu}}: While primarily a low-level, cross-platform GPU API, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is capable of rasterization and can be used to draw primitives. It doesn't provide high-level drawing functions directly, but it's the foundation upon which many higher-level graphics libraries are built. It's excellent for performance and control.
- [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}: Another low-level graphics API abstraction layer. Similar to [`wgpu`][c-wgpu]⮳{{hi:wgpu}} in that it can be used for drawing primitives, but requires more manual setup.
- [`pixels`][c-pixels]⮳{{hi:pixels}}: A crate for working with pixel buffers. Great for 2D raster graphics. You can draw points, lines, and other primitives by directly manipulating the pixel data.
- [`raqote`][c-raqote]⮳{{hi:raqote}}: A fast 2D graphics library focused on rasterization. Provides a canvas-like API for drawing shapes, text, and images.
- [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}: A small, fast, and portable 2D graphics library that can render to bitmaps. Supports paths, gradients, and text. Good for when you need something lightweight.
- [`lyon`][c-lyon]⮳{{hi:lyon}}: A library for generating vector paths and shapes. It's useful for creating the geometry for vector graphics, but you'd typically use a rendering library like [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`raqote`][c-raqote]⮳{{hi:raqote}}, or [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} to actually draw them.

### 2D Vector Graphics (Paths and Shapes)

[`lyon`][c-lyon]⮳{{hi:lyon}}: Generating vector paths and shapes. Often used with a rendering library.
Plotting and Charting:

[`plotters`][c-plotters]⮳{{hi:plotters}}: Creating charts and graphs, but can also be used for basic 2D vector graphics.

## Key Concepts and Related Crates

- Geometry: [`lyon`][c-lyon]⮳{{hi:lyon}} is dedicated to path and shape generation. For basic geometry, you might just use structs and functions within your own code. [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}} and [`glam`][c-glam]⮳{{hi:glam}} are used for vector and matrix math, which are essential for graphics.

- Curves: [`lyon`][c-lyon]⮳{{hi:lyon}} can generate various types of curves (Bézier curves, etc.).

- Color: The [`palette`][c-palette]⮳{{hi:palette}} crate is often used for working with colors in Rust.

- Image Formats: The [`image`][c-image]⮳{{hi:image}} crate is crucial for loading and saving images in various formats.

- Text Rendering: Libraries like `text-render` can be used for text rendering, often in conjunction with a 2D graphics library.

## Choosing the Right Library

- High Performance, Low-Level Control: [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} (but more complex).
- 2D Raster Graphics: [`pixels`][c-pixels]⮳{{hi:pixels}}, [`raqote`][c-raqote]⮳{{hi:raqote}}, [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}.
- Vector Graphics (Path Generation): [`lyon`][c-lyon]⮳{{hi:lyon}}.
- Plotting: [`plotters`][c-plotters]⮳{{hi:plotters}}.
- UI with 2D Graphics: [`iced`][c-iced]⮳{{hi:iced}}, [`egui`][c-egui]⮳{{hi:egui}}.

For most 2D graphics projects, [`raqote`][c-raqote]⮳{{hi:raqote}}, [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}, or a combination of [`pixels`][c-pixels]⮳{{hi:pixels}} and [`lyon`][c-lyon]⮳{{hi:lyon}} will be a good starting point. For 3D or performance-critical 2D, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is the way to go. If you're using a game engine, it will likely handle the graphics for you.

## See also

### Texture Handling

Often handled by the rendering engine or directly using the low-level graphics API.

### Shaders

Shaders are usually written in a shading language (like GLSL or WGSL) and then loaded by your Rust code. `wgpu` uses WGSL.

### 2D Graphics

[`pixels`][c-pixels]⮳{{hi:pixels}}: A crate for working with pixel buffers, often used for 2D rendering.

### Ray Tracing

Often requires specialized libraries or custom implementations using compute shaders.

</div>
