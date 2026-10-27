<!-- TODO Reviewed ??-??-2026 -->

# String Parsing

{{#include string_parsing.incl.md}}

## Implement the `FromStr` Trait for a Custom `struct` {#implement-the-fromstr-trait-for-a-custom-struct}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Creates an `Rgb` type and implements [`FromStr`][c~std::str::FromStr~docs]↗{{hi:FromStr}} to parse a string in the exact `#RRGGBB` form. The parser checks length, prefix, and ASCII before taking byte slices, so malformed or non-ASCII input returns an error instead of panicking. `FromStr` is a good fit when parsing does not need to borrow from the input; callers can use `.parse::<Rgb>()` or `Rgb::from_str`.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_parsing/from_str.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[parsing | Parsing]].
  - [[parse | Date and Time Parsing]].
  - Command-line [[argument_parsing | Argument Parsing]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  This review replaces unchecked byte slicing with explicit format and ASCII validation, gives parse failures a focused error type, and covers valid and malformed input in unit tests. The unrelated validator bookmark was dropped.
</div>
