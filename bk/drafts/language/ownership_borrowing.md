# Ownership and Borrowing

{{#include ownership_borrowing.incl.md}}

The memory of a computer program is primarily structured into a _stack_ and a _heap_. The stack is stuctured: it stores values in the order it gets them and removes the values in the opposite order ("last in, first out"). During program execution, the stack is used to store function parameters, non-static local variables, function return values and return addresses, or pointers to the heap.

Under the covers, most CPUs maintain a pointer to the top of the stack in one of its internal registers. Pushing data onto the stack is a matter of copying a value onto the next available memory slot. Popping data off the stack is simply adjusting the stack pointer register value. Therefore, the stack is fast. It suffers from one major constraint: _all data stored on the stack must have a known, fixed size_.

Data with an unknown size at compile time or a size that might change must be stored on the heap instead. The heap is less organized and managed by a _memory allocator_ subroutine. Upon request for a memory block, the memory allocator finds empty memory in the heap, marks it as being in use, and returns a pointer, which is the memory address of the allocated memory. Because a pointer to the heap is a known, fixed size, the pointer can be stored on the stack, but when you want the actual data, the pointer must be followed (dereferenced).

Allocating space on the heap is slower than pushing data on the stack, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation. The allocator must track outstanding allocations to ensure that they do not overlap and that no memory that is no longer used is _leaked_ (lost; not reusable for the remainder of the program's execution). The heap further suffers from _fragmentation_, which arises when many small chunks of free memory is interspersed with allocated memory, to a point where large enough chunks of free memory cannot allocated.

Some languages (Java, C#, Go...) use _garbage collection_ (GC) that regularly reclaims no-longer-used memory as the program runs, moving memory blocks around to defragment the free memory (compaction). GC is automatic but expensive, as it must either temporarily stop the program or run in the background.

In other languages (C, C++...), the programmer must explicitly allocate and free the memory. This is excessively error-prone: each allocation must be paired with exactly one deallocation. Forgetting to free memory leaks it. Freeing memory too early or more than once ("double free") may lead to memory overlap and invalid variables.

Rust manages memory through its _ownership_ system instead. The memory is automatically returned once the variable that owns it goes out of scope.

## Ownership {#ownership}

[![Rust by example - Ownership][book-rust-by-example-move-badge]][book-rust-by-example-move]{{hi:Move}}{{hi:Ownership}}{{hi:Scope}}

Rust's ownership system ensures automatic _memory safety_ without the need for a garbage collector.{{hi:Garbage collector}}

Ownership a set of rules that the compiler enforces _at compile time_. If these rules are violated, the program won't compile:

1. **Each value in Rust has a variable that is its owner.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value is dropped** (meaning Rust automatically calls a special drop method to deallocate the memory).

The _scope_ of a variable is defined as follows: A variable is valid from the point it is declared until the end of its current scope (usually a block of code enclosed in curly braces `{ }` and typically the body of a function).

Rules 1 and 3 ensures automatic memory cleanup (no leaks). Rule 2 ensures no "double free". The following example illustrates these rules:

```rust,editable
{{#include ../../crates/language/tests/ownership_borrowing/scope.rs:example}}
```

### Move Semantics {#skip}

When you assign a heap-allocated value (like a `String`) from one variable to another, Rust _moves_ the ownership of the value, since there can only be one owner.

When ownership of value moves to another variable, the original variable _becomes invalid_. The compiler ensures that it cannot be accessed (or the program does not compile). The following example explains these concepts:

```rust,editable
{{#include ../../crates/language/tests/ownership_borrowing/move1.rs:example}}
```

## Clone and Copy Data {#clone-and-copy-data}

Rust will never automatically create deep copies of your data. Use the [`std::clone::Clone`][c-std::clone::Clone]{{hi:std::clone::Clone}}⮳ trait to explicitly create a deep copy:

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

A _reference_ in Rust is essentially a pointer (a memory address) to a value in memory, plus a guarantee that the pointed-to data is valid.

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
</div>
