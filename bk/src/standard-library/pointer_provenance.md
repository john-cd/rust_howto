<!-- TODO Reviewed ??-??-2026. -->

# Pointer Provenance at an FFI Boundary

A pointer carries more than an address: it also has provenance that determines which allocation it may access. When a C-compatible function accepts a pointer and a length, its caller must guarantee that the allocation remains live and readable for the whole call. The callee must check the length before reading. This example uses an `extern "C"` function as a local stand-in for that boundary:

```rust
{{#include ../../crates/standard_library/examples/pointer_provenance.rs:example}}
```

Run `cargo test -p standard_library --example pointer_provenance` from `bk/crates`. With the Miri component installed on a compatible nightly toolchain, run `cargo +nightly miri test -p standard_library --example pointer_provenance` from the same directory.

[`addr` and `with_addr`](https://doc.rust-lang.org/std/primitive.pointer.html#method.with_addr) keep the new pointer tied to the original pointer's allocation. An integer address alone does not establish a valid pointer. Casting an arbitrary integer to a raw pointer does not prove that it is aligned, initialized, in bounds, or still live. Prefer a safe slice or reference whenever the API allows it. A real foreign API must document ownership, lifetime, nullability, and aliasing separately.

For a walkthrough of each caller and callee obligation, see [Unsafe Contracts](../language/unsafe_contracts.md).

See [This Week in Rust #677](https://this-week-in-rust.org/blog/2026/11/11/this-week-in-rust-677/) for the pointer-provenance walkthrough that motivated this recipe.

Rust 1.99 also stabilized defining C-ABI variadic functions and raw-pointer size/alignment queries. Those APIs have separate argument-type and validity contracts; see the [Rust 1.99 release notes](https://blog.rust-lang.org/releases/1.99.0/) and the [staging FFI guide](../categories/development-tools_ffi/index.md) before using them at a foreign boundary.
