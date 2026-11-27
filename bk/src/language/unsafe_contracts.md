<!-- TODO Reviewed ??-??-2026. -->

# Unsafe Contracts

Safe Rust checks many preconditions through its types and runtime checks. An
unsafe operation needs additional facts the compiler cannot verify. Write
those facts down where the operation depends on them.

- An `unsafe fn` tells callers to satisfy its documented `# Safety` contract.
  The function body still uses an explicit `unsafe` block for each operation
  that needs a proof. Rust 2024 warns about unsafe operations outside such a
  block, even inside an unsafe function.
- An `unsafe` block says the surrounding code has established the preconditions
  for the operations inside it. A nearby `SAFETY` comment should name the facts
  that make the operation sound.
- An `unsafe impl` promises to uphold the invariants of an unsafe trait. For
  example, safe callers may rely on a `Send` implementation to transfer a
  value between threads. Review the type's construction, mutation, and cleanup
  paths when checking the promise.

## Read Through a Safe Wrapper {#read-through-a-safe-wrapper}

The [pointer provenance recipe](../standard-library/pointer_provenance.md)
shows a borrowed slice passed across a C ABI. Follow the contract in both directions.

```rust,noplayground
{{#include ../../crates/standard_library/examples/pointer_provenance.rs:example}}
```

The safe wrapper accepts `&[u8]`. The slice supplies initialized bytes in a
live allocation for the duration of the call. Shared borrowing permits other
readers but rules out concurrent mutation of those bytes through ordinary
references. The wrapper passes the matching pointer and length, with no
ownership transfer.

Inside `second_byte`, the length check establishes that byte 1 is in bounds.
The pointer adjustment keeps the allocation provenance, and `u8` requires
only byte alignment. The unsafe dereference reads one byte while the original
borrow remains live. Early return on short input avoids dereferencing even an
empty slice's pointer.

If both sides of the API are Rust, `bytes.get(1).copied()` provides the same
result through checked slice access.

From `bk/crates`, use a normal test and then run it under Miri with a compatible nightly:

```console
cargo test --locked -p standard_library --example pointer_provenance
cargo +nightly-2026-11-16 miri test --locked -p standard_library --example pointer_provenance
```

The fixture covers empty, short, and readable slices. Miri checks the executed
paths. A reviewer must still establish the contract for every possible caller,
including a real foreign caller that supplies its own pointer and length.

## Related Topics

- [Pointer Provenance](../standard-library/pointer_provenance.md) explains address adjustment and allocation identity.
- [Memory Address Pinning](../standard-library/memory_address_pinning.md) describes address-stability promises.
- [Miri](../categories/development-tools/other/miri.md) explains setup and interpretation limits.
- [The unsafe keyword in the Rust Reference](https://doc.rust-lang.org/reference/unsafe-keyword.html) describes the language rules.
- [Unsafe operations in unsafe functions in the Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html) covers the lint.
