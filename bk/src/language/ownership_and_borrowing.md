<!-- TODO Reviewed ??-??-2026. -->

# Ownership and Borrowing

{{#include ownership_and_borrowing.incl.md}}

The memory of a computer program is primarily structured into a _stack_ and a _heap_. The stack is structured: it stores values in the order it gets them and removes the values in the opposite order ("last in, first out"). During program execution, the stack is used to store function parameters, non-static local variables, function return values and return addresses, or pointers to the heap.

Under the covers, most CPUs maintain a pointer to the top of the stack in one of its internal registers. Pushing data onto the stack is a matter of copying a value onto the next available memory slot. Popping data off the stack is simply adjusting the stack pointer register value. Therefore, the stack is fast, but it suffers from one major constraint: _all data stored on the stack must have a known, fixed size_.

Dynamically sized types such as `str` and `[T]` are usually accessed through a
pointer that carries metadata, such as the length of a slice. Their backing
storage can live on the stack, on the heap, or in static storage. A growable
collection such as `Vec<T>` has a fixed-size handle and can allocate or resize
its element buffer on the heap.

Allocating space on the heap is slower than pushing data on the stack, because the allocator must find a large enough block and perform bookkeeping for the next allocation. It tracks which blocks are in use so allocations do not overlap; if a program fails to release an allocation, that memory can remain unavailable until the process exits. The heap can also suffer from _fragmentation_, when free space is split into small chunks among allocated blocks, leaving no large enough block for a request and potentially causing an out-of-memory error.

Some languages (Java, C#, Go...) use _garbage collection_ (GC) to reclaim unreachable memory as the program runs. Some collectors compact the heap by moving live objects; others, including Go's non-moving mark-sweep collector, do not. GC is automatic and memory-safe, but it has runtime costs and may pause the program or run concurrently with it ([Go GC guide][go~gc-guide]↗).

In other languages (C, C++...), the programmer must explicitly allocate and free the memory. This is very error-prone: each allocation must be paired with exactly one deallocation. Forgetting to free memory leaks it. Freeing memory too early or more than once ("double free" error) may lead to memory overlap and invalid variables, likely causing a program crash.

Rust uses ownership to decide when to drop a value and run its cleanup. The compiler checks moves and borrows statically. A type can implement shared ownership, as `Rc` and `Arc` do, with runtime reference counts that determine when to drop the shared value.

## Ownership {#ownership}

[![Rust by example - Ownership][book~rust-by-example~move~badge]][book~rust-by-example~move]{{hi:Move}}{{hi:Ownership}}{{hi:Scope}}

Rust's ownership system ensures automatic _memory safety_ without the need for a garbage collector.{{hi:Garbage collector}}

Ownership is a set of rules that the compiler enforces _at compile time_. If these rules are violated, the program won't compile:

1. **A value can be owned by a binding, a temporary, or another value.**
2. **A move transfers ownership of that value.** Several separately owned handles can share an underlying allocation.
3. **A value is normally dropped at the end of its drop scope.** Dropping runs its destructor if it has one and drops owned fields. Types that own allocations release them as part of cleanup.

The _scope_ of a variable is defined as follows: A variable is valid from the point it is declared until the end of its current scope (usually a block of code enclosed in curly braces `{ }` and typically the body of a function).

Rules 1 and 3 provide automatic cleanup when an owner leaves scope, and rule 2 prevents double frees. They do not prevent every leak: safe Rust can intentionally or accidentally keep allocations alive, for example through reference-counted cycles. Such leaks do not violate memory safety. The following example illustrates ownership and drop behavior:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/scope.rs:example}}
```

## Move Semantics {#move-semantics}

When you assign a heap-allocated value (like a `String`) from one variable to another, Rust _moves_ the ownership of the value: the new variable owns the data, and _the original variable becomes invalid_.

Moving enforces rule 2 above: there can only be one owner of a value.

This is very different from what is common in other programming languages: they perform either a "shallow copy" (copying the pointer to the heap data) or a "deep copy" (copying the heap data to another heap location and storing a pointer to the new location in the new variable). Shallow copies are cheap but result in multiple owners of the heap data. Deep copy is safe but is expensive, since it involves allocating an arbitrary number of bytes on the heap.

In contrast, Rust's move is like a shallow copy, but with the added rule of _invalidating the original owner_. It is therefore cheap and safe.

The following example illustrates this concept:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/move1.rs:example}}
```

Let's explain what happens behind the scene: The local variable `s1` (on the stack) contains a pointer to the string's heap-allocated data (here, the unicode characters of the String). During the assignment `let s2 = s1`, the pointer is copied into `s2` (also on the stack). The heap data is not touched.

Most importantly, the `s1` variable is made inaccessible. That means that, during compilation, the Rust compiler made sure that `s1` could not be referenced by any line of code after the "move" event (or the program would simply not compile). Note that there are no runtime checks, just purely compiler-enforced rules.

Assignment (of non-`Copy` values) is not the only event that triggers a move: passing a variable to a function does as well:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/move_function.rs:example}}
```

## Duplicate a Value with `Clone` {#clone}

Rust will never automatically create deep copies of your data, because, as described above, they can be expensive.

The `clone` method of the [`std::clone::Clone`][c~std::clone::Clone~docs]↗{{hi:std::clone::Clone}} trait explicitly duplicates a value. The type determines what is duplicated. Cloning a `String` duplicates its contents. Cloning an `Arc<String>` creates a new handle to the same allocation:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/clone.rs:example}}
```

You can implement the `Clone` trait for your custom types (structs, enums...) to provide any type-specific behavior necessary to duplicate values safely.
However, you will often simply add the `#[derive(Clone)]` attribute to have the compiler automagically implement the `Clone` trait for you.

## Copy Semantics {#copy-semantics}

Types that implement the [`std::marker::Copy`][c~std::marker::Copy~docs]↗{{hi:std::marker::Copy}} trait are copied implicitly in places that would otherwise move a value. Examples include integers, floats, booleans, characters, and shared references. A shared reference can point to heap storage, and copying it still copies only the reference.

When you assign a variable of a `Copy` type to another, a simple _bitwise copy_ of the value is made, and _the original variable remains valid_.

The `Copy` trait is a marker trait, meaning it doesn't have any methods. Types that implement `Copy` must also implement `Clone`. A type cannot implement `Copy` if it or any of its parts implement the `Drop` trait (since it is used for custom cleanup, like deallocating heap memory).

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/copy.rs:example}}
```

## References and Borrowing {#references-borrowing}

A _reference_ in Rust is essentially a pointer (a memory address) to a value in memory, plus additional guarantees that the pointed-to data is valid (while a reference to an object exist, the object cannot be destroyed / dropped).

Crucially, references _do not own_ the value they point to. Creating a reference is called _borrowing_.

Immutable references (of type `&T` if `T` is the base type), also called shared references, provide read-only access to the underlying data:

```rust,editable
{{#include ../../crates/language/examples/ownership_borrowing/borrowing_immutable.rs:example}}
```

Mutable references (of type `&mut T`), called exclusive references, borrow a value and modify it:

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

We discussed above that passing a variable to a function by value will move or copy it, just as assignment does. To avoid transferring ownership of non-`Copy` data every time you call a function, you will very often "borrow" the value using references.{{hi:Borrowing}}

The following example shows a function that takes a sample `struct` by reference (`&T`), instead of by value (`T`). While that struct has move semantics, it is not consumed by the function when it is passed by reference. The function borrows, but does not gain ownership of, what it refers to, thus the referred value is _not_ dropped when the function returns:

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

- [[rust-patterns | Rust Patterns]].

</div>
