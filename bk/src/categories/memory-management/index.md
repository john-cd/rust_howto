# Memory Management

[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

Deal with allocation{{hi:Allocation}}, memory mapping{{hi:Memory mapping}}, garbage collection{{hi:Garbage collection}}, reference counting{{hi:Reference counting}}, or interfaces to foreign memory managers.

## Global statics

{{#include global_static.incl.md}}

## Lazy initialization

{{#include lazy_initialization.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[memory-management/index: organize (P1)](https://github.com/john-cd/rust_howto/issues/410)

## Key Points

- Ownership and borrowing are language features, not crates. They are fundamental to Rust's memory safety.
- Smart pointers are the most common way to manage memory beyond basic ownership.
- ASan is essential for catching memory errors during development.
- Memory profiling tools are important for optimizing memory usage.
- Custom allocators or garbage collection are rarely needed in typical Rust development.

Here's a short list of memory management crates in Rust, categorized by topic:

## Core Allocation (Rarely Used Directly)

- `alloc`: (Standard library) Provides the fundamental allocation APIs. Most other memory management tools are built on top of it.

## Smart Pointers (Commonly Used)

- [`std::boxed::Box`][c-std::boxed::Box]⮳{{hi:std::boxed::Box}}: For heap allocation.
- [`std::rc::Rc`][c-std::rc::Rc]⮳{{hi:std::rc::Rc}}: For reference-counted shared ownership.
- [`std::sync::Arc`][c-std::sync::Arc]⮳{{hi:std::sync::Arc}}: For atomically reference-counted shared ownership (thread-safe).
- [`std::cell::RefCell`][c-std::cell::RefCell]⮳{{hi:std::cell::RefCell}}: For interior mutability.
- [`std::sync::Mutex`][c-std::sync::Mutex]⮳{{hi:std::sync::Mutex}}: For safe mutable access from multiple threads.

## Specialized Allocators

- [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}: A small and efficient allocator, often used in embedded systems or WebAssembly.

## Memory Safety Tools (Essential for Development)

- Address Sanitizer (`ASan`): (Compiler feature) Detects memory errors. Enable with compiler flags (e.g., `-fsanitize=address`).
- Memory Sanitizer (`MSan`): (Compiler feature) Detects uninitialized memory usage.

## Memory Profiling

Link to:

- [`valgrind`][c-valgrind]⮳{{hi:valgrind}} (with `massif` or `memcheck`): External tool. Powerful memory profiler.
- [`heaptrack`][c-heaptrack]⮳{{hi:heaptrack}}: External tool. Heap profiler.

## Other (Less Common or Specialized)

- Garbage Collection: Rust generally avoids GC. If you need it, you'd likely have to explore very specialized options.
- Custom Allocators: You can implement custom allocators, but this is usually only necessary for very specific performance requirements.

---

Rust's memory management is a core strength, and while you don't often need to manually manage memory like in C/C++, understanding the concepts and using the right tools is important.

## Key Concepts

- Ownership: Every value in Rust has a single owner.
- Borrowing: You can borrow references to values without taking ownership.
- Lifetimes: Annotations that ensure that references are valid for as long as they are used.
- Memory leaks: Occur when memory is allocated but never deallocated.
- Use-after-free: Occurs when you try to access memory that has already been freed.
- Data races: Occur when multiple threads access the same memory location without proper synchronization.

For most common Rust development, understanding ownership, borrowing, and lifetimes, and using smart pointers like `Box`, `Rc`, and `Arc` will be sufficient. ASan is incredibly helpful for finding memory errors. Memory profiling tools like Valgrind are useful for optimizing memory usage. Avoid `unsafe` code and raw pointers unless absolutely necessary.

## Ownership, Borrowing, and Lifetimes (Fundamental Rust concepts)

These are built into the Rust language itself, not specific crates. Mastering these is essential for understanding Rust's memory management.

## Smart Pointers (for managing memory beyond basic ownership)

- `Box`: For allocating data on the heap.
- `Rc` (Reference Counting): For shared ownership of data.
- `Arc` (Atomic Reference Counting): For shared ownership across threads.
- `RefCell`: For interior mutability (allowing you to mutate data even when there are immutable references to it).
- `Mutex`: For safe mutable access to data from multiple threads.

## Memory Allocation

- `alloc`: (Standard library) Provides the core allocation APIs. You'll rarely use this directly, but it's what the other memory management tools are built on.
- [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}: A small and efficient allocator often used in embedded systems or WebAssembly.

## Memory Safety Tools

- Address Sanitizer (`ASan`): A compiler-based tool for detecting memory errors (use-after-free, memory leaks, etc.). Enable it with compiler flags (e.g., `-fsanitize=address`). Highly recommended during development.
- Memory Sanitizer (`MSan`): Detects use of uninitialized memory.

## Memory Profiling 2

Link to:

- [`valgrind`][c-valgrind]⮳{{hi:valgrind}} (with `massif` or `memcheck`): While not Rust-specific, Valgrind is a very common and powerful memory profiler. You'd run your Rust program under Valgrind.
- [`heaptrack`][c-heaptrack]⮳{{hi:heaptrack}}: A heap profiler that can track memory allocations.

## Garbage Collection (Not Typical in Rust)

Rust does not have a garbage collector in the traditional sense. It uses ownership and borrowing to manage memory automatically and deterministically. If you need garbage collection for specific reasons, you'd have to look for specialized crates, but this is extremely rare in Rust.

[`seize`][c-seize]⮳{{hi:seize}} crate

## Unsafe Code and Raw Pointers

Use `unsafe` code and raw pointers (`*const T`, `*mut T`) only when absolutely necessary for interacting with external code or hardware. They bypass Rust's safety guarantees and require very careful manual memory management.

---

TODO P2 add

## `bumpalo` {#bumpalo}

[![bumpalo][c-bumpalo-badge]][c-bumpalo] [![bumpalo-crates.io][c-bumpalo-crates.io-badge]][c-bumpalo-crates.io] [![bumpalo-github][c-bumpalo-github-badge]][c-bumpalo-github] [![bumpalo-lib.rs][c-bumpalo-lib.rs-badge]][c-bumpalo-lib.rs]{{hi:bumpalo}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

A fast bump allocation arena for Rust.

## `slab` {#slab}

[![slab][c-slab-badge]][c-slab] [![slab-crates.io][c-slab-crates.io-badge]][c-slab-crates.io] [![slab-github][c-slab-github-badge]][c-slab-github] [![slab-lib.rs][c-slab-lib.rs-badge]][c-slab-lib.rs]{{hi:slab}}{{hi:slab}}{{hi:Allocator}}{{hi:No_std}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Pre-allocated storage for a uniform data type.

Slab provides pre-allocated storage for a single data type. If many values of a single type are being allocated, it can be more efficient to pre-allocate the necessary storage. Since the size of the type is uniform, memory fragmentation can be avoided. Storing, clearing, and lookup operations become very cheap.

While Slab may look like other Rust collections, it is not intended to be used as a general purpose collection. The primary difference between Slab and Vec is that Slab returns the key when storing the value.

It is important to note that keys may be reused. In other words, once a value associated with a given key is removed from a slab, that key may be returned from future calls to insert.

</div>
