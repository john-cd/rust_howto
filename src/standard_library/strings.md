# Strings

{{#include strings.incl.md}}

## String type

[![std][c-std-badge]][c-std]{{hi:std}}{{hi:Strings}}

```rust
{{#include ../../deps/tests/std/strings.rs:example}}
```

## Placeholders

![std][c-std-badge]][c-std]

```rust
{{#include ../../deps/tests/std/strings2.rs:example}}
```

Use `{:?}` to use the [`std::fmt::Debug`][c-std::fmt::Debug]{{hi:std::fmt::Debug}}⮳ output format{{hi:Output format}} (annotate type with `#[derive(Debug)]` ) or `{:#?}` for pretty print{{hi:Pretty print}}.

Also use `dbg!(&rect1);` for debug output{{hi:Debug output}} (returns ownership of the expression’s value).

## String concatenation

![std][c-std-badge]][c-std]

Here are several common methods to concatenate{{hi:Concatenation}} Strings:

```rust
{{#include ../../deps/tests/std/strings3.rs:example}}
```

Examples from [concatenation_benchmarks-rs][concatenation_benchmarks-github]⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}
<div class="hidden">
TODO: review
</div>
