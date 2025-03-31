# Memory Management

[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

Deal with allocation{{hi:Allocation}}, memory mapping{{hi:Memory mapping}}, garbage collection{{hi:Garbage collection}}, reference counting{{hi:Reference counting}}, or interfaces to foreign memory managers.

Rust's memory management is a core strength: you won't often need to manually manage memory like in C/C++. For most common Rust development, relying on language features like ownership, borrowing, and lifetimes, and using smart pointers like `Box`, `Rc`, and `Arc` will be sufficient. Avoid `unsafe` code and raw pointers unless absolutely necessary.

| Topic | Rust Crates or Features |
|---|---|
| Smart Pointers | Use [`std::boxed::Box`][c-std::boxed::Box]⮳{{hi:std::boxed::Box}} for heap allocation; [`std::rc::Rc`][c-std::rc::Rc]⮳{{hi:std::rc::Rc}} for reference-counted shared ownership; [`std::sync::Arc`][c-std::sync::Arc]⮳{{hi:std::sync::Arc}} for atomically reference-counted shared ownership (thread-safe); [`std::cell::RefCell`][c-std::cell::RefCell]⮳{{hi:std::cell::RefCell}} for interior mutability; and [`std::sync::Mutex`][c-std::sync::Mutex]⮳{{hi:std::sync::Mutex}} for safe mutable access from multiple threads. |
| Global Statics and Lazy Initialization | FIXME |
| Core Allocation (Rarely Used Directly) | `alloc` (Standard library) provides the fundamental allocation APIs. Most other memory management tools are built on top of it. |
| Specialized Allocators | [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} is a small and efficient allocator, often used in embedded systems or WebAssembly. |
| Memory Profiling | [`valgrind`][c-valgrind]⮳{{hi:valgrind}} (with `massif` or `memcheck`): External tool. Powerful memory profiler. [`heaptrack`][c-heaptrack]⮳{{hi:heaptrack}}: External tool. Heap profiler. |

## Memory Management Beyond Basic Ownership with Smart Pointers

- `Box`: For allocating data on the heap.
- `Rc` (Reference Counting): For shared ownership of data.
- `Arc` (Atomic Reference Counting): For shared ownership across threads.
- `RefCell`: For interior mutability (allowing you to mutate data even when there are immutable references to it).
- `Mutex`: For safe mutable access to data from multiple threads.

See [[smart_pointers | Smart Pointers]].

## Statics and Lazy Initialization

`static` refers to items that have a fixed memory location and a `'static` lifetime, meaning they exist for the entire duration of the program.
These are similar to constants but represent a specific memory location. All references to a static item point to the same memory location.
Static items can be mutable (), but modifying them requires an  block due to potential concurrency issues. They are often used for global variables or shared state across threads. See [[shared_state | Shared State]].

Lazy initialization is a technique where a resource or variable is initialized only when it's first accessed, rather than at the start of the program. Lazy initialization is particularly useful for expensive computations or resources that may not be needed during the program's execution.
It is commonly used with statics.

{{#include lazy_initialization.incl.md}}

## Unsafe Code and Raw Pointers

Use `unsafe` code and raw pointers (`*const T`, `*mut T`) only when necessary for interacting with external code or hardware. They bypass Rust's safety guarantees and require careful manual memory management.

## Custom Memory Allocation, Garbage Collection

The core `alloc` crate provides the core allocation APIs. You'll rarely use this directly, but it's what the other memory management tools are built on. Custom allocators are rarely needed in typical Rust development.

- [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} is a small and efficient allocator often used in embedded systems or WebAssembly.

Rust does not have a garbage collector in the traditional sense. It uses ownership and borrowing to manage memory automatically and deterministically. If you need garbage collection for specific reasons, you'd have to look for specialized crates, but this is rare in Rust.

- The [`seize`][c-seize]⮳{{hi:seize}} crate allows for memory reclamation in concurrent data structures.

{{#include memory_allocation.incl.md}}

## Related Topics

### Memory Safety Tools

Memory profiling tools like [`Valgrind`][c-valgrind]⮳{{hi:Valgrind}} are useful for optimizing memory usage.

- [`valgrind`][c-valgrind]⮳{{hi:valgrind}} (with `massif` or `memcheck`): While not Rust-specific, Valgrind is a very common and powerful memory profiler. You'd run your Rust program under Valgrind.
- [`heaptrack`][c-heaptrack]⮳{{hi:heaptrack}}: A heap profiler that can track memory allocations.

Address Sanitizer (`ASan`, a compiler feature) detects memory errors. Enable with compiler flags (e.g., `-fsanitize=address`). Memory Sanitizer (`MSan`, another compiler feature) detects uninitialized memory usage.

See [[development-tools_profiling | Development Tools: Profiling]] and [[memory_usage_analysis | Memory Usage Analysis]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[memory-management/index: organize; align table and sections; write missing sections; cross link NOW](https://github.com/john-cd/rust_howto/issues/410)
cover https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
cover ASan and MSan
other allocators
unsafe
</div>
