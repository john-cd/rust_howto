<!-- TODO Reviewed ??-??-2026. -->

# Graphics

This crate contains the runnable examples in the book's **Graphics** category.

## Examples Covered

- `graphics`: write a simple SVG scene.
- `2d_raster_graphics`: draw a CPU-backed pixmap with `tiny-skia`.
- `2d_vector_graphics`: tessellate a path with `lyon`.
- `2d_renderers`: tessellate a path with `lyon` and rasterize it with `tiny-skia`.
- `3d_projection`: project a 3D point with `glam`.
- `svg_rendering`: parse SVG with `usvg` or rasterize it with `resvg`.
- `text_rendering`: rasterize a glyph with `fontdue` and read font metrics with `ttf-parser`.

## Usage

To check or run examples in this crate:

```console
cargo check --package graphics --examples
cargo test --locked --package graphics --all-targets
```

Code examples correspond to chapters in `bk/src/categories/graphics`. They run without a window or graphics device.
