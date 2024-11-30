# Ownership and Borrowing

{{#include ownership_borrowing.incl.md}}

## Ownership {#ownership}

[![Rust by example - Ownership][book-rust-by-example-move-badge]][book-rust-by-example-move]{{hi:move}}

- No garbage collector{{hi:Garbage collector}}. {{i:Ownership}} instead.
- Each value in Rust has an owner{{hi:Ownership}}.
- There can only be one owner at a time.

```rust,editable
{{#include ../../deps/tests/language/ownership.rs:example}}
```

When the owner goes out of scope{{hi:Scope}}, the value will be dropped.

```rust,editable
{{#include ../../deps/tests/language/ownership2.rs:example}}
```

Rust will never automatically create “deep” copies of your data. Use [`std::clone::Clone`][c-std::clone::Clone]{{hi:std::clone::Clone}}⮳

```rust,editable
{{#include ../../deps/tests/language/clone.rs:example}}
```

If a type implements the [`std::marker::Copy`][c-std::marker::Copy]{{hi:std::marker::Copy}}⮳ trait (stack-only, fixed-size values, like integers, floats, and tuples thereof), variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

```rust,editable
{{#include ../../deps/tests/language/copy.rs:example}}
```

### Borrowing {#borrowing}

Passing a variable to a function will move or copy, just as assignment does. To avoid passing a value along, borrow the value:{{hi:Borrowing}}

```rust,editable
{{#include ../../deps/tests/language/borrowing.rs:example}}
```

### Mutable reference {#mutable-reference}

```rust,editable
{{#include ../../deps/tests/language/borrowing_mutable.rs:example}}
```

If you have a mutable reference{{hi:Mutable references}} to a value, you can have no other simultaneous references{{hi:Simultaneous references}} to that value! Functions like a read/write lock{{hi:Read/write lock}}.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO P1: add text

## Memory Safety {#memory-safety}

- [rust-re-borrowing-and-memory-safety][blog-rust-re-borrowing-and-memory-safety]⮳

</div>