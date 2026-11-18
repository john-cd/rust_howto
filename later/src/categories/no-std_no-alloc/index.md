<!-- TODO Reviewed ??-??-2026. -->

# no_std and No Allocation

[![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No alloc}}

Rust code can omit the standard library with `#![no_std]` and use `core` instead. A `no_std` program may still use `alloc` when its target provides an allocator. A no-allocation program avoids runtime heap allocation too, which is useful for firmware, kernels, and systems that need predictable memory use.

Avoid heap-allocating types such as `Box`, `Vec`, `Rc`, `Arc`, and `HashMap` when the program must not allocate at runtime. Prefer arrays, fixed-capacity collections, and static storage when their sizes and lifetimes fit the design. Deterministic memory use can matter in real-time systems as well as in targets with no allocator.

## Fixed-Capacity Collections

Use arrays or fixed-capacity collections when the maximum size is known. [heapless][c~heapless~docs]↗{{hi:heapless}} provides collections and strings that do not allocate; [arrayvec][c~arrayvec~docs]↗{{hi:arrayvec}} provides array-backed vectors and strings. Use fallible insertion methods such as `heapless::Vec::push` and `arrayvec::ArrayVec::try_push` to handle full capacity explicitly; some infallible insertion methods panic.

## Formatting and Serialization

- [core::fmt::Write][c~core::fmt::Write~docs]↗ writes formatted values into a caller-provided buffer. The example formats into a fixed-capacity [heapless::String][c~heapless~docs]↗{{hi:heapless}}.
- [ufmt][c~ufmt~docs]↗ is a small formatting library for constrained targets.
- [postcard][c~postcard~docs]↗ supports compact serialization; configure [serde][c~serde~docs]↗ with default features disabled for `no_std` use.
- [defmt][c~defmt~docs]↗ provides compact logging for embedded targets.

See the [published No Allocation chapter](https://john-cd.com/rust_howto/categories/no-std/no_alloc.html) for collections, bounded queues, and formatting without heap allocation.

For embedded hardware interfaces, see [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}. Small panic handlers such as [`panic-halt`][c~panic-halt~docs]↗{{hi:panic-halt}} and [`panic-abort`][c~panic-abort~docs]↗{{hi:panic-abort}} provide target-appropriate panic behavior; choose one that matches the platform. Static allocation is another option for values whose storage can live for the program's duration; see [static items in the Rust Reference](https://doc.rust-lang.org/reference/items/static-items.html).

## Published Examples

The fixed-capacity collection, bounded queue, and formatting recipes now live in the [published No Allocation chapter](https://john-cd.com/rust_howto/categories/no-std/no_alloc.html). Their tested sources are in `bk/crates/cats/no_std_no_alloc`; this preview does not duplicate the moved recipe table.

## Related Topics

- [No `std`](../no-std/index.md).
- [Embedded Systems](../embedded/index.md).
- [Memory Management](https://john-cd.com/rust_howto/categories/memory-management/index.html).
- [Memory Usage Analysis](https://john-cd.com/rust_howto/categories/development-tools_profiling/memory_usage_analysis.html).
- [The crates.io no-allocation category][cat~no-std::no-alloc]↗ lists crates tagged for no-alloc use.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
