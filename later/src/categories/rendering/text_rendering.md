<!-- TODO Reviewed ??-??-2026 -->

# Font Loading & Text Rendering

{{#include text_rendering.incl.md}}

Libraries and tools for loading and [rendering][p~rendering] fonts.

[`fontdue`][c~fontdue~docs]↗{{hi:fontdue}} is a fast font rasterizer. [`ttf-parser`][c~ttf-parser~docs]↗{{hi:ttf-parser}} and [`opentype`][c~opentype~docs]↗{{hi:opentype}} provide lower-level font parsing capabilities. [`ab_glyph`][c~ab_glyph~docs]↗{{hi:ab_glyph}} is another font rendering option.

## Rasterize a Glyph {#rasterize-a-glyph}

This example loads a TrueType font from memory, rasterizes one glyph into a coverage bitmap with `fontdue`, and reads design metrics with `ttf-parser`. The bundled font is Roboto, under the Apache License 2.0. Coverage values are alpha: composite them with a text color to draw. Shaping, bidirectional text, and fallback fonts need a layout library; see [[text_layout | Text Layout]].

```rust,editable
{{#include ../../../crates/cats/rendering/examples/text_rendering/main.rs:example}}
```

## `ab_glyph` {#ab_glyph}

API for loading, scaling, positioning and rasterizing OpenType font glyphs.

## `rusttype` {#rusttype}

{{#badges rusttype}}{{hi:Otf}}{{hi:Opentype}}{{hi:Ttf}}{{hi:Font}}{{hi:Truetype}}

[`rusttype`][c~rusttype~docs]↗{{hi:rusttype}} is a TrueType font rasterizer and a pure Rust alternative to libraries like FreeType. RustType provides an API for loading, querying and rasterising TrueType fonts. It also provides an implementation of a dynamic [GPU][p~gpu] glyph cache for hardware font rendering.

## `fontdue` {#fontdue}

{{#badges fontdue gui=GUI no-std="No standard library"}}{{hi:Font}}{{hi:Opentype}}{{hi:Text}}{{hi:Truetype}}{{hi:Ttf}}

[`fontdue`][c~fontdue~docs]↗{{hi:fontdue}} is a fast, pure Rust font loading and rasterization library. [`glyph_brush`][c~glyph_brush~docs]↗{{hi:glyph_brush}} is for efficient text layout and caching.

A simple [`#![no_std]`][book~rust-reference~no_std]{{hi:no_std}} font parser and rasterizer.

## `ttf-parser` and `opentype` {#ttf-parser}

{{#badges ttf-parser parser-implementations="Parser implementations"}}{{hi:Opentype}}{{hi:Truetype}}{{hi:Ttf}}

[`ttf-parser`][c~ttf-parser~docs]↗{{hi:ttf-parser}} and [`opentype`][c~opentype~docs]↗{{hi:opentype}} provide lower-level font parsing capabilities.

[`ttf-parser`][c~ttf-parser~docs]↗{{hi:ttf-parser}}

[`opentype`][c~opentype~docs]↗{{hi:opentype}}

## Related Topics {#related-topics .skip}

- [`sdl2`][c~sdl2~docs]↗{{hi:sdl2}}: SDL2 bindings for Rust.
- [Are we Game yet? Text Rendering][are-we-game-yet?-text-rendering~website]↗.
- [[text_layout | Text Layout]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
