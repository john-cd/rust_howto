<!-- TODO Reviewed ??-??-2026. -->

# 2D Renderers

A 2D renderer turns paths, shapes, or text into pixels. A useful way to reason
about a rendering pipeline is to separate the description of a shape from the
steps that create geometry and paint pixels:

1. Build a path in a local coordinate system.
2. Tessellate the path into triangles when the renderer needs explicit
   geometry.
3. Rasterize the geometry into a pixel buffer.
4. Present or encode that buffer for the application.

Some libraries combine these steps. [`lyon`][c~lyon~docs]↗{{hi:lyon}} creates
geometry but does not draw it; [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}}
can paint paths or triangles into an in-memory pixmap. Separating these
boundaries makes it possible to change the renderer without changing how a
path is constructed.

## Tessellate, Then Rasterize {#tessellate-and-rasterize}

This example sends a triangle path through both stages. Lyon produces vertices
and indices; the application passes those triangles to tiny-skia's CPU
rasterizer. No window or GPU is needed.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p graphics --example 2d_renderers`.

```rust,editable,noplayground
{{#include ../../../crates/cats/graphics/examples/2d_renderers/tessellate_and_rasterize.rs:example}}
```

Run the example and inspect the number of triangles and painted pixels:

```console
cargo run --locked -p graphics --example 2d_renderers
```

## Related Topics

- [Draw into a raster pixmap](2d_raster_graphics.md).
- [Build and tessellate vector paths](2d_vector_graphics.md).
- [Image Processing](../multimedia_images/images.md).

{{#include 2d_renderers.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
