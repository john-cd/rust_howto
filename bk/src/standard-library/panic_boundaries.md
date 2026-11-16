<!-- TODO Reviewed ??-??-2026. -->

# Handle Panics at a Boundary

Use `Result` for expected failures. A panic normally signals a broken invariant or defect; a service may choose to terminate so its supervisor can restart it. At a deliberately isolated callback or foreign-function boundary, `catch_unwind` can translate an unwinding panic into an explicit failure status. This example keeps the expected error and panic paths separate and returns a C-compatible value:

```rust
{{#include ../../crates/standard_library/examples/panic_boundary.rs:example}}
```

Run `cargo test -p standard_library --example panic_boundary` from `bk/crates`. The callback's panic hook still runs before `catch_unwind` returns; catching does not suppress the hook. Preserve the panic payload or record it in a diagnostic, as the example does. The return status is intentionally simple; a production API needs a documented way to retrieve detailed errors.

`catch_unwind` only catches unwinding Rust panics. It cannot recover when the binary uses `panic = "abort"`, and it is not a general guard against undefined behavior. An `extern "C"` entry point must contain any unwind before it reaches callers; choose an unwind-permitting ABI only with an explicit cross-language contract. See the [`catch_unwind` documentation](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) and the [Rust Reference on unwinding](https://doc.rust-lang.org/reference/items/functions.html#unwinding).

If a panic happens while a thread owns a standard-library `Mutex`, the lock becomes poisoned. A later `lock()` returns `PoisonError`; inspect the protected state before deciding whether recovery is sound. See the [`Mutex` documentation](https://doc.rust-lang.org/std/sync/struct.Mutex.html#poisoning). In an async service, contain failures at the task or request boundary only when the state and cleanup remain valid. [This Week in Rust #677](https://this-week-in-rust.org/blog/2026/11/11/this-week-in-rust-677/) links further discussion of production panic handling and poisoned locks.
