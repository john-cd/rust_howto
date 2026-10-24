<!-- TODO Reviewed ??-??-2026 -->

# String Manipulation

{{#include string_manipulation.incl.md}}

## Change the Case of a String with `heck` {#heck}

[![heck][c~heck~docs~badge]][c~heck~docs] [![heck~crates.io][c~heck~crates.io~badge]][c~heck~crates.io] [![heck~repo][c~heck~repo~badge]][c~heck~repo] [![heck~lib.rs][c~heck~lib.rs~badge]][c~heck~lib.rs]{{hi:heck}}{{hi:Unicode}}{{hi:String}}{{hi:Case}}{{hi:Snake}}{{hi:Camel}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`heck`][c~heck~docs]↗{{hi:heck}} is a case conversion library. This library exists to provide case conversion between common cases like `CamelCase` and `snake_case`.

The older [`Inflector`][c~inflector~crates.io] crate's latest crates.io release is `0.11.4` from January 2019, so prefer `heck` for common case conversions. Rust strings are UTF-8: the example below uppercases the first Unicode scalar value without assuming it occupies one byte. The [capitalization discussion][blog~stackoverflow~why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust]↗ explains the indexing issue.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/heck.rs:example}}
```

### Capitalize the First Unicode Character {#case-conversion}

This helper uses `char::to_uppercase`, which can produce more than one character, and leaves the remaining characters intact. It handles an empty string and avoids slicing through a multibyte UTF-8 character.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/case_conversion.rs:example}}
```

## `textwrap` {#textwrap}

[![textwrap][c~textwrap~docs~badge]][c~textwrap~docs] [![textwrap~crates.io][c~textwrap~crates.io~badge]][c~textwrap~crates.io] [![textwrap~repo][c~textwrap~repo~badge]][c~textwrap~repo] [![textwrap~lib.rs][c~textwrap~lib.rs~badge]][c~textwrap~lib.rs]{{hi:textwrap}}{{hi:Text}}{{hi:Typesetting}}{{hi:Wrap}}{{hi:Formatting}}{{hi:Hyphenation}} [![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line interface}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

[`textwrap`][c~textwrap~docs]↗{{hi:textwrap}} provides word wrapping, indenting, and dedenting strings. It has optional support for Unicode and emojis, as well as machine hyphenation.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/textwrap.rs:example}}
```

## `indoc` {#indoc}

[![indoc][c~indoc~docs~badge]][c~indoc~docs] [![indoc~crates.io][c~indoc~crates.io~badge]][c~indoc~crates.io] [![indoc~repo][c~indoc~repo~badge]][c~indoc~repo] [![indoc~lib.rs][c~indoc~lib.rs~badge]][c~indoc~lib.rs]{{hi:indoc}}{{hi:String}}{{hi:Literal}}{{hi:Heredoc}}{{hi:Nowdoc}}{{hi:Multiline}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

The [`indoc!()`][c~indoc::indoc~docs]↗{{hi:indoc::indoc!}} macro takes a multiline string literal and un-indents it at compile time so the leftmost non-space character is in the first column.
The [`indoc`][c~indoc~docs]↗{{hi:indoc}} crate exports five additional macros to substitute conveniently for the standard library's formatting macros:

`formatdoc!($fmt, ...)` equivalent to `format!(indoc!($fmt), ...)`
`printdoc!($fmt, ...)` equivalent to `print!(indoc!($fmt), ...)`
`eprintdoc!($fmt, ...)` equivalent to `eprint!(indoc!($fmt), ...)`
`writedoc!($dest, $fmt, ...)` equivalent to `write!($dest, indoc!($fmt), ...)`
`concatdoc!(...)` equivalent to `concat!(...)` with each string literal wrapped in `indoc!`

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/indoc.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
This chapter compares the old `Inflector` release with `heck`, adds tested title-case and snake-case conversions, and demonstrates Unicode-safe first-character capitalization.

</div>
