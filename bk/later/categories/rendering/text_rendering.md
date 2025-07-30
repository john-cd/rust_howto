# Font Loading & Text Rendering

{{#include text_rendering.incl.md}}

Libraries and tools for loading and [rendering][p~rendering] fonts.

[`fontdue`][c~fontdue~docs]⮳{{hi:fontdue}} is a fast font rasterizer. [`ttf-parser`][c~ttf_parser~docs]⮳{{hi:ttf-parser}} and [`opentype`][c~opentype~docs]⮳{{hi:opentype}} provide lower-level font parsing capabilities. [`ab_glyph`][c~ab_glyph~docs]⮳{{hi:ab_glyph}} is another font rendering option.

## `ab_glyph` {#ab_glyph}

API for loading, scaling, positioning and rasterizing OpenType font glyphs.

## `rusttype` {#rusttype}

`rusttype` is a TrueType font rasterizer and a pure Rust alternative to libraries like FreeType. RustType provides an API for loading, querying and rasterising TrueType fonts. It also provides an implementation of a dynamic [GPU][p~gpu] glyph cache for hardware font rendering.

## `fontdue` {#fontdue}

[`fontdue`][c~fontdue~docs]⮳{{hi:fontdue}} is a fast, pure Rust font loading and rasterization library. [`glyph_brush`][c~glyph_brush~docs]⮳{{hi:glyph_brush}} is for efficient text layout and caching.

A simple `no_std` font parser and rasterizer.

## `ttf-parser` and  `opentype` {#ttf-parser}

[`ttf-parser`][c~ttf_parser~docs]⮳{{hi:ttf-parser}} and [`opentype`][c~opentype~docs]⮳{{hi:opentype}} provide lower-level font parsing capabilities.

[`ttf-parser`][c~ttf_parser~docs]⮳{{hi:ttf-parser}}

[`opentype`][c~opentype~docs]⮳{{hi:opentype}}

## Related Topics {#related-topics}

- [`sdl2`][c~sdl2~docs]⮳{{hi:sdl2}}: SDL2 bindings for Rust.

## Related Topics {#skip1}

- [Are we Game yet? Text Rendering][are we game yet? text rendering].
- [[text_layout | Text Layout]].

[are we game yet? text rendering]: https://arewegameyet.rs/ecosystem/textrendering
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write / decide what to cover](https://github.com/john-cd/rust_howto/issues/1212)
</div>
