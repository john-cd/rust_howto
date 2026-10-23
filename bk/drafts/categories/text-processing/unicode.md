<!-- TODO Reviewed ??-??-2026 -->
# Unicode

{{#include unicode.incl.md}}

Rust `str` and `String` values are UTF-8. Their `len()` method returns a byte count, while `chars()` iterates Unicode scalar values. Neither count necessarily matches the characters a reader sees: a visible character can be encoded as more than one scalar value, and an emoji sequence can contain several scalars.

For cursor movement, selection, or a user-facing character count, use grapheme clusters. The [`unicode-segmentation`][c~unicode-segmentation~docs]↗ crate implements the default grapheme, word, and sentence boundaries described by [Unicode Text Segmentation (UAX #29)][unicode~uax29]↗. These boundaries approximate user-perceived characters; applications with language-specific rules may need to tailor them.

Grapheme segmentation does not normalize text. For example, a precomposed `é` and `e` followed by a combining acute accent can look the same while having different UTF-8 bytes and comparing unequal as Rust strings. Normalize text when an application requires canonically equivalent strings to compare equally.

## Collect Unicode Graphemes {#collect-unicode-graphemes}

[![unicode-segmentation][c~unicode-segmentation~docs~badge]][c~unicode-segmentation~docs] [![unicode-segmentation~crates.io][c~unicode-segmentation~crates.io~badge]][c~unicode-segmentation~crates.io] [![unicode-segmentation~repo][c~unicode-segmentation~repo~badge]][c~unicode-segmentation~repo] [![unicode-segmentation~lib.rs][c~unicode-segmentation~lib.rs~badge]][c~unicode-segmentation~lib.rs]{{hi:unicode-segmentation}}{{hi:Boundary}}{{hi:Grapheme}}{{hi:Text}}{{hi:Unicode}}{{hi:Word}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`unicode-segmentation`][c~unicode-segmentation~docs]↗{{hi:unicode-segmentation}} collects individual Unicode graphemes{{hi:Graphemes}} from UTF-8{{hi:UTF-8}} strings. See in particular the [`unicode-segmentation::UnicodeSegmentation::graphemes`][c~unicode-segmentation::UnicodeSegmentation::graphemes~docs]↗{{hi:unicode-segmentation::UnicodeSegmentation::graphemes}} function.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/unicode/graphemes.rs:example}}
```

The example distinguishes grapheme clusters from scalar values and bytes. A user-facing count should name which unit it uses; a grapheme count is not a limit on encoded data size.

## Related Topics {#related-topics .skip}

- [[strings | Strings]].
- [[string_encoding | String Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/1192)

- [unicode_width][unicode-width~website]↗: Determine displayed width of [`char`][primitive~char]↗{{hi:char}} and [`str`][primitive~str]↗{{hi:str}} types according to Unicode Standard Annex #11 rules.

</div>
