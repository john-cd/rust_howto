# No-alloc

[![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No alloc}}

The term "no_alloc" refers to environments where *dynamic* memory allocation is not available or desirable (e.g., [[embedded | Embedded]] systems, kernel development, or performance-sensitive applications). The concept revolves around avoiding the use of the [`alloc`](https://doc.rust-lang.org/alloc/index.html)↗{{hi:alloc}} crate and its associated functions.

`no_alloc`{{hi:no_alloc}} memory allocation can be made deterministic, which is important for real-time systems or other applications where predictable performance is crucial.

When working in a `no_alloc`{{hi:no_alloc}} environment, you'll primarily rely on stack allocation and static allocation. You'll avoid using crates or data structures that depend on the [`alloc`](https://doc.rust-lang.org/alloc/index.html)↗{{hi:alloc}} crate. Understanding memory management techniques like bump allocation or memory pooling can be helpful if you need more complex allocation strategies.

Note the following:

- `no_alloc`{{hi:no_alloc}} environments often have limited memory, so it's important to use memory efficiently.
- There is no Garbage Collection: You're responsible for managing memory manually (though often simply by using stack allocation).

## Core Concepts and Techniques

- Stack Allocation: The primary way to manage memory in `no_alloc`{{hi:no_alloc}} environments is through stack allocation. This is done by declaring variables directly within functions. Stack allocation is fast and deterministic, but it's limited by the stack size.

- Avoiding Dynamic Data Structures: In `no_alloc`{{hi:no_alloc}}, you'll avoid dynamic data structures like [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)↗, [`String`](https://doc.rust-lang.org/alloc/string/struct.String.html)↗, [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)↗{{hi:std::boxed::Box}}, [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)↗{{hi:std::rc::Rc}}, [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)↗{{hi:std::sync::Arc}}, etc., as these rely on heap allocation. Instead, you'll use fixed-size arrays, structs, and other data structures that can be allocated on the stack or statically.

- [[Const Generics]] are very useful for working with fixed-size data structures in `no_alloc` contexts.

- Static Allocation: You can also use [[static allocation]] for data that is known at compile time. This is done using the [`static`](https://doc.rust-lang.org/std/keyword.static.html)↗{{hi:static}} keyword. Static variables have a fixed memory location and a lifetime that lasts for the entire duration of the program. See [[global_static | Global Static]].

- Bump Allocators: A simple and fast allocation strategy often used in `no_alloc`{{hi:no_alloc}} environments. A bump allocator simply increments a pointer to allocate memory. It's very efficient but doesn't support deallocation. See [[memory-management | Memory Management]].

- Memory Pooling: A technique where a fixed block of memory is divided into smaller chunks, and these chunks are then allocated and deallocated as needed.

- Custom Allocators: For more complex allocation needs, you can implement custom allocators. However, this is usually only necessary for very specialized cases.

## Relevant Crates and Features

- [`core`](https://doc.rust-lang.org/core/index.html)↗{{hi:core}}: it provides the foundation for [`no_std`](https://doc.rust-lang.org/reference/names/preludes.html#r-names.preludes.extern.no_std)↗{{hi:no_std}} programming, which is often a prerequisite for `no_alloc`{{hi:no_alloc}}.

- [`alloc`](https://doc.rust-lang.org/alloc/index.html)↗{{hi:alloc}}: It's the standard library's allocation crate. While you're avoiding it, understanding why you're avoiding it is important.

- [`wee_alloc`][c~wee_alloc~docs]↗{{hi:wee_alloc}}: A small and efficient allocator often used in embedded systems or WebAssembly ([[wasm | WASM]]). It can be useful if you -do- need some allocation, but want a very small allocator. See [[embedded | Embedded]] Systems.

- [`panic-halt`][c~panic-halt~docs]↗{{hi:panic-halt}}, [`panic-abort`][c~panic-abort~docs]↗{{hi:panic-abort}}: Crates for defining panic behavior in `no_std` environments.

- [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}: Essential for interacting with hardware in embedded systems, which are often `no_alloc` environments.

## Code Examples

{{#include no_alloc.incl.md}}

## Crates Functioning Without the Rust [`alloc`][c~alloc~docs]↗{{hi:alloc}} Crate

Consult the [`no_alloc`][cat~no-std::no-alloc] category on [`crates.io`](https://crates.io)↗{{hi:crates.io}}.

## Related Topics

- [[embedded | Embedded]] Systems.
- [[memory-management | Memory Management]].
- [[memory_usage_analysis | Memory Usage Analysis]].
- [[stack_allocated_arrays | Stack Allocated Arrays]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review; cover use of no_std attribute to remove alloc crate](https://github.com/john-cd/rust_howto/issues/945)
</div>
