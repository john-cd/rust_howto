# Font Loading & Text Rendering

{{#include text_rendering.incl.md}}

Libraries and tools for loading and [rendering][p~rendering] fonts.

[`fontdue`][c~fontdue~docs]↗{{hi:fontdue}} is a fast font rasterizer. [`ttf-parser`][c~ttf_parser~docs]↗{{hi:ttf-parser}} and [`opentype`][c~opentype~docs]↗{{hi:opentype}} provide lower-level font parsing capabilities. [`ab_glyph`][c~ab_glyph~docs]↗{{hi:ab_glyph}} is another font rendering option.

## `ab_glyph` {#ab_glyph}

API for loading, scaling, positioning and rasterizing OpenType font glyphs.

## `rusttype` {#rusttype}

[![rusttype][c~rusttype~docs~badge]][c~rusttype~docs] [![rusttype~crates.io][c~rusttype~crates.io~badge]][c~rusttype~crates.io] [![rusttype~github][c~rusttype~github~badge]][c~rusttype~github] [![rusttype~lib.rs][c~rusttype~lib.rs~badge]][c~rusttype~lib.rs]{{hi:rusttype}}{{hi:Otf}}{{hi:Opentype}}{{hi:Ttf}}{{hi:Font}}{{hi:Truetype}}

[`rusttype`][c~rusttype~docs]↗{{hi:rusttype}} is a TrueType font rasterizer and a pure Rust alternative to libraries like FreeType. RustType provides an API for loading, querying and rasterising TrueType fonts. It also provides an implementation of a dynamic [GPU][p~gpu] glyph cache for hardware font rendering.

## `fontdue` {#fontdue}

[![fontdue][c~fontdue~docs~badge]][c~fontdue~docs] [![fontdue~crates.io][c~fontdue~crates.io~badge]][c~fontdue~crates.io] [![fontdue~github][c~fontdue~github~badge]][c~fontdue~github] [![fontdue~lib.rs][c~fontdue~lib.rs~badge]][c~fontdue~lib.rs]{{hi:fontdue}}{{hi:Font}}{{hi:Opentype}}{{hi:Text}}{{hi:Truetype}}{{hi:Ttf}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`fontdue`][c~fontdue~docs]↗{{hi:fontdue}} is a fast, pure Rust font loading and rasterization library. [`glyph_brush`][c~glyph_brush~docs]↗{{hi:glyph_brush}} is for efficient text layout and caching.

A simple `no_std` font parser and rasterizer.

## `ttf-parser` and  `opentype` {#ttf-parser}

[![ttf-parser][c~ttf_parser~docs~badge]][c~ttf_parser~docs] [![ttf-parser~crates.io][c~ttf_parser~crates.io~badge]][c~ttf_parser~crates.io] [![ttf-parser~github][c~ttf_parser~github~badge]][c~ttf_parser~github] [![ttf-parser~lib.rs][c~ttf_parser~lib.rs~badge]][c~ttf_parser~lib.rs]{{hi:ttf-parser}}{{hi:Opentype}}{{hi:Truetype}}{{hi:Ttf}} [![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}

[`ttf-parser`][c~ttf_parser~docs]↗{{hi:ttf-parser}} and [`opentype`][c~opentype~docs]↗{{hi:opentype}} provide lower-level font parsing capabilities.

[`ttf-parser`][c~ttf_parser~docs]↗{{hi:ttf-parser}}

[`opentype`][c~opentype~docs]↗{{hi:opentype}}

## Related Topics {#related-topics}

- [`sdl2`][c~sdl2~docs]↗{{hi:sdl2}}: SDL2 bindings for Rust.

- [Are we Game yet? Text Rendering][are we game yet? text rendering].
- [[text_layout | Text Layout]].

[are we game yet? text rendering]: https://arewegameyet.rs/ecosystem/textrendering
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write / decide what to cover](https://github.com/john-cd/rust_howto/issues/1212)
</div>
