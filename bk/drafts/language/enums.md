# Enums

{{#include enums.incl.md}}

## Enum Syntax {#skip}

[![Rust by example - Enums][book-rust-by-example-enums-badge]][book-rust-by-example-enums]{{hi:Enums}}{{hi:Variants}}

Enums are custom data types that define a set of possible 'variants' or states that a value can be. A value of that enum type can only be one of those variants at any given time. Each variant can optionally hold associated data (fields) of different types like a `struct` or like a `tuple`:

```rust,editable
{{#include ../../crates/language/tests/enums/enums.rs:example}}
```

If we make an enum{{hi:Enums}} public, all of its variants are then public. We only need [`pub`][book-rust-reference-visibility-and-privacy]{{hi:pub}}⮳ before the [`enum`][book-rust-reference-enum]⮳ keyword.

### Common `enums` {#skip}

- [[option | Option]].
- [[result | Result]].

## Convert between Strings and Enum Variants with `strum` {#strum}

[![strum][c-strum-badge]][c-strum] [![strum-crates.io][c-strum-crates.io-badge]][c-strum-crates.io] [![strum-github][c-strum-github-badge]][c-strum-github] [![strum-lib.rs][c-strum-lib.rs-badge]][c-strum-lib.rs]{{hi:strum}}{{hi:Enum}}{{hi:Macros}}{{hi:Proc-macros}}{{hi:String}} [![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}} [![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

The [`strum`][c-strum]⮳{{hi:strum}} crate provides helpful macros for working with enums and strings. It also can convert from an integer to an enum, add custom properties to enum variants, etc.

```rust,editable
{{#include ../../crates/language/tests/enums/strum.rs:example}}
```

## Related Topics {#skip}

- [[match | Match]].
- [[rust-patterns | Rust Patterns]].
- [[functional_programming | Functional Programming]].
- [[data_types | Data Types]].
- [[structs | Structs]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
