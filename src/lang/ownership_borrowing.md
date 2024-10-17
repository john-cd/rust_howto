# Ownership and Borrowing

## Ownership

- No garbage collector. Ownership instead.
- Each value in Rust has an owner{{hi:Ownership}}.
- There can only be one owner at a time.

```rust
{{#include ../../deps/tests/ownership.rs}}
```

When the owner goes out of scope{{hi:Scope}}, the value will be dropped.

```rust
{{#include ../../deps/tests/ownership2.rs}}
```

Rust will never automatically create “deep” copies of your data. Use [`std::clone::Clone`][c-std::clone::Clone]{{hi:std::clone::Clone}}⮳

```rust
{{#include ../../deps/tests/clone.rs}}
```

If a type implements the [`std::marker::Copy`][c-std::marker::Copy]{{hi:std::marker::Copy}}⮳ trait (stack-only, fixed-size values, like integers, floats, and tuples thereof), variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

```rust
{{#include ../../deps/tests/copy.rs}}
```

### Borrowing

Passing a variable to a function will move or copy, just as assignment does. To avoid passing a value along, borrow the value:

```rust
{{#include ../../deps/tests/borrowing.rs}}
```

### Mutable reference

```rust
{{#include ../../deps/tests/borrowing_mutable.rs}}
```

If you have a mutable reference to a value, you can have no other simultaneous references{{hi:Simultaneous references}} to that value! Functions like a read/write lock{{hi:Read/write lock}}.

{{#include ../refs/link-refs.md}}
<div class="hidden">
TODO: add text

## Memory Safety

- [rust-re-borrowing-and-memory-safety][blog-rust-re-borrowing-and-memory-safety]⮳

</div>
