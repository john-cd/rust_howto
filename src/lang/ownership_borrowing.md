# Ownership and Borrowing

## Ownership

- No garbage collector. Ownership instead.
- Each value in Rust has an owner.
- There can only be one owner at a time.

```rust,editable
{{#include ../../deps/examples/ownership.rs}}
```

When the owner goes out of scope, the value will be dropped.

```rust,editable
{{#include ../../deps/examples/ownership2.rs}}
```

Rust will never automatically create “deep” copies of your data. Use `clone`

```rust,editable
{{#include ../../deps/examples/clone.rs}}
```

If a type implements the `Copy` trait (stack-only, fixed-size values, like integers, floats, and tuples thereof), variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

```rust,editable
{{#include ../../deps/examples/copy.rs}}
```

### Borrowing

Passing a variable to a function will move or copy, just as assignment does. To avoid passing a value along, borrow the value:

```rust,editable
{{#include ../../deps/examples/borrowing.rs}}
```

### Mutable reference

```rust,editable
{{#include ../../deps/examples/borrowing_mutable.rs}}
```

If you have a mutable reference to a value, you can have no other simultaneous references to that value! Functions like a read/write lock.

{{#include ../refs/link-refs.md}}
