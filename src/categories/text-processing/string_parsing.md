# String Parsing

{{#include string_parsing.incl.md}}

## Collect Unicode Graphemes

[![unicode-segmentation][unicode-segmentation-badge]][unicode-segmentation]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Collect individual Unicode {{i:graphemes}} from {{i:UTF-8}} string using the [`UnicodeSegmentation::graphemes`][unicode_segmentation::UnicodeSegmentation::graphemes]⮳ function from the [`unicode-segmentation`][unicode-segmentation]⮳ crate.

```rust,editable
{{#include ../../../deps/tests/graphemes.rs}}
```

## Implement the `FromStr` trait for a custom `struct`

[![std][std-badge]][std]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Creates a custom struct `RGB` and implements the `{{i:FromStr}}` trait to convert a provided color hex code into its RGB color code.

```rust,editable
{{#include ../../../deps/tests/from_str.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
