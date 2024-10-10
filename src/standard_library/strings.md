# Strings

{{#include strings.incl.md}}

## String type

[![std][std-badge]][c-std]

```rust,editable
{{#include ../../deps/tests/strings.rs}}
```

## Placeholders

```rust,editable
{{#include ../../deps/tests/strings2.rs}}
```

Use `{:?}` to use the [`{{i:Debug}}`][c-std::fmt::Debug]⮳ {{i:output format}} (annotate type with `#[derive(Debug)]` ) or `{:#?}` for {{i:pretty print}}.

Also use `dbg!(&rect1);` for {{i:debug output}} (returns ownership of the expression’s value).

## String concatenation

Here are several common methods to {{i:concatenate}} Strings:

```rust,editable
{{#include ../../deps/tests/strings3.rs}}
```

Examples from [concatenation_benchmarks-rs]( https://github.com/hoodie/concatenation_benchmarks-rs )⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
