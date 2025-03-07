# 2D Renderers

{{#include 2d_renderers.incl.md}}

A 2D renderer is a software component that generates two-dimensional images by displaying graphics on a screen, typically using techniques like rasterization to create shapes, text, and images. It is commonly used in applications such as games, graphic design, and user interfaces.

Two essential components of 2D rendering are rasterization and anti-aliasing. Rasterization involves converting vector-based graphic images into raster images, which consist of grids of pixels. Anti-aliasing, on the other hand, is a technique used to smooth the edges of images to reduce jaggedness or pixelation.

## Choosing the Right Library

[`raqote`][c-raqote]⮳{{hi:raqote}} is a fast, pure Rust 2D graphics library. [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} is another good option. [`cairo-rs`][c-cairo]⮳{{hi:cairo-rs}} provides bindings to the Cairo library. |

Most 2D projects will find [`raqote`][c-raqote]⮳{{hi:raqote}} or [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} to be a good balance of features and performance. If you need very low-level control or are working with GPU-accelerated [graphics][p-graphics], [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is the way to go. If you're making a game, one of the [[game_engines | game]] frameworks will likely be the easiest option.

- Simple 2D, Direct Pixel Manipulation: [`pixels`][c-pixels]⮳{{hi:pixels}}.
- General-Purpose 2D, Canvas-Like API: [`raqote`][c-raqote]⮳{{hi:raqote}}.
- Lightweight, Performant 2D: [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}}.
- Vector Path Generation: [`lyon`][c-lyon]⮳{{hi:lyon}}.
- High Performance 2D/3D (using GPU): [`wgpu`][c-wgpu]⮳{{hi:wgpu}} (but more complex).

## Topics

- Sprite Rendering.
- Vector Graphics.
- Text Rendering.
- Canvas Drawing.

## Pixel-Based (Raster) 2D Rendering

See [[2d_raster_graphics | 2D Raster Graphics]].

## Vector-Based 2D Rendering

See [[2d_vector_graphics | 2D Vector Graphics]].

## Hybrid (Raster and Vector)

While primarily a low-level, cross-platform GPU API, [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is capable of rasterization and can be used to draw both raster and vector graphics. It's very powerful and performant but requires more setup and code compared to the higher-level libraries. You'd use [`lyon`][c-lyon]⮳{{hi:lyon}} to generate vector paths and then [`wgpu`][c-wgpu]⮳{{hi:wgpu}} to render them.

See [[gpu_abstraction_layers | Gpu Abstraction Layers]].

## `skia-safe` {#skia-safe}

[![skia-safe][c-skia_safe-badge]][c-skia_safe] [![skia-safe-crates.io][c-skia_safe-crates.io-badge]][c-skia_safe-crates.io] [![skia-safe-github][c-skia_safe-github-badge]][c-skia_safe-github] [![skia-safe-lib.rs][c-skia_safe-lib.rs-badge]][c-skia_safe-lib.rs]{{hi:skia-safe}}{{hi:OpenGL}}{{hi:Pdf}}{{hi:Rust-bindings}}{{hi:Skia}}{{hi:Vulkan}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}} [![cat-multimedia::images][cat-multimedia::images-badge]][cat-multimedia::images]{{hi:Images}} [![cat-rendering::graphics-api][cat-rendering::graphics-api-badge]][cat-rendering::graphics-api]{{hi:Graphics APIs}} [![cat-visualization][cat-visualization-badge]][cat-visualization]{{hi:Visualization}}

[`skia-safe`][c-skia_safe]⮳{{hi:skia-safe}} offers "Bindings to the Skia C++ library. The most complete option with excellent performance. However, it can be difficult to get it to compile." ([blessed.rs](https://blessed.rs/crates#section-graphics)).

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/skia_safe.rs:example}}
```

## `webrender` {#webrender}

[![webrender][c-webrender-badge]][c-webrender] [![webrender-crates.io][c-webrender-crates.io-badge]][c-webrender-crates.io] [![webrender-github][c-webrender-github-badge]][c-webrender-github] [![webrender-lib.rs][c-webrender-lib.rs-badge]][c-webrender-lib.rs]{{hi:webrender}}

[`webrender`][c-webrender]⮳{{hi:webrender}} is a GPU-accelerated 2D renderer for web content. "OpenGL-based. Mature with production usage in `Firefox` but documentation and OSS maintenance are lacking." ([blessed.rs](https://blessed.rs/crates#section-graphics)).

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/webrender.rs:example}}
```

## Other renderers {#skip1}

## `vello` {#vello}

[![vello][c-vello-badge]][c-vello] [![vello-crates.io][c-vello-crates.io-badge]][c-vello-crates.io] [![vello-github][c-vello-github-badge]][c-vello-github] [![vello-lib.rs][c-vello-lib.rs-badge]][c-vello-lib.rs]{{hi:vello}}{{hi:2d}}{{hi:Vector-graphics}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}} [![cat-rendering][cat-rendering-badge]][cat-rendering]{{hi:Rendering}}

[`vello`][c-vello]⮳{{hi:vello}} is a GPU compute-centric 2D renderer. "WGPU-based and uses cutting edge techniques to render vector paths using the GPU. Still somewhat immature and hasn't yet put out a stable release." ([blessed.rs](https://blessed.rs/crates#section-graphics)).

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/vello.rs:example}}
```

## Related Topics

- [[color_handling | Color Handling]].
- Image Loading and Manipulation: see [[multimedia_images | Multimedia: Images]].
- [[rendering_engine | Rendering Engine]].
- [[rendering_graphics-api | Rendering: Graphics API]].
- [[shaders | Shaders]].
- [[svg_rendering | SVG Rendering]].
- [[text_rendering | Text Rendering]].

### Applications

- [[game-development | Game Development]].
- [[game_engines | Game Engines]].
- UI with 2D [Graphics][p-graphics]: see [[gui | GUI]].

## See also

- [Are we Game yet? 2D Rendering](https://arewegameyet.rs/ecosystem/2drendering/): Sprites, vectors, splines, hex grids and more.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[2d_renderers: write; titles](https://github.com/john-cd/rust_howto/issues/377)
decide what to cover
what goes into 2D raster / vector / this page?
</div>
