# No-alloc

[![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No alloc}}

Crates that are able to function without the Rust [`alloc`][c-alloc]⮳{{hi:alloc}} crate.

{{#include no_alloc.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/945)

The term "no_alloc" usually refers to environments where dynamic memory allocation is not available or desirable (e.g., embedded systems, kernel development, or performance-sensitive applications). The concept revolves around avoiding the use of the `alloc` crate and its associated functions.

When working in a `no_alloc` environment, you'll primarily rely on stack allocation and static allocation. You'll avoid using crates or data structures that depend on the `alloc` crate. Understanding memory management techniques like bump allocation or memory pooling can be helpful if you need more complex allocation strategies. And you'll need to be mindful of memory limitations and deterministic behavior.

Here's a breakdown:

## Core Concepts and Techniques

- Stack Allocation: The primary way to manage memory in `no_alloc` environments is through stack allocation. This is done by declaring variables directly within functions. Stack allocation is fast and deterministic, but it's limited by the stack size.

- Static Allocation: You can also use static allocation for data that is known at compile time. This is done using the `static` keyword. Static variables have a fixed memory location and a lifetime that lasts for the entire duration of the program.

- Bump Allocators: A simple and fast allocation strategy often used in `no_alloc` environments. A bump allocator simply increments a pointer to allocate memory. It's very efficient but doesn't support deallocation.

- Custom Allocators: For more complex allocation needs, you can implement custom allocators. However, this is usually only necessary for very specialized cases.

- Memory Pooling: A technique where a fixed block of memory is divided into smaller chunks, and these chunks are then allocated and deallocated as needed.

- Avoiding Dynamic Data Structures: In `no_alloc`, you'll typically avoid dynamic data structures like `Vec`, `String`, `Box`, `Rc`, `Arc`, etc., as these rely on heap allocation. Instead, you'll use fixed-size arrays, structs, and other data structures that can be allocated on the stack or statically.

- Const Generics: These are very useful for working with fixed-size data structures in `no_alloc` contexts.

## Relevant Crates and Features

- `core`: (Standard library) Provides the foundation for `no_std` programming, which is often a prerequisite for `no_alloc`.

- `alloc`: (Standard library) While you're -avoiding- it, understanding -why- you're avoiding it is important. It's the standard library's allocation crate.

- [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}: A small and efficient allocator often used in embedded systems or WebAssembly. It can be useful if you -do- need some allocation, but want a very small allocator.

- [`panic-halt`][c-panic_halt]⮳{{hi:panic-halt}}, [`panic-abort`][c-panic_abort]⮳{{hi:panic-abort}}: Crates for defining panic behavior in `no_std` environments.

- [`embedded-hal`][c-embedded_hal]⮳{{hi:embedded-hal}}: Essential for interacting with hardware in embedded systems, which are often `no_alloc` environments.

## Key Considerations

- Limited Memory: `no_alloc` environments often have limited memory, so it's important to use memory efficiently.
- Deterministic Behavior: Stack allocation is deterministic, which is important for real-time systems or other applications where predictable performance is crucial.
- No Garbage Collection: You're responsible for managing memory manually (though often simply by using stack allocation).

</div>
