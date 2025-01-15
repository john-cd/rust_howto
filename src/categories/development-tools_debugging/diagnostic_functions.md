# Diagnostic functions

{{#include diagnostic_functions.incl.md}}

## Get the type name of the pointed-to value {#type-name-of-val}

[![std][c-std-badge]][c-std]{{hi:std}}

```rust,editable
{{#include ../../../crates/ex/categories/d/tests/development_tools_debugging/type_name_of_val.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P1 review](https://github.com/john-cd/rust_howto/issues/926)

- move to proper location
- cover [`std::any::type_name`][std::any::type_name]

[std::any::type_name]: https://doc.rust-lang.org/std/any/fn.type_name.html
</div>
