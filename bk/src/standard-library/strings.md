# Strings

{{#include strings.incl.md}}

## `String` {#string}

[![std][c-std-badge]][c-std]{{hi:std}}{{hi:Strings}}

```rust,editable
{{#include ../../crates/standard_library/tests/other/strings.rs:example}}
```

## Placeholders {#placeholders}

[![std][c-std-badge]][c-std]

```rust,editable
{{#include ../../crates/standard_library/tests/other/strings2.rs:example}}
```

Use `{:?}` to use the [`std::fmt::Debug`][c-std::fmt::Debug]{{hi:std::fmt::Debug}}⮳ output format{{hi:Output format}} (annotate type with `#[derive(Debug)]` ) or `{:#?}` for pretty print{{hi:Pretty print}}.

Also use `dbg!(&rect1);` for debug output{{hi:Debug output}} (returns ownership of the expression's value).

## Concatenate strings {#string-concatenation}

[![std][c-std-badge]][c-std]

Here are several common methods to concatenate{{hi:Concatenation}} [`String`][c-std::string::String]s⮳{{hi:String}}:

```rust,editable
{{#include ../../crates/standard_library/tests/strings3.rs:example}}
```

Examples from [concatenation_benchmarks-rs][concatenation_benchmarks-github]⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[strings: review (P1)](https://github.com/john-cd/rust_howto/issues/630)

String manipulation, formatting, searching, replacing, regular expressions, Unicode handling, and working with different string types (String, &str, OsString).
</div>
