# Functions {#functions}

{{#include functions.incl.md}}

[![Rust by example - Functions][book-rust-by-example-fn-badge]][book-rust-by-example-fn]{{hi:fn}}{{hi:Functions}}

```rust,editable
{{#include ../../deps/tests/language/functions.rs:example}}
```

The unit type{{hi:Unit type}} `()`{{hi:()}} (`void` in some languages) is the default return type{{hi:Return type}} when no type is given for a function. It could be omitted: `fn log(message: &str) { ... }`

## Generic functions {#generic-functions}

```rust,editable
{{#include ../../deps/tests/language/generic_functions.rs:example}}
```

```rust,editable
{{#include ../../deps/tests/language/generic_functions2.rs:example}}
```

## Function pointers {#function-pointers}

```rust,editable
{{#include ../../deps/tests/language/function_pointers.rs:example}}
```

## Diverging functions {#diverging-functions}

{{i:Diverging functions}} never return.

```rust,editable,should_panic
{{#include ../../deps/tests/language/diverging_functions.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO P1: add desc
</div>
