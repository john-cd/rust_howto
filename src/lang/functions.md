# Functions

```rust
{{#include ../../deps/tests/functions.rs}}
```

The unit type{{hi:Unit type}} `()`{{hi:()}} (`void` in some languages) is the default return type{{hi:Return type}} when no type is given for a function. It could be omitted: `fn log(message: &str) { ... }`

## Generic functions

```rust
{{#include ../../deps/tests/generic_functions.rs}}
```

```rust
{{#include ../../deps/tests/generic_functions2.rs}}
```

## Function pointers

```rust
{{#include ../../deps/tests/function_pointers.rs}}
```

## Diverging functions

Diverging functions never return.

```rust,should_panic
{{#include ../../deps/tests/diverging_functions.rs}}
```

{{#include ../refs/link-refs.md}}
<div class="hidden">
TODO: add desc
</div>
