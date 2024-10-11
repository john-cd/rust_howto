# String Parsing

{{#include string_parsing.incl.md}}

## Collect Unicode Graphemes

[![unicode_segmentation][c-unicode_segmentation-badge]][c-unicode_segmentation]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Collect individual Unicode {{i:graphemes}} from {{i:UTF-8}} string using the {{hi:UnicodeSegmentation::graphemes}}[`UnicodeSegmentation::graphemes`][c-unicode_segmentation::UnicodeSegmentation::graphemes]⮳ function from the {{hi:unicode_segmentation}}[`unicode_segmentation`][c-unicode_segmentation]⮳ crate.

```rust,editable
{{#include ../../../deps/tests/graphemes.rs}}
```

## Implement the `FromStr` trait for a custom `struct`

[![std][c-std-badge]][c-std]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Creates a custom struct `RGB` and implements the `FromStr`{{hi:FromStr}} trait to convert a provided color hex code into its RGB color code.

```rust,editable
{{#include ../../../deps/tests/from_str.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
