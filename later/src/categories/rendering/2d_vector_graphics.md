<!-- TODO Reviewed ??-??-2026 -->
# Vector Graphics in 2D

{{#include 2d_vector_graphics.incl.md}}

Vector graphics describe shapes and paths with coordinates, lines, and curves instead of storing a fixed grid of pixels. A vector scene can be scaled to different sizes, but it still has to be rasterized or drawn by a renderer before it appears on a screen.

## Build a Path

A path is an ordered set of subpaths. A subpath starts at a point, adds straight or curved segments, and ends open or closed. This host-buildable example creates a closed path and tessellates it into vertices and indices:

{{#include ../../../crates/cats/rendering/examples/2d_vector_graphics/main.rs:example}}

The path is geometry, not a colored image. A drawing operation also needs a fill or stroke style, a transform, and a renderer that turns the geometry into pixels.

## From Geometry to Pixels

1. Build paths in a consistent local coordinate system.
2. Apply transforms for placement, scaling, and rotation; keep device-pixel scaling separate from the shape's design coordinates.
3. Choose a fill rule and stroke width, cap, and join. These settings change which regions are covered and how outlines meet at corners.
4. Tessellate curves into line segments or triangles, or use a renderer that performs this step internally. A tolerance controls the approximation error and should account for the final display scale.
5. Rasterize the result with a CPU or GPU backend, then composite it with the rest of the scene using the expected color and alpha conventions.

Use bounds and clipping to limit work for large scenes. For interactive diagrams, charts, and editors, retain the source paths so you can update geometry and perform hit testing without trying to reconstruct shapes from the rendered pixels.

## `lyon` {#lyon}

[`lyon`][c~lyon~docs]↗{{hi:lyon}} builds paths and tessellates fills and strokes into vertices and indices. It does not provide a window or final drawing backend; pass its generated geometry to a renderer such as `wgpu` or another graphics API. Lyon is useful when an application needs to inspect or control the generated geometry, or when it uses the same path data across rendering operations.

Run the example from `later/crates` with `cargo run -p rendering --example 2d_vector_graphics`; it only generates and reports mesh geometry, so it needs no window or graphics device.

## `femtovg` {#femtovg}

[![femtovg][c~femtovg~docs~badge]][c~femtovg~docs] [![femtovg~crates.io][c~femtovg~crates.io~badge]][c~femtovg~crates.io] [![femtovg~repo][c~femtovg~repo~badge]][c~femtovg~repo] [![femtovg~lib.rs][c~femtovg~lib.rs~badge]][c~femtovg~lib.rs]{{hi:femtovg}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

[`femtovg`][c~femtovg~docs]↗{{hi:femtovg}} provides a canvas-style API for antialiased 2D paths, images, and text. Its OpenGL renderer needs a compatible graphics context supplied by the application. Use it when you want a higher-level drawing canvas and already have, or can create, the required window and context.

The companion example creates a window and OpenGL context with `winit` and `glutin`, then draws a path. Run it from `later/crates` with `cargo run -p gui --example femtovg --features femtovg`. It needs a desktop display and a working OpenGL driver; automated headless tests compile it but skip opening the window.

{{#include ../../../crates/cats/gui/examples/2d_renderers/femtovg.rs:example}}

## `vger` {#vger}

[![vger][c~vger~docs~badge]][c~vger~docs] [![vger~crates.io][c~vger~crates.io~badge]][c~vger~crates.io] [![vger~repo][c~vger~repo~badge]][c~vger~repo] [![vger~lib.rs][c~vger~lib.rs~badge]][c~vger~lib.rs]{{hi:vger}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

[`vger`][c~vger~docs]↗{{hi:vger}} is a GPU renderer aimed at dynamic 2D interfaces. It provides a rendering layer for applications that need frequently changing shapes and UI elements; check its current backend and platform requirements before choosing it.

This book does not ship a runnable `vger` example. The current `vger` API expects the application to provide its WGPU device, queue, texture format, and render-pass integration, and the crate targets a different WGPU version from this workspace. See the [vger API documentation][c~vger~docs] for the version-specific integration contract before adding it to an application.

## Choose the Rendering Boundary

Use Lyon when your application needs path geometry and wants to select its own backend. Use a canvas renderer such as `femtovg` when you prefer drawing commands and the renderer's built-in path handling. Use a GPU renderer such as `vger` when its scene model and backend fit the rest of the application. For static vector assets, rendering SVG through an SVG-focused pipeline may be simpler than building paths by hand; see [SVG Rendering](svg_rendering.md).

## See Also {#see-also .skip}

- [2D Rendering][are-we-game-yet?-2drendering~website]↗: Sprites, vectors, splines, hex grids and more.

## Related Topics {#related-topics .skip}

- [2D Renderers](2d_renderers.md).
- [2D Raster Graphics](2d_raster_graphics.md).
- [3D Renderers](3d_renderers.md).
- [Rendering](index.md).
- [Rendering Engines](../rendering_engine/index.md).
- [SVG Rendering](svg_rendering.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1214)
[content audit: content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1904)
</div>
