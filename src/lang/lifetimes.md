# Lifetimes

Prevent dangling references.

`&i32`        a reference
`&'a i32`     a reference with an explicit lifetime
`&'a mut i32` a mutable reference with an explicit lifetime

```rust,editable,ignore
let s: &'static str = "I have a static lifetime.";
```

The generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`:

```rust,editable
{{#include ../../deps/examples/generic_lifetime.rs}}
```

## Lifetime Annotations in Struct Definitions and methods

```rust,editable
{{#include ../../deps/examples/lifetime.rs}}
```

{{#include ../link-refs.md}}
