<!-- TODO Reviewed ??-??-2026. -->

# Bare-metal Startup and Allocation

A bare-metal application needs startup code, a linker memory layout, and a panic handler. Start with the runtime or board support crate for your target. For a portable library that can also run under `std`, see [No Standard Library](no_std.md).

## Provide a Panic Handler {#panic-handler}

A freestanding binary must have exactly one panic handler in its dependency graph. Use the board or runtime's handler if it provides one. This excerpt supplies a handler that stops useful work after a panic:

```rust,ignore,noplayground
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
```

For debugging or recovery, choose target-specific behavior such as reporting the panic over a debug channel or handing recovery to a watchdog. The [Embedded Rust Book](https://docs.rust-embedded.org/book/) covers target setup, startup, and panic behavior.

## Add Heap Allocation {#heap-allocation}

A library can import heap-backed types with `extern crate alloc`. The final binary registers and initializes the global allocator. If the maximum size is known, [No Allocation](no_alloc.md) shows how to use fixed-capacity storage.

For a Cortex-M firmware that needs a heap, the [embedded-alloc Cortex-M example](https://github.com/rust-embedded/embedded-alloc/blob/v0.7.0/examples/global_alloc.rs) shows how to register `LlffHeap` and initialize it before creating a `Vec`. Enable `embedded-alloc`'s `llff` feature and select the [critical-section implementation](https://github.com/rust-embedded/critical-section#usage-in-no-std-binaries) for your hardware. On a single-core Cortex-M without an implementation from the HAL, `cortex-m` can provide it with its `critical-section-single-core` feature.

Reserve heap space in RAM and initialize it once, before any code that allocates can run, including interrupts. The allocator's initialization macro was introduced in `embedded-alloc` 0.7. These steps depend on the board's linker layout, runtime, and panic setup.

## Related Topics {#related-topics .skip}

- [Hardware Abstraction](../hardware-support/index.md) for choosing a HAL and target.
- [Concurrency](../concurrency/index.md) for reasoning about shared state.
