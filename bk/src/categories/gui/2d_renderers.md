# 2D Renderers

{{#include 2d_renderers.incl.md}}

## `femtovg` {#femtovg}

[![femtovg][c-femtovg-badge]][c-femtovg] [![femtovg-crates.io][c-femtovg-crates.io-badge]][c-femtovg-crates.io] [![femtovg-github][c-femtovg-github-badge]][c-femtovg-github] [![femtovg-lib.rs][c-femtovg-lib.rs-badge]][c-femtovg-lib.rs]{{hi:femtovg}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

[`femtovg`][c-femtovg]⮳{{hi:femtovg}} is an anti-aliased 2D vector drawing library. It is "OpenGL based. Offers a simple API. Probably the easiest to get started with."

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/femtovg.rs:example}}
```

## `skia-safe` {#skia-safe}

[![skia-safe][c-skia_safe-badge]][c-skia_safe] [![skia-safe-crates.io][c-skia_safe-crates.io-badge]][c-skia_safe-crates.io] [![skia-safe-github][c-skia_safe-github-badge]][c-skia_safe-github] [![skia-safe-lib.rs][c-skia_safe-lib.rs-badge]][c-skia_safe-lib.rs]{{hi:skia-safe}}{{hi:OpenGL}}{{hi:Pdf}}{{hi:Rust-bindings}}{{hi:Skia}}{{hi:Vulkan}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}} [![cat-multimedia::images][cat-multimedia::images-badge]][cat-multimedia::images]{{hi:Images}} [![cat-rendering::graphics-api][cat-rendering::graphics-api-badge]][cat-rendering::graphics-api]{{hi:Graphics APIs}} [![cat-visualization][cat-visualization-badge]][cat-visualization]{{hi:Visualization}}

[`skia-safe`][c-skia_safe]⮳{{hi:skia-safe}} offers "Bindings to the Skia C++ library. The most complete option with excellent performance. However, it can be difficult to get it to compile."

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/skia_safe.rs:example}}
```

## `vello` {#vello}

[![vello][c-vello-badge]][c-vello] [![vello-crates.io][c-vello-crates.io-badge]][c-vello-crates.io] [![vello-github][c-vello-github-badge]][c-vello-github] [![vello-lib.rs][c-vello-lib.rs-badge]][c-vello-lib.rs]{{hi:vello}}{{hi:2d}}{{hi:Vector-graphics}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}} [![cat-rendering][cat-rendering-badge]][cat-rendering]{{hi:Rendering}}

[`vello`][c-vello]⮳{{hi:vello}} is a GPU compute-centric 2D renderer. "WGPU-based and uses cutting edge techniques to render vector paths using the GPU. Still somewhat immature and hasn't yet put out a stable release."

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/vello.rs:example}}
```

## `vger` {#vger}

[![vger][c-vger-badge]][c-vger] [![vger-crates.io][c-vger-crates.io-badge]][c-vger-crates.io] [![vger-github][c-vger-github-badge]][c-vger-github] [![vger-lib.rs][c-vger-lib.rs-badge]][c-vger-lib.rs]{{hi:vger}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

`vger` is a 2D [GPU][p-gpu] renderer for dynamic UIs.

"A simpler WGPU-based option which is less innovative but currently more stable than [`vello`][c-vello]⮳{{hi:vello}}."

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/vger.rs:example}}
```

## `webrender` {#webrender}

[![webrender][c-webrender-badge]][c-webrender] [![webrender-crates.io][c-webrender-crates.io-badge]][c-webrender-crates.io] [![webrender-github][c-webrender-github-badge]][c-webrender-github] [![webrender-lib.rs][c-webrender-lib.rs-badge]][c-webrender-lib.rs]{{hi:webrender}}

[`webrender`][c-webrender]⮳{{hi:webrender}} is a GPU accelerated 2D renderer for web content. "OpenGL-based. Mature with production usage in `Firefox` but documentation and OSS maintenance are lacking."

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/webrender.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[2d_renderers: write; titles (P2)](https://github.com/john-cd/rust_howto/issues/377)

## Pixel-Based (Raster) 2D Rendering

- `pixels`: A crate for working with pixel buffers directly. Provides low-level access to pixel data, allowing you to draw primitives by manipulating the buffer. Good for simple 2D graphics or when you need fine-grained control.
- `raqote`: A fast 2D graphics library focused on rasterization. Provides a canvas-like API for drawing shapes, text, and images. A good choice for general-purpose 2D rendering.
- `tiny-skia`: A small, fast, and portable 2D graphics library that can render to bitmaps. Supports paths, gradients, and text. Excellent for when you need something lightweight and performant.

## Vector-Based 2D Rendering

- `lyon`: A library for generating vector paths and shapes. It's useful for creating the -geometry- for vector graphics, but you'd typically use a rendering library like `wgpu`, `raqote`, or `tiny-skia` to actually -draw- them. `lyon` handles the math and path calculations.

## Hybrid (Raster and Vector)

- `wgpu`: While primarily a low-level, cross-platform GPU API, `wgpu` -is- capable of rasterization and can be used to draw both raster and vector graphics. It's very powerful and performant but requires more setup and code compared to the higher-level libraries. You'd use `lyon` to generate vector paths and then `wgpu` to render them.

## Higher-Level 2D Rendering (Often Part of UI or Game Frameworks)

- `iced`: A cross-platform GUI library that uses a renderer (often `wgpu` or `tiny-skia`) to draw its UI elements. This means it can be used for basic 2D graphics as well.
- `egui`: An immediate mode GUI library that can also be used for simple 2D drawing.
- `ggez`, [`macroquad`][c-macroquad]⮳{{hi:macroquad}} , `tetra`: 2D game frameworks that provide built-in rendering capabilities.

## Text Rendering

- `text-render`: A crate for rendering text. Often used with other 2D graphics libraries.

## Image Loading and Manipulation (Essential for 2D Graphics)

- `image`: A widely used crate for loading and manipulating various image formats.

## Color Handling

- `palette`: Working with colors.

## Choosing the Right Library

- Simple 2D, Direct Pixel Manipulation: `pixels`.
- General-Purpose 2D, Canvas-Like API: `raqote`.
- Lightweight, Performant 2D: `tiny-skia`.
- Vector Path Generation: `lyon`.
- High Performance 2D/3D (using GPU): `wgpu` (but more complex).
- UI with 2D Graphics: `iced`, `egui`.
- Game Development: `ggez`, [`macroquad`][c-macroquad]⮳{{hi:macroquad}} , `tetra`.

Most 2D projects will find `raqote` or `tiny-skia` to be a good balance of features and performance. If you need very low-level control or are working with GPU-accelerated graphics, `wgpu` is the way to go. If you're making a game, one of the game frameworks will likely be the easiest option.
</div>
