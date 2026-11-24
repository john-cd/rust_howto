<!-- TODO Reviewed ??-??-2026. -->

# Ownership and Borrowing

{{#include ownership_and_borrowing.incl.md}}

Ownership determines who is responsible for cleaning up a value. It applies to local integers, heap-allocated strings, and resources such as open files. Where a value lives and who owns it are separate questions.

A function call typically uses a stack frame for local storage. When the function returns, its frame can be reclaimed as a unit. Heap allocations can outlive the call that created them if ownership passes to the caller or another part of the program.

Dynamically sized types such as `str` and `[T]` are usually accessed through a
pointer that carries metadata, such as the length of a slice. Their backing
storage can live on the stack, on the heap, or in static storage. A growable
collection such as `Vec<T>` has a fixed-size handle and can allocate or resize
its element buffer on the heap.

A heap allocator tracks which blocks are available for reuse. Languages with tracing garbage collection reclaim objects that are no longer reachable. The [Go GC guide][go~gc-guide]↗ describes one such implementation.

Rust uses ownership to decide when to drop a value and run its cleanup. The compiler checks moves and borrows statically. A type can implement shared ownership, as `Rc` and `Arc` do, with runtime reference counts that determine when to drop the shared value.

## Ownership {#ownership}

[![Rust by example - Ownership][book~rust-by-example~move~badge]][book~rust-by-example~move]{{hi:Move}}{{hi:Ownership}}{{hi:Scope}}

Rust's ownership system ensures automatic _memory safety_ without the need for a garbage collector.{{hi:Garbage collector}}

Ownership is a set of rules that the compiler enforces _at compile time_. If these rules are violated, the program won't compile:

1. **A value can be owned by a binding, a temporary, or another value.**
2. **A move transfers ownership of that value.** Several separately owned handles can share an underlying allocation.
3. **A value is normally dropped at the end of its drop scope.** Dropping runs its destructor if it has one and drops owned fields. Types that own allocations release them as part of cleanup.

A local binding is usually in scope from its declaration to the end of the enclosing block. A move can transfer its value before the block ends.

Rules 1 and 3 provide automatic cleanup when an owner leaves scope, and rule 2 prevents double frees. They do not prevent every leak: safe Rust can intentionally or accidentally keep allocations alive, for example through reference-counted cycles. Such leaks do not violate memory safety. The following example illustrates ownership and drop behavior:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/scope.rs:example}}
```

## Move Semantics {#move-semantics}

Assigning a non-`Copy` value to another variable transfers ownership. For example, `let s2 = s1;` moves a `String` into `s2`. The assignment transfers its buffer pointer, length, and capacity. The bytes of the string stay in the same allocation.

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/move1.rs:example}}
```

After the move, reading `s1` is a compile-time error. Passing a `String` to a function that takes it by value also transfers ownership:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/move_function.rs:example}}
```

## Duplicate a Value with `Clone` {#clone}

The `clone` method of the [`std::clone::Clone`][c~std::clone::Clone~docs]↗{{hi:std::clone::Clone}} trait explicitly duplicates a value. The type determines what is duplicated. Cloning a `String` duplicates its contents. Cloning an `Arc<String>` creates a new handle to the same allocation:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/clone.rs:example}}
```

Use `#[derive(Clone)]` to clone each field of a struct or enum. Implement `Clone` yourself when the type needs different behavior.

## Copy Semantics {#copy-semantics}

Types that implement the [`std::marker::Copy`][c~std::marker::Copy~docs]↗{{hi:std::marker::Copy}} trait are copied implicitly in places that would otherwise move a value. Examples include integers, floats, booleans, characters, and shared references. A shared reference can point to heap storage, and copying it still copies only the reference.

When you assign a variable of a `Copy` type to another, a simple _bitwise copy_ of the value is made, and _the original variable remains valid_.

The `Copy` trait is a marker trait, meaning it doesn't have any methods. Types that implement `Copy` must also implement `Clone`. A type cannot implement `Copy` if it or any of its parts implement the `Drop` trait (since it is used for custom cleanup, like deallocating heap memory).

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/copy.rs:example}}
```

## References and Borrowing {#references-borrowing}

A _reference_ borrows a value from its owner. Rust checks that the value remains valid for as long as the reference can be used. A shared reference (`&T`) lets multiple readers access the same value:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/borrowing_immutable.rs:example}}
```

An exclusive reference (`&mut T`) lets its holder modify the borrowed value:

```rust,noplayground
{{#include ../../crates/language/examples/ownership_borrowing/mutate_through_reference.rs:example}}
```

A mutable reference grants exclusive access to the data it borrows. A reborrow can temporarily lend that access to a new reference. The original reference becomes usable again after the reborrow ends. Shared references normally allow only shared reads, with controlled mutation available through types such as `Cell` and `Mutex`. Rust checks the borrowing rules at compile time. A `RwLock` additionally coordinates access at runtime across threads.{{hi:Simultaneous references}}{{hi:Read/write lock}}

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/read_write_lock.rs:example}}
```

This strict rule prevents data races, which occur when:

- Two or more pointers (or references) access the same data concurrently,
- At least one of them is a write,
- There's no synchronization mechanism being used to control the access.

Data races lead to undefined behavior, which can manifest as crashes, incorrect results, or subtle bugs that are hard to track down.

### Borrowing when Calling a Function {#borrowing}

Pass a shared reference when a function needs to inspect a value and the caller needs to keep it. In this example, `calculate` takes a `&MyStruct`, so the caller still owns the struct after the call returns.{{hi:Borrowing}}

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/borrowing_function.rs:example}}
```

The same applies to mutable references:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/borrowing_function_mutable.rs:example}}
```

## References {#references .skip}

- [Rust Re-borrowing and Memory Safety][blog~rust-re-borrowing-and-memory-safety]↗.

## Related Topics {#related-topics .skip}

- [[lifetimes | Lifetimes]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

- [Rust Patterns](../categories/rust-patterns/index.md).

</div>
