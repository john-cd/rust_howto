<!-- TODO Reviewed ??-??-2026. -->

# Count Unicode Text for Display

Rust `str` and `String` contain valid UTF-8. `.len()` counts bytes and `.chars()` counts Unicode scalar values. A combining accent and a flag sequence show why neither is a user-facing character count. Grapheme clusters are a useful default for cursor movement and selection, although [Unicode UAX #29](https://www.unicode.org/reports/tr29/) allows language-specific tailoring.

## Collect Unicode Graphemes {#collect-unicode-graphemes}

```rust,noplayground
{{#include ../../../crates/cats/text_processing/examples/unicode/graphemes.rs:example}}
```

Run `cargo test -p text_processing --example unicode` from `bk/crates`. To print the four measurements, run `cargo run -p text_processing --example unicode -- graphemes`. The example compares UTF-8 bytes, scalar values, grapheme clusters from [`unicode-segmentation`](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/), and estimated terminal columns from [`unicode-width`](https://docs.rs/unicode-width/latest/unicode_width/). Column width can vary with terminal rendering and fonts. A grapheme count does not limit encoded storage size.

Segmentation does not normalize text. Precomposed `é` and `e` plus a combining accent have different bytes and compare unequal as Rust strings, although their NFC-normalized forms compare equal in the example. Normalize only when the application needs canonical equivalence; keep the original text when its exact bytes matter. For encoding boundaries, see [Character Encoding](../encoding/string_encoding.md). For string ownership and mutation, see [Strings](../data-structures/strings.md).
