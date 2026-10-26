<!-- TODO Reviewed ??-??-2026 -->

# no_std and No Allocation

[![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No alloc}}

Rust code can omit the standard library with `#![no_std]` and use `core` instead. A `no_std` program may still use `alloc` when its target provides an allocator. A no-allocation program avoids runtime heap allocation too, which is useful for firmware, kernels, and systems that need predictable memory use.

Avoid heap-allocating types such as `Box`, `Vec`, `Rc`, `Arc`, and `HashMap` when the program must not allocate at runtime. Prefer arrays, fixed-capacity collections, and static storage when their sizes and lifetimes fit the design. Deterministic memory use can matter in real-time systems as well as in targets with no allocator.

## Fixed-Capacity Collections

Use arrays or fixed-capacity collections when the maximum size is known. [heapless][c~heapless~docs]↗{{hi:heapless}} provides collections and strings that do not allocate; [arrayvec][c~arrayvec~docs]↗{{hi:arrayvec}} provides array-backed vectors and strings. Operations beyond capacity return an error instead of growing the collection.

## Formatting and Serialization

- [core::fmt::Write][c~core::fmt::Write~docs]↗ writes formatted values into a caller-provided buffer. The example formats into a fixed-capacity [heapless::String][c~heapless~docs]↗{{hi:heapless}}.
- [ufmt][c~ufmt~docs]↗ is a small formatting library for constrained targets.
- [postcard][c~postcard~docs]↗ supports compact serialization; configure [serde][c~serde~docs]↗ with default features disabled for `no_std` use.
- [defmt][c~defmt~docs]↗ provides compact logging for embedded targets.

For embedded hardware interfaces, see [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}. Small panic handlers such as [`panic-halt`][c~panic-halt~docs]↗{{hi:panic-halt}} and [`panic-abort`][c~panic-abort~docs]↗{{hi:panic-abort}} provide target-appropriate panic behavior; choose one that matches the platform. Static allocation is another option for values whose storage can live for the program's duration; see [[static allocation | Static Allocation]].

## Example

{{#include no_alloc.incl.md}}

## Related Topics

- [[no-std | No `std`]].
- [[embedded | Embedded Systems]].
- [[memory-management | Memory Management]].
- [[memory_usage_analysis | Memory Usage Analysis]].
- [The crates.io no-allocation category][cat~no-std::no-alloc]↗ lists crates tagged for no-alloc use.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  content audit: category index content removed by recent commits; review and restore
</div>
