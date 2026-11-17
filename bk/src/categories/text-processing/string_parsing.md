<!-- TODO Reviewed ??-??-2026. -->

# String Parsing

Use [`FromStr`][c~std::str::FromStr~docs]↗{{hi:FromStr}} when parsing creates an owned value instead of borrowing from the input. Its associated `Err` type lets the parser report useful failures, and it enables `"...".parse::<T>()`.

## Implement the `FromStr` Trait for a Custom `struct` {#implement-the-fromstr-trait-for-a-custom-struct}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Creates an `Rgb` type and implements [`FromStr`][c~std::str::FromStr~docs]↗{{hi:FromStr}} to parse a string in the exact `#RRGGBB` form. The parser checks length, prefix, and ASCII before taking byte slices, so malformed or non-ASCII input returns an error instead of panicking. `FromStr` is a good fit when parsing does not need to borrow from the input; callers can use `.parse::<Rgb>()` or `Rgb::from_str`.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_parsing/from_str.rs:example}}
```

Run from `bk/crates` to print the parsed components or run the example's valid and malformed input tests:

```sh
cargo run -p text_processing --example string_parsing
cargo test -p text_processing --example string_parsing
```

## Related Topics {#related-topics .skip}

- [[parsing | Parsing]].
  - [[parse | Date and Time Parsing]].
  - Command-line [[argument_parsing | Argument Parsing]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
