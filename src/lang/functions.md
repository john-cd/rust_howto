# Functions

[![Rust by example - Functions][book-rust-by-example-fn-badge]][book-rust-by-example-fn]{{hi:fn}}{{hi:Functions}}

```rust
{{#include ../../deps/tests/lang/functions.rs:example}}
```

The unit type{{hi:Unit type}} `()`{{hi:()}} (`void` in some languages) is the default return type{{hi:Return type}} when no type is given for a function. It could be omitted: `fn log(message: &str) { ... }`

## Generic functions {#generic-functions}

```rust
{{#include ../../deps/tests/lang/generic_functions.rs:example}}
```

```rust
{{#include ../../deps/tests/lang/generic_functions2.rs:example}}
```

## Function pointers {#function-pointers}

```rust
{{#include ../../deps/tests/lang/function_pointers.rs:example}}
```

## Diverging functions {#diverging-functions}

{{i:Diverging functions}} never return.

```rust,should_panic
{{#include ../../deps/tests/lang/diverging_functions.rs:example}}
```

{{#include ../refs/link-refs.md}}
<div class="hidden">
TODO: add desc
</div>
