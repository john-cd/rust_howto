# String Manipulation

{{#include string_manipulation.incl.md}}

## `heck` {#heck}

[![heck][c-heck-badge]][c-heck] [![heck-crates.io][c-heck-crates.io-badge]][c-heck-crates.io] [![heck-github][c-heck-github-badge]][c-heck-github] [![heck-lib.rs][c-heck-lib.rs-badge]][c-heck-lib.rs]{{hi:heck}}{{hi:Unicode}}{{hi:String}}{{hi:Case}}{{hi:Snake}}{{hi:Camel}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[`heck`][c-heck]⮳{{hi:heck}} is a case conversion library. This library exists to provide case conversion between common cases like `CamelCase` and `snake_case`.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/heck.rs:example}}
```

## `textwrap` {#textwrap}

[![textwrap][c-textwrap-badge]][c-textwrap] [![textwrap-crates.io][c-textwrap-crates.io-badge]][c-textwrap-crates.io] [![textwrap-github][c-textwrap-github-badge]][c-textwrap-github] [![textwrap-lib.rs][c-textwrap-lib.rs-badge]][c-textwrap-lib.rs]{{hi:textwrap}}{{hi:Text}}{{hi:Typesetting}}{{hi:Wrap}}{{hi:Formatting}}{{hi:Hyphenation}} [![cat-command-line-interface][cat-command-line-interface-badge]][cat-command-line-interface]{{hi:Command-line interface}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

[`textwrap`][c-textwrap]⮳{{hi:textwrap}} provides word wrapping, indenting, and dedenting strings. It has optional support for Unicode and emojis, as well as machine hyphenation.

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/textwrap.rs:example}}
```

## `indoc` {#indoc}

[![indoc][c-indoc-badge]][c-indoc] [![indoc-crates.io][c-indoc-crates.io-badge]][c-indoc-crates.io] [![indoc-github][c-indoc-github-badge]][c-indoc-github] [![indoc-lib.rs][c-indoc-lib.rs-badge]][c-indoc-lib.rs]{{hi:indoc}}{{hi:String}}{{hi:Literal}}{{hi:Heredoc}}{{hi:Nowdoc}}{{hi:Multiline}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}} [![cat-text-processing][cat-text-processing-badge]][cat-text-processing]{{hi:Text processing}}

The `indoc!()` macro takes a multiline string literal and un-indents it at compile time so the leftmost non-space character is in the first column.

The [`indoc`][c-indoc]⮳{{hi:indoc}} crate exports five additional macros to substitute conveniently for the standard library's formatting macros:

`formatdoc!($fmt, ...)` — equivalent to `format!(indoc!($fmt), ...)`
`printdoc!($fmt, ...)` — equivalent to `print!(indoc!($fmt), ...)`
`eprintdoc!($fmt, ...)` — equivalent to `eprint!(indoc!($fmt), ...)`
`writedoc!($dest, $fmt, ...)` — equivalent to `write!($dest, indoc!($fmt), ...)`
`concatdoc!(...)` — equivalent to `concat!(...)` with each string literal wrapped in `indoc!`

```rust,editable
{{#include ../../../crates/cats/text_processing/examples/string_manipulation/indoc.rs:example}}
```

## Related Topics

- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1190)

cover inflector

```rust,noplayground
use inflector::Inflector;

fn main() {
  let capitalized = "some string".to_title_case();
}
```

- [Why is capitalizing the first letter of a string so convoluted in Rust? - Stack Overflow](https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust)

</div>
