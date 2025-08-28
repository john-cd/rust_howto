# Unicode

{{#include unicode.incl.md}}

Unicode segmentation is the process of dividing a string of Unicode text into meaningful units, such as grapheme clusters (user-perceived characters), words, and sentences, following the rules defined by the Unicode Standard.

## Collect Unicode Graphemes {#collect-unicode-graphemes}

[![unicode-segmentation][c~unicode-segmentation~docs~badge]][c~unicode-segmentation~docs] [![unicode-segmentation~crates.io][c~unicode-segmentation~crates.io~badge]][c~unicode-segmentation~crates.io] [![unicode-segmentation~repo][c~unicode-segmentation~repo~badge]][c~unicode-segmentation~repo] [![unicode-segmentation~lib.rs][c~unicode-segmentation~lib.rs~badge]][c~unicode-segmentation~lib.rs]{{hi:unicode-segmentation}}{{hi:Boundary}}{{hi:Grapheme}}{{hi:Text}}{{hi:Unicode}}{{hi:Word}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`unicode-segmentation`][c~unicode-segmentation~docs]↗{{hi:unicode-segmentation}} collects individual Unicode graphemes{{hi:Graphemes}} from UTF-8{{hi:UTF-8}} strings. See in particular the [`unicode-segmentation::UnicodeSegmentation::graphemes`][c~unicode-segmentation::UnicodeSegmentation::graphemes~docs]↗{{hi:unicode-segmentation::UnicodeSegmentation::graphemes}} function.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/unicode/graphemes.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[strings | Strings]].
- [[string_encoding | String Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/1192)

- [unicode_width][unicode-width~website]↗: Determine displayed width of [`char`][primitive~char]↗{{hi:char}} and [`str`][primitive~str]↗{{hi:str}} types according to Unicode Standard Annex #11 rules.

</div>
