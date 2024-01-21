# String Parsing

## Collect Unicode Graphemes

[![unicode-segmentation-badge]][unicode-segmentation] [![cat-text-processing-badge]][cat-text-processing]

Collect individual Unicode graphemes from UTF-8 string using the `[UnicodeSegmentation::graphemes]` function from the [`unicode-segmentation`][unicode-segmentation] crate.

```rust,editable
{{#include ../../deps/examples/graphemes.rs}}
```

## Implement the `FromStr` trait for a custom `struct`

[![std-badge]][std] [![cat-text-processing-badge]][cat-text-processing]

Creates a custom struct `RGB` and implements the `FromStr` trait to convert a provided color hex code into its RGB color code.

```rust,editable
{{#include ../../deps/examples/from_str.rs}}
```

[UnicodeSegmentation::graphemes]: https://docs.rs/unicode-segmentation/*/unicode_segmentation/trait.UnicodeSegmentation.html#tymethod.graphemes
[unicode-segmentation]: https://docs.rs/unicode-segmentation/1.2.1/unicode_segmentation/
{{#include ../refs/link-refs.md}}
