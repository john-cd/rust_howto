# Unicode

{{#include unicode.incl.md}}

Unicode segmentation is the process of dividing a string of Unicode text into meaningful units, such as grapheme clusters (user-perceived characters), words, and sentences, following the rules defined by the Unicode Standard.

## Collect Unicode Graphemes {#collect-unicode-graphemes}

[![unicode-segmentation][c~unicode_segmentation~docs~badge]][c~unicode_segmentation~docs] [![unicode-segmentation~crates.io][c~unicode_segmentation~crates.io~badge]][c~unicode_segmentation~crates.io] [![unicode-segmentation~github][c~unicode_segmentation~github~badge]][c~unicode_segmentation~github] [![unicode-segmentation~lib.rs][c~unicode_segmentation~lib.rs~badge]][c~unicode_segmentation~lib.rs]{{hi:unicode-segmentation}}{{hi:Boundary}}{{hi:Grapheme}}{{hi:Text}}{{hi:Unicode}}{{hi:Word}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`unicode-segmentation`][c~unicode_segmentation~docs]⮳{{hi:unicode-segmentation}} collects individual Unicode graphemes{{hi:Graphemes}} from UTF-8{{hi:UTF-8}} strings. See in particular the [`unicode_segmentation::UnicodeSegmentation::graphemes`][c~unicode_segmentation::UnicodeSegmentation::graphemes~docs]{{hi:unicode_segmentation::UnicodeSegmentation::graphemes}}⮳ function.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/unicode/graphemes.rs:example}}
```

## Related Topics {#skip}

- [[strings | Strings]].
- [[string_encoding | String Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/1192)

- [unicode_width](https://unicode-rs.github.io/unicode-width/unicode_width/index.html) Determine displayed width of `char` and `str` types according to Unicode Standard Annex #11 rules.

</div>
