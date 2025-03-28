# Strings

{{#include strings.incl.md}}

## `String` {#string}

[![std][c-std-badge]][c-std]{{hi:std}}{{hi:Strings}}

```rust,editable
{{#include ../../crates/standard_library/tests/other/strings.rs:example}}
```

## String Formatting; Placeholders {#placeholders}

[![std][c-std-badge]][c-std]

```rust,editable
{{#include ../../crates/standard_library/tests/other/strings2.rs:example}}
```

Use `{:?}` to use the [`std::fmt::Debug`][c-std::fmt::Debug]{{hi:std::fmt::Debug}}⮳ output format{{hi:Output format}} (annotate type with `#[derive(Debug)]` ) or `{:#?}` for pretty print{{hi:Pretty print}}.

Also use `dbg!(&rect1);` for debug output{{hi:Debug output}} (returns ownership of the expression's value).

## Concatenate Strings {#string-concatenation}

[![std][c-std-badge]][c-std]

Here are several common methods to concatenate{{hi:Concatenation}} [`String`][c-std::string::String]s⮳{{hi:String}}:

```rust,editable
{{#include ../../crates/standard_library/tests/strings3.rs:example}}
```

Examples from [concatenation_benchmarks-rs][concatenation_benchmarks-github]⮳.

## Related Data Structures {#skip}

- [[slices | Slices]].
- [[vectors | Vectors]].
- [[cow | COW]].

## String Manipulation {#skip}

- [[regex | Regex (Regular Expressions)]].
- [[parsing | Parsing]].
- [[text-processing | Text Processing]].
- [[string_parsing | String Parsing]].
- [[string_concat | String Concatenation]].

## Related Topics {#skip}

- [[algorithms | Algorithms]].
- [[encoding | Encoding]].
- [[internationalization | Internationalization]].
- [[localization | Localization]].
- [[search | Search]].
- [[rust_search_engines | Rust Search Engines]].
- [[template-engine | Template Engine]].
- [[text_layout | Text Layout]].
- [[unicode | Unicode]] handling.
- [[value-formatting | Value Formatting]].
- Working with [[other_strings | Other Strings]] (CString, OsString).

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[strings: add](https://github.com/john-cd/rust_howto/issues/630)
</div>
