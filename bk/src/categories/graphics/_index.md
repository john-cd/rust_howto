# Graphics

[![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

Crates for graphics libraries and applications, including raster and vector graphics primitives such as geometry, curves, and color.

## WebGPU

{{#include webgpu.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

For modern graphics development in Rust, `wgpu` is a great starting point. If you're using a game engine, it will likely handle a lot of the low-level graphics details for you. `image` is essential for image handling. Linear algebra libraries like `nalgebra` or `glam` are crucial. And depending on your needs, you might need to explore UI libraries or other specialized crates.

## Key Concepts

Graphics pipeline: The sequence of steps involved in rendering an image.
Shaders: Programs that run on the GPU.
Textures: Images used in rendering.
Meshes: 3D models.
Transformations: How objects are moved, rotated, and scaled.
Lighting: How light interacts with objects.

Let's break down the graphics libraries and applications in Rust, focusing on raster and vector graphics primitives:

## Libraries (Crates)

- `wgpu`: While primarily a low-level, cross-platform GPU API, `wgpu` is capable of rasterization and can be used to draw primitives. It doesn't provide high-level drawing functions directly, but it's the foundation upon which many higher-level graphics libraries are built. It's excellent for performance and control.
- `gfx-hal`: Another low-level graphics API abstraction layer. Similar to `wgpu` in that it can be used for drawing primitives, but requires more manual setup.
- `pixels`: A crate for working with pixel buffers. Great for 2D raster graphics. You can draw points, lines, and other primitives by directly manipulating the pixel data.
- `raqote`: A fast 2D graphics library focused on rasterization. Provides a canvas-like API for drawing shapes, text, and images.
- `tiny-skia`: A small, fast, and portable 2D graphics library that can render to bitmaps. Supports paths, gradients, and text. Good for when you need something lightweight.
- `lyon`: A library for generating vector paths and shapes. It's useful for creating the geometry for vector graphics, but you'd typically use a rendering library like `wgpu`, `raqote`, or `tiny-skia` to actually draw them.
- `plotters`: While primarily a plotting library, `plotters` can be used to draw basic 2D vector graphics primitives.

- `iced`: A cross-platform GUI library that uses a renderer (often `wgpu` or `tiny-skia`) to draw its UI elements. This means it can be used for basic 2D graphics as well.

- `egui`: An immediate mode GUI library that can also be used for simple 2D drawing.

### Low-Level Graphics APIs

`pixels`: A crate for working with pixel buffers, often used for rendering graphics.
`sdl2`: Bindings to the SDL library, which can be used for window management, input, and graphics.

`wgpu`: A cross-platform, safe, and portable GPU API. It's a popular choice for modern graphics development in Rust.
`gfx-hal`: A lower-level graphics API abstraction layer. More control, but more complex. wgpu is often preferred for its ease of use.
`vulkano`: Rust bindings for the Vulkan API. Very powerful, but also very complex. Only use this if you absolutely need the fine-grained control that Vulkan provides.

### 2D Raster Graphics (Pixel Manipulation)

`pixels`: Working with pixel buffers directly. Good for simple 2D.
`raqote`: Fast 2D graphics library with a canvas-like API.
`tiny-skia`: Small, fast, portable 2D graphics library.

### 2D Vector Graphics (Paths and Shapes)

`lyon`: Generating vector paths and shapes. Often used with a rendering library.
Plotting and Charting:

`plotters`: Creating charts and graphs, but can also be used for basic 2D vector graphics.

### UI Libraries (Often Include 2D Graphics Capabilities)

`iced`: Cross-platform GUI library.
`egui`: Immediate mode GUI library.

### Image Loading and Manipulation

`image`: Loading and manipulating various image formats.

### Text Rendering

`text-render`: Rendering text. Often used with 2D graphics libraries.

### Color Handling

`palette`: Working with colors.

### Mathematics (Essential for Graphics)

`nalgebra`: Linear algebra library.
`glam`: Another linear algebra library.

## Key Concepts and Related Crates

- Geometry: `lyon` is dedicated to path and shape generation. For basic geometry, you might just use structs and functions within your own code. `nalgebra` and `glam` are used for vector and matrix math, which are essential for graphics.

- Curves: `lyon` can generate various types of curves (Bézier curves, etc.).

- Color: The `palette` crate is often used for working with colors in Rust.

- Image Formats: The `image` crate is crucial for loading and saving images in various formats.

- Text Rendering: Libraries like `text-render` can be used for text rendering, often in conjunction with a 2D graphics library.

## Choosing the Right Library

- High Performance, Low-Level Control: `wgpu`, `gfx-hal` (but more complex).
- 2D Raster Graphics: `pixels`, `raqote`, `tiny-skia`.
- Vector Graphics (Path Generation): `lyon`.
- Plotting: `plotters`.
- UI with 2D Graphics: `iced`, `egui`.

For most 2D graphics projects, `raqote`, `tiny-skia`, or a combination of `pixels` and `lyon` will be a good starting point. For 3D or performance-critical 2D, `wgpu` is the way to go. If you're using a game engine, it will likely handle the graphics for you.

## See also

### Rendering Engines (Built on Low-Level APIs)

`rend3`: A 3D rendering engine built on top of wgpu. Provides higher-level abstractions for rendering.
bevy_render: Bevy's rendering system (part of the Bevy game engine).
Windowing and Input:

`winit`: A window creation and event handling library. Essential for getting a window on the screen and handling input.
`sdl2`: (Can also be used for windowing and input, but winit is often preferred in the Rust ecosystem).

### Image Loading and Manipulation

`image`: A widely used crate for loading and manipulating various image formats.

### Texture Handling

Often handled by the rendering engine or directly using the low-level graphics API.

### Shaders

Shaders are usually written in a shading language (like GLSL or WGSL) and then loaded by your Rust code. wgpu uses WGSL.

### Mathematics (Linear Algebra)

`nalgebra`: A popular linear algebra library. Essential for graphics programming.
`glam`: Another linear algebra library, often used in game and graphics development.

### User Interface (UI)

`egui`: An immediate mode GUI library.
`iced`: A cross-platform GUI library focused on simplicity.

### 2D Graphics

`pixels`: A crate for working with pixel buffers, often used for 2D rendering.

### Compute Shaders (GPGPU)

Handled through the low-level graphics APIs like `wgpu` or `vulkano`.

### Ray Tracing

Often requires specialized libraries or custom implementations using compute shaders.

### Game Engines (High-Level)

`Bevy`: A data-driven game engine that includes a rendering system.
`Amethyst`: Another game engine with rendering capabilities.

</div>
