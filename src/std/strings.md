# Strings

## String type

```rust,editable
{{#include ../../deps/examples/strings.rs}}
```

## Placeholders

```rust,editable
{{#include ../../deps/examples/strings2.rs}}
```

Use `{:?}` to use the `Debug` output format (annotate type with `#[derive(Debug)]`) or `{:#?}` for pretty print.

Also use `dbg!(&rect1);` for debug output (returns ownership of the expression’s value).

{{#include ../link-refs.md}}
