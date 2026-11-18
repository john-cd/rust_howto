<!-- TODO Reviewed ??-??-2026. -->

# Graphics

[![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

Graphics code turns descriptions of shapes, paths, text, and 3D points into
images. These recipes focus on CPU and off-screen work that can be tested
without a display server or graphics device: drawing into a pixel buffer,
tessellating vector paths, rasterizing SVG, and projecting a point through a
3D camera.

## Choose a Starting Point

| Goal | Start with | Result |
| --- | --- | --- |
| Draw paths into an in-memory image | [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} | A CPU-backed pixel buffer. |
| Turn a vector path into geometry | [`lyon`][c~lyon~docs]↗{{hi:lyon}} | Vertices and indices for a fill. |
| Parse or rasterize an SVG | [`usvg`][c~usvg~docs]↗{{hi:usvg}} and [`resvg`][c~resvg~docs]↗{{hi:resvg}} | A normalized SVG tree or a raster image. |
| Rasterize a font glyph | [`fontdue`][c~fontdue~docs]↗{{hi:fontdue}} | A coverage bitmap and glyph metrics. |
| Transform a 3D point to screen coordinates | [`glam`][c~glam~docs]↗{{hi:glam}} | Pixel coordinates and depth, without drawing a frame. |

Run the examples from `bk/crates` with `cargo run --locked -p graphics --example <name>`. The SVG example accepts either `usvg` or `resvg` after `--`; the triangle example writes SVG to standard output, so redirect it to a file before opening it in a browser.

These examples do not open a window or submit GPU work. Windowing, shaders,
graphics APIs, and game-engine integration require platform-specific setup and
are outside this category's runnable scope.

## Chapters

- [2D renderers](2d_renderers.md) compares paths, meshes, and pixel buffers.
- [2D raster graphics](2d_raster_graphics.md) draws into a CPU-backed pixmap.
- [2D vector graphics](2d_vector_graphics.md) builds and tessellates paths.
- [3D projection](3d_projection.md) applies model, view, and projection transforms.
- [SVG rendering](svg_rendering.md) parses SVG and paints it into pixels.
- [Text rendering](text_rendering.md) rasterizes a glyph from a bundled font.

## Draw a Triangle as SVG {#draw-a-triangle-as-svg}

SVG describes shapes in a shared coordinate system. The rectangle paints the
background first, and the polygon paints the triangle over it.

```console
cargo run --locked -p graphics --example graphics > triangle.svg
```

Open `triangle.svg` in a browser to view the result.

```rust,editable
{{#include ../../../crates/cats/graphics/examples/graphics.rs:example}}
```

## Related Topics

- [Image Processing](../multimedia_images/index.md) covers image loading,
  encoding, resizing, and filtering.
- [Linear Algebra](../mathematics/linear_algebra.md) covers vectors, matrices,
  and transformations.

{{#include graphics.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
