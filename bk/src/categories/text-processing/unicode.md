<!-- TODO Reviewed ??-??-2026. -->

# Count Unicode Text for Display

Rust `str` and `String` values contain valid UTF-8. `.len()` counts bytes and `.chars()` counts Unicode scalar values; neither necessarily matches what a reader sees. A combining accent or an emoji sequence can contain multiple scalar values while displaying as one grapheme cluster.

For cursor movement, selection, or a user-facing character count, grapheme clusters are a useful default. The [`unicode-segmentation`][c~unicode-segmentation~docs]↗{{hi:unicode-segmentation}} crate implements the default grapheme and word boundaries from [Unicode Text Segmentation (UAX #29)][unicode~uax29]↗. Applications may need language-specific tailoring.

Segmentation does not normalize text. A precomposed `é` and `e` followed by a combining acute accent can look the same while having different bytes and comparing unequal as Rust strings. Normalize only when canonical equivalence is part of the application's comparison rules.

## Collect Unicode Graphemes {#collect-unicode-graphemes}

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/unicode/graphemes.rs:example}}
```

The example compares byte length, scalar values, grapheme clusters, and estimated terminal columns. Column width depends on terminal rendering and fonts; a grapheme count does not limit the encoded storage size. Normalize text separately when needed.

For string ownership and mutation, see [Strings](../data-structures/strings.md). For byte encodings, see [Character Encoding](../encoding/string_encoding.md).

## Related Topics {#related-topics .skip}

- [String Manipulation](string_manipulation.md).
- [Text Analysis](text_analysis.md).

{{#include unicode.incl.md}}
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
