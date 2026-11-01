<!-- TODO Reviewed ??-??-2026 -->

# String Manipulation

{{#include string_manipulation.incl.md}}

## Change the Case of a String with `heck` {#heck}

{{#badges heck no-std="No standard library"}}{{hi:Unicode}}{{hi:String}}{{hi:Case}}{{hi:Snake}}{{hi:Camel}}

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

{{#badges textwrap command-line-interface="Command-line interface" text-processing="Text processing"}}{{hi:Text}}{{hi:Typesetting}}{{hi:Wrap}}{{hi:Formatting}}{{hi:Hyphenation}}

[`textwrap`][c~textwrap~docs]↗{{hi:textwrap}} provides word wrapping, indenting, and dedenting strings. It has optional support for Unicode and emojis, as well as machine hyphenation.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/textwrap.rs:example}}
```

## `indoc` {#indoc}

{{#badges indoc no-std="No standard library" no-std::no-alloc="No dynamic allocation" rust-patterns="Rust patterns" text-processing="Text processing"}}{{hi:String}}{{hi:Literal}}{{hi:Heredoc}}{{hi:Nowdoc}}{{hi:Multiline}}

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

## Create a URL-Friendly Slug with `slug` {#slug}

{{#badges slug text-processing=}}

[`slug::slugify`][c~slug~docs]↗{{hi:slug}} turns a title into a lowercase ASCII string made of words separated by hyphens: it drops punctuation, collapses whitespace and transliterates accented characters. The result is suitable for URLs and stable references; two different titles can produce the same slug, so check for collisions if the slug is a key.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/slug.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
This chapter compares the old `Inflector` release with `heck`, adds tested title-case and snake-case conversions, and demonstrates Unicode-safe first-character capitalization.

</div>
