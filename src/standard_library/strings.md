# Strings

[![std][std-badge]][std]

## String type

```rust,editable
{{#include ../../deps/tests/strings.rs}}
```

## Placeholders

```rust,editable
{{#include ../../deps/tests/strings2.rs}}
```

Use `{:?}` to use the `Debug` output format (annotate type with `#[derive(Debug)]` ) or `{:#?}` for pretty print.

Also use `dbg!(&rect1);` for debug output (returns ownership of the expression’s value).

## String concatenation

Here are several common methods to concatenate Strings:

```rust,editable
{{#include ../../deps/tests/strings3.rs}}
```

Examples from [concatenation_benchmarks-rs]( https://github.com/hoodie/concatenation_benchmarks-rs )

{{#include ../refs/link-refs.md}}
