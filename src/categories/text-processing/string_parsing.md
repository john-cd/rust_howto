# String Parsing

{{#include string_parsing.incl.md}}

## Collect unicode graphemes {#collect-unicode-graphemes}

[![unicode-segmentation][c-unicode_segmentation-badge]][c-unicode_segmentation] [![unicode-segmentation-crates.io][c-unicode_segmentation-crates.io-badge]][c-unicode_segmentation-crates.io] [![unicode-segmentation-github][c-unicode_segmentation-github-badge]][c-unicode_segmentation-github] [![unicode-segmentation-lib.rs][c-unicode_segmentation-lib.rs-badge]][c-unicode_segmentation-lib.rs]{{hi:unicode-segmentation}}{{hi:Boundary}}{{hi:Grapheme}}{{hi:Text}}{{hi:Unicode}}{{hi:Word}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Collect individual Unicode graphemes{{hi:Graphemes}} from UTF-8{{hi:UTF-8}} string using the [`unicode_segmentation::UnicodeSegmentation::graphemes`][c-unicode_segmentation::UnicodeSegmentation::graphemes]{{hi:unicode_segmentation::UnicodeSegmentation::graphemes}}⮳ function from the [`unicode_segmentation`][c-unicode_segmentation]{{hi:unicode_segmentation}}⮳ crate.

```rust,editable
{{#include ../../../crates/ex/categories/stuv/tests/text_processing/graphemes.rs:example}}
```

## Implement the `FromStr` trait for a custom `struct` {#implement-the-fromstr-trait-for-a-custom-struct}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

Creates a custom struct `RGB` and implements the `FromStr`{{hi:FromStr}} trait to convert a provided color hex code into its RGB color code.

```rust,editable
{{#include ../../../crates/ex/categories/stuv/tests/text_processing/from_str.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
