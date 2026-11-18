<!-- TODO Reviewed ??-??-2026. -->

# SVG Rendering

SVG describes vector graphics as XML. A rendering workflow parses the
document, resolves styles and resources, then paints the scene into a window
or image. [`usvg`][c~usvg~docs]↗{{hi:usvg}} normalizes the document into a
tree; [`resvg`][c~resvg~docs]↗{{hi:resvg}} renders that tree into a
[`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}} pixmap.

The examples use inline SVG so their inputs are self-contained. If an
application accepts SVG from users, bound input size and output dimensions and
control access to external resources. For files with relative image or font
references, configure the parser's resource directory and font database
deliberately.

## Parse and Inspect SVG with usvg {#usvg}

[`usvg`][c~usvg~docs]↗{{hi:usvg}} parses and normalizes an SVG document. It
does not rasterize the scene; its reported viewport may differ from the bounds
of visible elements.

```rust,editable
{{#include ../../../crates/cats/graphics/examples/svg_rendering/usvg.rs:example}}
```

```console
cargo run --locked -p graphics --example svg_rendering -- usvg
```

## Rasterize SVG with resvg {#resvg}

[`resvg`][c~resvg~docs]↗{{hi:resvg}} paints a parsed SVG tree to a pixmap.
The example writes a PNG to the system temporary directory and prints its
path.

```rust,editable
{{#include ../../../crates/cats/graphics/examples/svg_rendering/resvg.rs:example}}
```

```console
cargo run --locked -p graphics --example svg_rendering -- resvg
```

## Related Topics

- [Build and tessellate vector paths](2d_vector_graphics.md).
- [Image Processing](../multimedia_images/images.md).

{{#include svg_rendering.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
