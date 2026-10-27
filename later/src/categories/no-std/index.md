<!-- TODO Reviewed ??-??-2026 -->

# No Standard Library

[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No std}}

The `#![no_std]` attribute makes a crate use [`core`][c~core~docs]↗{{hi:core}} instead of the standard library prelude. This is useful on targets without [`std`][c~std~docs]↗{{hi:std}}, but a `no_std` library can also be used by an application that links `std`. Available I/O, allocation, and concurrency depend on the target and the surrounding runtime.

- A freestanding binary needs a suitable runtime and panic handler.
- You'll typically interact directly with hardware peripherals.
- You may need to manage memory manually, especially if you need dynamic allocation.
- If you're working with multiple tasks or interrupts, you'll need to consider [[concurrency | Concurrency]] carefully.

Under [`#![no_std]`][book~rust-reference~no_std]↗{{hi:no_std}}, you use [`core`][c~core~docs]↗{{hi:core}}, and optionally [`alloc`][c~alloc~docs]↗{{hi:alloc}} with an allocator you provide. For [[embedded | Embedded]] systems, also choose the HAL and PAC crates for your microcontroller.

## The Three Layers

| Layer   | Crate                                | Needs              | Provides                                                                                                             |
| ------- | ------------------------------------ | ------------------ | -------------------------------------------------------------------------------------------------------------------- |
| `core`  | [`core`][c~core~docs]↗{{hi:core}}    | Nothing            | Primitive types, `Option`, `Result`, iterators, atomics, formatting. Always available.                               |
| `alloc` | [`alloc`][c~alloc~docs]↗{{hi:alloc}} | A global allocator | `Box`, `Vec`, `String`, `Rc`, `Arc`, `BTreeMap`. See [No Alloc](../no-std_no-alloc/index.md) for working without it. |
| `std`   | [`std`][c~std~docs]↗{{hi:std}}       | Target support     | Higher-level APIs such as files and threads, where the target supports them.                                         |

A freestanding `#![no_std]` binary needs a panic handler. Crates such as [`panic-halt`][c~panic-halt~docs]↗{{hi:panic-halt}} and [`panic-abort`][c~panic-abort~docs]↗{{hi:panic-abort}} supply one. See the [no-std chapter](no_std.md) for examples.

## Useful `no_std` Crates

| Need                                       | Crate                                                                               | Notes                                                                                                                  |
| ------------------------------------------ | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Fixed-capacity collections without `alloc` | [`heapless`][c~heapless~docs]↗{{hi:heapless}}                                       | `Vec`, `String`, queues and maps with a compile-time capacity.                                                         |
| Hardware abstraction traits                | [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}                           | Portable traits for embedded interfaces. See [Embedded](../embedded/index.md).                                         |
| Logging                                    | [`defmt`][c~defmt~docs]↗{{hi:defmt}}                                                | Compact, deferred-formatting logger for microcontrollers.                                                              |
| Serialization                              | [`postcard`][c~postcard~docs]↗{{hi:postcard}}                                       | Compact `serde` format that works without `std` or `alloc`.                                                            |
| Floating-point math                        | [`libm`][c~libm~docs]↗{{hi:libm}}, [`micromath`][c~micromath~repo]↗{{hi:micromath}} | Math functions for `no_std` environments; compare precision and target performance for your application.               |
| Non-blocking I/O                           | [`nb`][c~nb~docs]↗{{hi:nb}}                                                         | Helpers for polling-style peripherals.                                                                                 |
| Atomics on any target                      | [`atomic`][c~atomic~docs]↗{{hi:atomic}}                                             | Generic `Atomic<T>` wrapper.                                                                                           |
| Global allocator                           | [`embedded-alloc`][c~embedded-alloc~docs]↗{{hi:embedded-alloc}}                     | Heap allocator for use with `alloc`. Prefer it over the unmaintained [`wee_alloc`][c~wee_alloc~docs]↗{{hi:wee_alloc}}. |

## Code Examples

{{#include no_std.incl.md}}

## Crates Functioning Without the Rust Standard Library

Consult the [`no-std`][cat~no-std]↗ category on [`crates.io`][crates.io~website]↗{{hi:crates.io}}.

## Related Topics

- [[embedded | Embedded]] Systems.
- [[memory-management | Memory Management]].
- [[memory_usage_analysis | Memory Usage Analysis]].
- [[no-std_no-alloc | No Alloc]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
