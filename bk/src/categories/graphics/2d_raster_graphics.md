<!-- TODO Reviewed ??-??-2026. -->

# Pixel-Based (Raster) 2D Graphics

A raster image stores a fixed grid of pixels. Drawing a path into a raster
surface samples its filled or stroked area onto that grid; the output size is
therefore part of the result. A CPU-backed pixmap is useful for generating
thumbnails, testing drawing logic, or preparing an image for another part of an
application without opening a window.

## Draw a CPU Raster Scene {#draw-a-cpu-raster-scene}

[`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} renders vector paths into an
in-memory pixmap on the CPU. This example fills a circle, draws a diagonal
stroke over it, and samples pixels to verify the result. It needs no display
server or GPU.

```rust,editable
{{#include ../../../crates/cats/graphics/examples/2d_raster_graphics/tiny_skia.rs:example}}
```

Run it to encode the pixmap as PNG bytes and inspect a sample pixel:

```console
cargo run --locked -p graphics --example 2d_raster_graphics
```

## Related Topics

- [Tessellate vector geometry before rasterizing it](2d_renderers.md).
- [Image Processing](../multimedia_images/images.md) covers image codecs and
  operations on raster buffers.

{{#include 2d_raster_graphics.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
