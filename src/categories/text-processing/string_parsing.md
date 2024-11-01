# String Parsing

{{#include string_parsing.incl.md}}

## Collect Unicode Graphemes

[![unicode_segmentation][c-unicode_segmentation-badge]][c-unicode_segmentation]{{hi:unicode_segmentation}}  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Collect individual Unicode graphemes{{hi:Graphemes}} from UTF-8{{hi:UTF-8}} string using the [`unicode_segmentation::UnicodeSegmentation::graphemes`][c-unicode_segmentation::UnicodeSegmentation::graphemes]{{hi:unicode_segmentation::UnicodeSegmentation::graphemes}}⮳ function from the [`unicode_segmentation`][c-unicode_segmentation]{{hi:unicode_segmentation}}⮳ crate.

```rust
{{#include ../../../deps/tests/cats/text_processing/graphemes.rs:example}}
```

## Implement the `FromStr` trait for a custom `struct`

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Creates a custom struct `RGB` and implements the `FromStr`{{hi:FromStr}} trait to convert a provided color hex code into its RGB color code.

```rust
{{#include ../../../deps/tests/cats/text_processing/from_str.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
