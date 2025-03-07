# No Standard Library

[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No std}}

The term "no_std" refers to environments where you don't have access to the standard library ([`std`][c-std]⮳{{hi:std}}), which means no file I/O, dynamic memory allocation (unless you provide it), threading, or other OS-dependent features.

- You'll need to provide your own runtime environment, including a panic handler.
- You'll typically interact directly with hardware peripherals.
- You may need to manage memory manually, especially if you need dynamic allocation.
- If you're working with multiple tasks or interrupts, you'll need to consider [[concurrency | Concurrency]] carefully.

When working in `no_std`, you'll almost always use the `core` crate. If you need dynamic allocation, you'll also use the `alloc` crate and provide an allocator. For [[embedded | Embedded]] Systems, you'll need to choose the appropriate HAL and PAC crates for your target microcontroller.

Remember to handle panics appropriately. Consider other useful crates like [`nb`][c-nb]⮳{{hi:nb}}, [`defmt`][c-defmt]⮳{{hi:defmt}}, and [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} as needed.

## Key Crates

- The Core library (`core`) is the foundation of `no_std` programming. It provides the bare minimum functionality required for Rust code, including basic types, traits, and memory management primitives. It's always used in `no_std` environments.

- Allocation (`alloc`) library: If you need dynamic memory allocation in `no_std`, the `alloc` crate provides the necessary APIs. It's important to note that you'll need to provide your own allocator implementation. See also the [[no-std_no-alloc | No Alloc]] chapter.

- In `no_std` environments, you can't rely on the standard library's panic handling mechanisms. You'll need to define your own panic handler. Crates like [`panic-halt`][c-panic_halt]⮳{{hi:panic-halt}} (halts execution) or [`panic-abort`][c-panic_abort]⮳{{hi:panic-abort}} (aborts execution) can be helpful. You might also implement custom panic behavior.

## Other Useful Crates for `no_std`

- [`nb`][c-nb]⮳{{hi:nb}}: Non-blocking I/O helpers.
- [`defmt`][c-defmt]⮳{{hi:defmt}}: A logging framework designed for embedded systems.
- [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}: A small and efficient allocator for embedded systems or WebAssembly.
- [`atomic`][c-atomic]⮳{{hi:atomic}}: Provides atomic types for safe concurrency in embedded systems.

## Code Examples

{{#include no_std.incl.md}}

## Crates that are able to function without the Rust standard library

Consult the [`no-std`][cat-no-std] category on `crates.io`.

## Related Topics

- [[embedded | Embedded]] Systems.
- [[memory-management | Memory Management]].
- [[memory_usage_analysis | Memory Usage Analysis]].
- [[no-std_no-alloc | No Alloc]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/427)
review in depth
</div>
