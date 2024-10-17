# Lifetimes

Prevent dangling references{{hi:Dangling references}}.

`&i32`        a reference
`&'a i32`     a reference with an explicit lifetime
`&'a mut i32` a mutable reference with an explicit lifetime

```rust
// 'static indicates that the data pointed to by the reference lives for the _remaining_ lifetime of the running program.
// It can still be coerced to a shorter lifetime.
fn my_string() -> &'static str {
  let s: &'static str = "I have a static lifetime.";
  s
}

fn main() {
  println!("{}", my_string());
}
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
