<!-- TODO Reviewed ??-??-2026. -->

# Vector Graphics in 2D

Vector graphics describe shapes with coordinates, lines, and curves instead
of storing a fixed grid of pixels. A vector scene can be scaled, but it still
needs a renderer to turn its geometry into pixels.

## Tessellate a Vector Path {#tessellate-a-vector-path}

A path is an ordered set of subpaths. Build the path, close it when it
describes a filled region, and ask [`lyon`][c~lyon~docs]↗{{hi:lyon}} to
tessellate the fill into vertices and indices. The example reports the
generated mesh; it does not draw the mesh.

```rust,editable
{{#include ../../../crates/cats/graphics/examples/2d_vector_graphics/vector_graphics_2d.rs:example}}
```

```console
cargo run --locked -p graphics --example 2d_vector_graphics
```

## From Geometry to Pixels

For a visible shape, the renderer also needs a fill or stroke style, a
transform, and a target surface. A typical pipeline is:

1. Build paths in design coordinates.
2. Apply transforms for placement, scaling, and rotation.
3. Choose a fill rule or stroke width, cap, and join.
4. Tessellate curves into geometry, or use a renderer that does so internally.
5. Rasterize and composite the result using the target's color and alpha
   conventions.

Keep device-pixel scaling separate from design coordinates. For interactive
diagrams and editors, retaining the source paths also makes updates and hit
testing simpler than reconstructing shapes from pixels.

## Choose a Drawing Backend

Use Lyon when you need tessellated geometry for your own pipeline. A canvas renderer such as [`femtovg`][c~femtovg~docs] draws paths to an OpenGL context. [`vello`][c~vello~docs] uses GPU compute to draw a vector scene, while [`skia-safe`][c~skia-safe~docs] exposes Skia's CPU and GPU drawing APIs. Choose against the target and the context or device your application can supply.

## Related Topics

- [Tessellate, Then Rasterize](2d_renderers.md#tessellate-and-rasterize) passes
  Lyon's mesh to a CPU renderer.
- [SVG Rendering](svg_rendering.md) covers static vector documents.

{{#include 2d_vector_graphics.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
