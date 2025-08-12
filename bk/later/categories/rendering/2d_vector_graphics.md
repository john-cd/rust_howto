# Vector-Based 2D Rendering

{{#include 2d_vector_graphics.incl.md}}

## `lyon` {#lyon}

[`lyon`][c~lyon~docs]↗{{hi:lyon}} is a library for generating vector paths and shapes. It's useful for creating the geometry for vector [graphics][p~graphics], but you'd typically use a [rendering][p~rendering] library like [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`raqote`][c~raqote~docs]↗{{hi:raqote}}, or [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} to actually draw them. [`lyon`][c~lyon~docs]↗{{hi:lyon}} handles the math and path calculations.

[`lyon`][c~lyon~docs]↗{{hi:lyon}} generates vector paths and shapes. Often used with a rendering library. For basic geometry, you might just use structs and functions within your own code. [`lyon`][c~lyon~docs]↗{{hi:lyon}} can also generate various types of curves (Bézier curves, etc.).

It's useful for creating the geometry for vector graphics, but you'd typically use a [rendering][p~rendering] library like [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`raqote`][c~raqote~docs]↗{{hi:raqote}}, or [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} to actually draw them.

## `femtovg` {#femtovg}

[![femtovg][c~femtovg~docs~badge]][c~femtovg~docs] [![femtovg~crates.io][c~femtovg~crates.io~badge]][c~femtovg~crates.io] [![femtovg~github][c~femtovg~github~badge]][c~femtovg~github] [![femtovg~lib.rs][c~femtovg~lib.rs~badge]][c~femtovg~lib.rs]{{hi:femtovg}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

[`femtovg`][c~femtovg~docs]↗{{hi:femtovg}} is an anti-aliased 2D vector drawing library. It is "OpenGL based. Offers a simple API. Probably the easiest to get started with." ([blessed.rs](https://blessed.rs/crates#section-graphics)).

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/femtovg.rs:example}}
```

## `vger` {#vger}

[![vger][c~vger~docs~badge]][c~vger~docs] [![vger~crates.io][c~vger~crates.io~badge]][c~vger~crates.io] [![vger~github][c~vger~github~badge]][c~vger~github] [![vger~lib.rs][c~vger~lib.rs~badge]][c~vger~lib.rs]{{hi:vger}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

[`vger`][c~vger~docs]↗{{hi:vger}} is a 2D [GPU][p~gpu] renderer for dynamic UIs.

"A simpler WGPU-based option which is less innovative but currently more stable than [`vello`][c~vello~docs]↗{{hi:vello}}." ([blessed.rs](https://blessed.rs/crates#section-graphics)).

```rust,editable
{{#include ../../../crates/cats/gui/examples/2d_renderers/vger.rs:example}}
```

## Related Topics {#related-topics}

- [[2d_renderers | 2D Renderers]].
- [[2d_raster_graphics | 2D Raster Graphics]].
- [[3d_renderers | 3d Renderers]].
- [[opengl | OpenGL]].
- [[rendering | Rendering]].
- [[rendering_engines | Rendering Engines]].
- [[svg_rendering | SVG Rendering]].

## See Also

[2D Rendering][are-we-game-yet?-2drendering~website]↗: Sprites, vectors, splines, hex grids and more.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1214)
</div>
