<!-- TODO Reviewed ??-??-2026. -->

# Rasterize a Font Glyph

Glyph rasterization converts one character from a font into a coverage bitmap.
Coverage describes how much each pixel is covered by the glyph; an application
combines those values with a text color to paint the glyph.

This example uses [`skrifa`][c~skrifa~docs]↗{{hi:skrifa}} to scale a glyph
outline and read its advance, then [`tiny-skia`][c~tiny-skia~docs]↗{{hi:tiny-skia}}
to rasterize the path into a pixmap. It bundles Roboto Regular under the SIL
Open Font License 1.1; the license text is at
bk/crates/cats/graphics/assets/OFL.txt.

## Rasterize a Glyph and Read Its Metrics {#rasterize-a-glyph}

```rust,editable
{{#include ../../../crates/cats/graphics/examples/text_rendering/rasterize_glyph.rs:example}}
```

```console
cargo run --locked -p graphics --example text_rendering
```

Glyph rasterization does not perform text shaping or layout. Bidirectional
text, ligatures, script shaping, line breaking, and fallback fonts require a
text-layout layer.

## Related Topics

- [Image Processing](../multimedia_images/images.md) covers raster image data.
- [Text Processing](../text-processing/index.md) covers working with text.

{{#include text_rendering.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
