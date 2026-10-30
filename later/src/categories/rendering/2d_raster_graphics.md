<!-- TODO Reviewed ??-??-2026 -->

# Pixel-Based (Raster) 2D Rendering

{{#include 2d_raster_graphics.incl.md}}

## `pixels` {#pixels}

[`pixels`][c~pixels~docs]↗{{hi:pixels}} exposes a pixel buffer for direct manipulation when an application needs low-level control.

## `tiny-skia` {#tiny-skia}

[`tiny-skia`][c~tiny-skia~docs]↗ renders vector paths into an in-memory pixmap on the CPU. This example fills a circle, draws a diagonal stroke, and reads pixel colors without opening a window or using a GPU.

```rust,editable
{{#include ../../../crates/cats/rendering/examples/2d_raster_graphics/tiny_skia.rs:example}}
```

## `raqote` {#raqote}

[`raqote`][c~raqote~docs]↗{{hi:raqote}} is a fast 2D [graphics][p~graphics] library focused on rasterization. Provides a canvas-like API for drawing shapes, text, and images. A good choice for general-purpose 2D rendering.

## See Also {#see-also .skip}

- [Are we Game yet? 2D Rendering][are-we-game-yet?-2d-rendering~website]: Sprites, vectors, splines, hex grids and more

## Related Topics {#related-topics .skip}

- [[color_handling | Color Handling]].
- Image Loading and Manipulation: see [[multimedia_images | Multimedia: Images]].
- [[2d_vector_graphics | 2D Vector Graphics]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
