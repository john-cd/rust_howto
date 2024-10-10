# String Parsing

{{#include string_parsing.incl.md}}

## Collect Unicode Graphemes

[![unicode-segmentation][c-unicode-segmentation-badge]][c-unicode-segmentation]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Collect individual Unicode {{i:graphemes}} from {{i:UTF-8}} string using the [`{{i:UnicodeSegmentation::graphemes}}`][c-unicode_segmentation::UnicodeSegmentation::graphemes]⮳ function from the [`{{i:unicode-segmentation}}`][c-unicode-segmentation]⮳ crate.

```rust,editable
{{#include ../../../deps/tests/graphemes.rs}}
```

## Implement the `FromStr` trait for a custom `struct`

[![std][c-std-badge]][c-std]  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]

Creates a custom struct `RGB` and implements the `{{i:FromStr}}` trait to convert a provided color hex code into its RGB color code.

```rust,editable
{{#include ../../../deps/tests/from_str.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
