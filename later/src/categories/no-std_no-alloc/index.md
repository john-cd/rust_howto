<!-- TODO Reviewed ??-??-2026 -->
# no_std and No Allocation

[![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No alloc}}

Rust code can omit the standard library with `#![no_std]` and use `core` instead. A `no_std` program may still use `alloc` when its target provides an allocator. A no-allocation program avoids runtime heap allocation too, which is useful for firmware, kernels, and systems that need predictable memory use.

## Fixed-Capacity Collections

Use arrays or fixed-capacity collections when the maximum size is known. [heapless][c~heapless~docs]↗{{hi:heapless}} provides collections and strings that do not allocate; [arrayvec][c~arrayvec~docs]↗{{hi:arrayvec}} provides array-backed vectors and strings. Operations beyond capacity return an error instead of growing the collection.

## Formatting and Serialization

- [core::fmt::Write][c~core::fmt::Write~docs]↗ writes formatted values into a caller-provided buffer. The example formats into a fixed-capacity [heapless::String][c~heapless~docs]↗{{hi:heapless}}.
- [ufmt][c~ufmt~docs]↗ is a small formatting library for constrained targets.
- [postcard][c~postcard~docs]↗ supports compact serialization; configure [serde][c~serde~docs]↗ with default features disabled for `no_std` use.
- [defmt][c~defmt~docs]↗ provides compact logging for embedded targets.

## Example

{{#include no_alloc.incl.md}}

## Related Topics

- [[no-std | No `std`]].
- [[embedded | Embedded Systems]].
- [[memory-management | Memory Management]].
- [[memory_usage_analysis | Memory Usage Analysis]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: category index content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1903)
</div>
