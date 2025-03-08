# Font Loading & Text Rendering

{{#include text_rendering.incl.md}}

Libraries and tools for loading and rendering fonts.

[`fontdue`][c-fontdue]⮳{{hi:fontdue}} is a fast font rasterizer. [`ttf-parser`][c-ttf_parser]⮳{{hi:ttf-parser}} and [`opentype`][c-opentype]⮳{{hi:opentype}} provide lower-level font parsing capabilities. [`ab_glyph`][c-ab_glyph]⮳{{hi:ab_glyph}} is another font rendering option.

## `ab_glyph` {#ab_glyph}

API for loading, scaling, positioning and rasterizing OpenType font glyphs.

## `rusttype` {#rusttype}

`rusttype` is a TrueType font rasterizer and a pure Rust alternative to libraries like FreeType. RustType provides an API for loading, querying and rasterising TrueType fonts. It also provides an implementation of a dynamic GPU glyph cache for hardware font rendering.

## `fontdue` {#fontdue}

[`fontdue`][c-fontdue]⮳{{hi:fontdue}} is a fast, pure Rust font loading and rasterization library. [`glyph_brush`][c-glyph_brush]⮳{{hi:glyph_brush}} is for efficient text layout and caching.

A simple `no_std` font parser and rasterizer.

## `ttf-parser` and  `opentype`

[`ttf-parser`][c-ttf_parser]⮳{{hi:ttf-parser}} and [`opentype`][c-opentype]⮳{{hi:opentype}} provide lower-level font parsing capabilities.

[`ttf-parser`][c-ttf_parser]⮳{{hi:ttf-parser}}

[`opentype`][c-opentype]⮳{{hi:opentype}}

## See also

- `sdl2`: SDL2 bindings for Rust.

## Related Topics

- [Are we Game yet? Text Rendering](https://arewegameyet.rs/ecosystem/textrendering/).
- [[text_layout | Text Layout]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write / decide what to cover
</div>
