# Strings

{{#include strings.incl.md}}

## String type

[![std][c-std-badge]][c-std]

```rust
{{#include ../../deps/tests/strings.rs}}
```

## Placeholders

```rust
{{#include ../../deps/tests/strings2.rs}}
```

Use `{:?}` to use the [`std::fmt::Debug`][c-std::fmt::Debug]{{hi:std::fmt::Debug}}⮳ output format{{hi:output format}} (annotate type with `#[derive(Debug)]` ) or `{:#?}` for pretty print{{hi:Pretty print}}.

Also use `dbg!(&rect1);` for debug output{{hi:Debug output}} (returns ownership of the expression’s value).

## String concatenation

Here are several common methods to concatenate{{hi:Concatenation}} Strings:

```rust
{{#include ../../deps/tests/strings3.rs}}
```

Examples from [concatenation_benchmarks-rs][concatenation_benchmarks-github]⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
<div class="hidden">
TODO: review
</div>
