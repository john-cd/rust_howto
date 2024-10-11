# Functions

```rust,editable
{{#include ../../deps/tests/functions.rs}}
```

The unit type{{hi:unit type}} `()`{{hi:()}} (`void` in some languages) is the default return type{{hi:return type}} when no type is given for a function. It could be omitted: `fn log(message: &str) { ... }`

## Generic functions

```rust,editable
{{#include ../../deps/tests/generic_functions.rs}}
```

```rust,editable
{{#include ../../deps/tests/generic_functions2.rs}}
```

## Function pointers

```rust,editable
{{#include ../../deps/tests/function_pointers.rs}}
```

## Diverging functions

Diverging functions never return.

```rust,editable,should_panic
{{#include ../../deps/tests/diverging_functions.rs}}
```

{{#include ../refs/link-refs.md}}
