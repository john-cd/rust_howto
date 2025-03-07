# Functions {#functions}

{{#include functions.incl.md}}

[![Rust by example - Functions][book-rust-by-example-fn-badge]][book-rust-by-example-fn]{{hi:fn}}{{hi:Functions}}

```rust,editable
{{#include ../../crates/language/tests/feat/functions.rs:example}}
```

The unit type{{hi:Unit type}} `()`{{hi:()}} (`void` in some languages) is the default return type{{hi:Return type}} when no type is given for a function. It could be omitted: `fn log(message: &str) { ... }`

## Generic functions {#generic-functions}

```rust,editable
{{#include ../../crates/language/tests/feat/generic_functions.rs:example}}
```

```rust,editable
{{#include ../../crates/language/tests/feat/generic_functions2.rs:example}}
```

## Function pointers {#function-pointers}

```rust,editable
{{#include ../../crates/language/tests/feat/function_pointers.rs:example}}
```

## Diverging functions {#diverging-functions}

{{i:Diverging functions}} never return.

```rust,editable,should_panic
{{#include ../../crates/language/tests/feat/diverging_functions.rs:example}}
```

## Related Topics

- [[closures | Closures]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[functions: add desc](https://github.com/john-cd/rust_howto/issues/543)
</div>
