<!-- TODO Reviewed ??-??-2026 -->

# SVG Rendering

{{#include svg_rendering.incl.md}}

SVG is a resolution-independent vector format. A rendering workflow usually parses SVG markup, resolves styles and resources, then paints the result into a window or an image. [`usvg`][c~usvg~docs]↗{{hi:usvg}} builds a normalized tree; [`resvg`][c~resvg~docs]↗{{hi:resvg}} renders that tree into a [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} pixmap.

These examples use inline SVG to keep the parsing and rendering steps self-contained. For files with relative image or font references, configure `usvg::Options` with an appropriate resource directory and font database. Treat SVG input as untrusted data when accepting it from users, and bound input size and output dimensions in applications.

## `resvg` {#resvg}

[`resvg`][c~resvg~docs]↗{{hi:resvg}} rasterizes a parsed SVG tree into a pixmap. The output dimensions below come from the SVG viewport; choose a different pixmap size and transform when an application needs a particular output scale.

```rust,editable
{{#include ../../../crates/cats/rendering/examples/svg_rendering/resvg.rs:example}}
```

## `usvg` {#usvg}

[`usvg`][c~usvg~docs]↗{{hi:usvg}} parses and normalizes an SVG document into a tree that downstream renderers can consume. It is not itself a raster renderer. `Tree::size()` reports the SVG viewport, which can differ from the bounds of the visible nodes.

```rust,editable
{{#include ../../../crates/cats/rendering/examples/svg_rendering/usvg.rs:example}}
```

## Related Topics {#related-topics .skip}

- Vector graphics and rendering.
- UI toolkits and shape drawing.
- Cross-platform graphics libraries.
- Image and multimedia workflows.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write.](https://github.com/john-cd/rust_howto/issues/1216)
Decide what to cover.
</div>
