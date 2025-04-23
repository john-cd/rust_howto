# Enums

{{#include enums.incl.md}}

[![Rust by example - Enums][book-rust-by-example-enums-badge]][book-rust-by-example-enums]{{hi:Enums}}

```rust,editable
{{#include ../../crates/language/tests/feat/enums.rs:example}}
```

If we make an enum{{hi:Enums}} public, all of its variants{{hi:Variants}} are then public. We only need [`pub`][book-rust-reference-visibility-and-privacy]{{hi:pub}}⮳ before the [`enum`][book-rust-reference-enum]⮳ keyword.

## `strum` {#strum}

[![strum][c-strum-badge]][c-strum] [![strum-crates.io][c-strum-crates.io-badge]][c-strum-crates.io] [![strum-github][c-strum-github-badge]][c-strum-github] [![strum-lib.rs][c-strum-lib.rs-badge]][c-strum-lib.rs]{{hi:strum}}{{hi:Enum}}{{hi:Macros}}{{hi:Proc-macros}}{{hi:String}} [![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}} [![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

[`strum`][c-strum]⮳{{hi:strum}} provides helpful macros for working with enums and strings.

## Common `enums` {#skip}

- [[option | Option]].
- [[result | Result]].

## Related Topics {#skip}

- [[match | Match]].
- [[rust-patterns | Rust Patterns]].
- [[functional_programming | Functional Programming]].
- [[data_types | Data Types]].
- [[structs | Structs]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[enums: edit NOW](https://github.com/john-cd/rust_howto/issues/542)
</div>
