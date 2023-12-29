# Functions

```rust,editable
{{#include ../../deps/examples/functions.rs}}
```

The unit type `()` (`void` in some languages) is the default return type when no type is given for a function.
It could be omitted: `fn log(message: &str) { ... }`

## Generic functions

```rust,editable
{{#include ../../deps/examples/generic_functions.rs}}
```

```rust,editable
{{#include ../../deps/examples/generic_functions2.rs}}
```

## Function pointers

```rust,editable
{{#include ../../deps/examples/function_pointers.rs}}
```

## Diverging functions

Diverging functions never return.

```rust,editable,should_panic
{{#include ../../deps/examples/diverging_functions.rs}}
```
