# Lifetimes

Prevent dangling references{{hi:Dangling references}}.

`&i32`        a reference
`&'a i32`     a reference with an explicit lifetime
`&'a mut i32` a mutable reference with an explicit lifetime

```rust
{{#include ../../deps/tests/static_lifetime.rs}}
```

The generic lifetime{{hi:Lifetime}} `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`:

```rust
{{#include ../../deps/tests/generic_lifetime.rs}}
```

## Lifetime Annotations in Struct Definitions and methods

```rust
{{#include ../../deps/tests/lifetime.rs}}
```

{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: review
</div>
