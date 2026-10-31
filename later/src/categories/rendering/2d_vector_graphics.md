<!-- TODO Reviewed ??-??-2026 -->

# Vector Graphics in 2D

{{#include 2d_vector_graphics.incl.md}}

Vector graphics describe shapes and paths with coordinates, lines, and curves instead of storing a fixed grid of pixels. A vector scene can be scaled to different sizes, but it still has to be rasterized or drawn by a renderer before it appears on a screen.

## Build a Path

A path is an ordered set of subpaths. A subpath starts at a point, adds straight or curved segments, and ends open or closed. This host-buildable example creates a closed path and tessellates it into vertices and indices:

```rust,editable
{{#include ../../../crates/cats/rendering/examples/2d_vector_graphics/main.rs:example}}
```

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

{{#badges femtovg graphics=Graphics}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}}

[`femtovg`][c~femtovg~docs]↗{{hi:femtovg}} provides a canvas-style API for antialiased 2D paths, images, and text. Its OpenGL renderer needs a compatible graphics context supplied by the application. Use it when you want a higher-level drawing canvas and already have, or can create, the required window and context.

The companion example creates a window and OpenGL context with `winit` and `glutin`, then draws a path. Run it from `later/crates` with `cargo run -p gui --example femtovg --features femtovg`. It needs a desktop display and a working OpenGL driver; automated headless tests compile it but skip opening the window. On a machine without a display or GPU, run it under a virtual X server with software OpenGL: on Debian or Ubuntu, install `xvfb libgl1-mesa-dri libxkbcommon-x11-0` and use `xvfb-run -a cargo run -p gui --example femtovg --features femtovg`; the window stays open until you close it, so stop it with a timeout.

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/femtovg.rs:example}}
```

## `vger` {#vger}

{{#badges vger graphics=Graphics}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}}

[`vger`][c~vger~docs]↗{{hi:vger}} is a GPU renderer aimed at dynamic 2D interfaces. It provides a rendering layer for applications that need frequently changing shapes and UI elements; check its current backend and platform requirements before choosing it.

The latest `vger` release (0.4.0) dates from September 2024, so treat the crate as inactive. This book does not ship a runnable `vger` example. The current `vger` API expects the application to provide its WGPU device, queue, texture format, and render-pass integration, and the crate targets a different WGPU version from this workspace. See the [vger API documentation][c~vger~docs] for the version-specific integration contract before adding it to an application.

## `skia-safe` {#skia-safe}

{{#badges skia-safe api-bindings="API bindings" graphics=Graphics multimedia::images=Images rendering::graphics-api="Graphics APIs" visualization=Visualization}}{{hi:OpenGL}}{{hi:Pdf}}{{hi:Rust-bindings}}{{hi:Skia}}{{hi:Vulkan}}

[`skia-safe`][c~skia-safe~docs]↗{{hi:skia-safe}} offers "Bindings to the Skia C++ library. The most complete option with excellent performance. However, it can be difficult to get it to compile." ([blessed.rs][blessed.rs-graphics~website]↗).

The build script of `skia-safe` downloads prebuilt Skia binaries for common targets, or compiles Skia from source (which needs Python, a C++ toolchain and `ninja`) when none is available. Plan for this in CI and offline builds, and pin the crate version to match other crates that embed Skia. The example below renders on the CPU into an in-memory surface, so it needs no GPU or window. From `later/crates`, run it with `cargo run -p gui --example 2d_renderers --features skia,femtovg -- skia_safe`.

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/skia_safe.rs:example}}
```

## `vello` {#vello}

{{#badges vello graphics=Graphics rendering=Rendering}}{{hi:2d}}{{hi:Vector-graphics}}

[`vello`][c~vello~docs]↗{{hi:vello}} is a GPU compute-centric 2D renderer built on `wgpu`. Build and test its scene independently of a device, then render it when a compatible GPU adapter is available. From `later/crates`, run the off-screen example with `cargo run -p gui --example vello_renderer --features vello`; it requires a compatible GPU adapter.

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/vello.rs:example}}
```

## WebRender {#webrender}

[`webrender`][c~webrender~docs] is the rendering component used by Gecko. Its integration is oriented toward a browser engine, so this book treats it as background rather than a standalone application recipe. For a new canvas or UI renderer, compare the alternatives above instead of assuming the retained experimental WebRender source is a supported integration.

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
  write
</div>
