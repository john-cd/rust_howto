# Pixel-Based (Raster) 2D Rendering

{{#include 2d_raster_graphics.incl.md}}

## `pixels` {#pixels}

[`pixels`][c-pixels]⮳{{hi:pixels}} is a crate for working with pixel buffers directly. Provides low-level access to pixel data, allowing you to draw primitives by manipulating the buffer. Good for simple 2D [graphics][p-graphics] or when you need fine-grained control.

```rust,editable
{{#include ../../../crates/cats/rendering/tests/render.rs:example}}
```

## `raqote` {#raqote}

[`raqote`][c-raqote]⮳{{hi:raqote}} is a fast 2D [graphics][p-graphics] library focused on rasterization. Provides a canvas-like API for drawing shapes, text, and images. A good choice for general-purpose 2D rendering.

## `tiny-skia` {#tiny-skia}

[`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} is a small, fast, and portable 2D [graphics][p-graphics] library that can render to bitmaps. Supports paths, gradients, and text. Excellent for when you need something lightweight and performant.

## Related Topics {#skip}

- [[color_handling | Color Handling]].
- Image Loading and Manipulation: see [[multimedia_images | Multimedia: Images]].
- [[2d_vector_graphics | 2D Vector Graphics]].

## See Also

[Are we Game yet? 2D Rendering][are we game yet? 2d rendering]: Sprites, vectors, splines, hex grids and more

[are we game yet? 2d rendering]: https://arewegameyet.rs/ecosystem/2drendering
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write. decide what to cover.](https://github.com/john-cd/rust_howto/issues/1215)
</div>
