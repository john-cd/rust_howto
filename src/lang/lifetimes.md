# Lifetimes

Prevent dangling references{{hi:dangling references}}.

`&i32`        a reference
`&'a i32`     a reference with an explicit lifetime
`&'a mut i32` a mutable reference with an explicit lifetime

```rust,editable
# fn main() {
let _s: &'static str = "I have a static lifetime.";
# }
```

The generic lifetime{{hi:lifetime}} `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`:

```rust,editable
{{#include ../../deps/tests/generic_lifetime.rs}}
```

## Lifetime Annotations in Struct Definitions and methods

```rust,editable
{{#include ../../deps/tests/lifetime.rs}}
```

{{#include ../refs/link-refs.md}}
