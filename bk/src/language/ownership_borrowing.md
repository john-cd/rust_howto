# Ownership and Borrowing

{{#include ownership_borrowing.incl.md}}

## Ownership {#ownership}

[![Rust by example - Ownership][book-rust-by-example-move-badge]][book-rust-by-example-move]{{hi:move}}{{hi:Ownership}}

Rust's ownership system ensures memory safety without needing a garbage collector.{{hi:Garbage collector}} It is a set of rules that the compiler enforces at compile time:

- Each value in Rust has a variable that is its owner{{hi:Ownership}}.
- There can only be one owner at a time.

```rust,editable
{{#include ../../crates/language/tests/ownership_borrowing/ownership.rs:example}}
```

When the owner goes out of scope{{hi:Scope}}, the value is dropped.

```rust,editable
{{#include ../../crates/language/tests/ownership_borrowing/ownership2.rs:example}}
```

Rust will never automatically create deep copies of your data. Use the [`std::clone::Clone`][c-std::clone::Clone]{{hi:std::clone::Clone}}⮳ trait to explicitly create a deep copy.

```rust,editable
{{#include ../../crates/language/tests/ownership_borrowing/clone.rs:example}}
```

The `Copy` trait is a marker trait, meaning it doesn't have any methods. It's used to indicate that a type can be copied by simply copying its bits.

If a type implements the [`std::marker::Copy`][c-std::marker::Copy]{{hi:std::marker::Copy}}⮳ trait (which is the case for stack-only, fixed-size values, like integers, floats, and tuples thereof), variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable. Types that implement `Copy` are implicitly `Clone`.

```rust,editable
{{#include ../../crates/language/tests/ownership_borrowing/copy.rs:example}}
```

### Borrowing {#borrowing}

Passing a variable to a function will move or copy, just as assignment does. To avoid transferring ownership, you can "borrow" the value:{{hi:Borrowing}}

```rust,editable
{{#include ../../crates/language/tests/ownership_borrowing/borrowing.rs:example}}
```

### Mutable References {#mutable-reference}

Mutable references allow you to modify the borrowed value. However, there are strict rules to prevent data races:

If you have a mutable reference{{hi:Mutable references}} to a value, you can have no other simultaneous references{{hi:Simultaneous references}} to that value! References function like a read/write lock{{hi:Read/write lock}}.

```rust,editable
{{#include ../../crates/language/tests/ownership_borrowing/borrowing_mutable.rs:example}}
```

## Related Topics {#skip}

- [[lifetimes | Lifetimes]].
- [[rust-patterns | Rust Patterns]].

## References {#skip}

- [Rust re-borrowing and memory safety][blog-rust-re-borrowing-and-memory-safety]⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[ownership_borrowing: add text NOW](https://github.com/john-cd/rust_howto/issues/554)
</div>
