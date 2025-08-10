# String Parsing

{{#include string_parsing.incl.md}}

## Implement the `FromStr` Trait for a Custom `struct` {#implement-the-fromstr-trait-for-a-custom-struct}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

Creates a custom struct `RGB` and implements the [`FromStr`][c~std::str::FromStr~docs]↗{{hi:FromStr}} trait to convert a provided color hex code into its RGB color code.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_parsing/from_str.rs:example}}
```

## Related Topics

- [[parsing | Parsing]].
  - [[parse | Date and Time Parsing]].
  - Command-line [[argument_parsing | Argument Parsing]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/965)

- [validator](https://lib.rs/crates/validator)

[`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)↗ provides _string parsing_ with idiomatic error handling, and it's typically implemented for types like [`u32`](https://doc.rust-lang.org/stable/std/primitive.u32.html)↗{{hi:u32}}, [`Url`](https://docs.rs/url/latest/url/struct.Url.html)↗{{hi:url::Url}}, and [`enum`](https://doc.rust-lang.org/std/keyword.enum.html)↗{{hi:enum}}s. It powers `.parse::<T>()`. It can only parse types that do not contain a lifetime parameter.

</div>
