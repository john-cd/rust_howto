<!-- TODO Reviewed ??-??-2026 -->

# No `std`

{{#include no_std.incl.md}}

[`#![no_std]`][rust~no_std~attribute] prevents a crate from automatically linking Rust's standard library. The crate uses [`core`][c~core~docs], which provides primitive types, `Option`, `Result`, iterators, atomics, and formatting. `no_std` does not by itself prohibit heap allocation: a program can also use [`alloc`][c~alloc~docs] when its final binary supplies a global allocator.

## Write a library for both `std` and `no_std`

A library can enable `no_std` unless its `std` Cargo feature is selected. Dependencies that enable `std` by default should disable those defaults and re-enable the dependency's `std` feature from your crate's feature:

```toml
[features]
default = ["std"]
std = ["serde/std"]

[dependencies]
serde = { version = "1", default-features = false, features = ["derive"] }
```

Put the conditional attribute at the crate root (`src/lib.rs`), as in this workspace's `no_std` library:

```rust,ignore
#![cfg_attr(not(feature = "std"), no_std)]

mod checksum;
pub use checksum::checksum;
```

The separate module implements a wrapping byte checksum without allocation or operating-system calls:

```rust,editable
{{#include ../../../crates/cats/no_std/src/checksum.rs:example}}
```

With these defaults, downstream users get `std` unless they disable default features. Omit `default = ["std"]` when downstream users should opt in to `std`. If the library needs a `Vec` or `String`, gate that API behind an `alloc` feature and use `alloc` types; the final application still chooses the allocator.

## Use heap collections with `alloc`

`alloc` provides owning collections such as `Vec` and `String`, but it expects a global allocator to be registered by the final program. A `no_std` library should use `extern crate alloc` and `alloc` types without selecting the allocator for its users.

For a Cortex-M binary, [`embedded-alloc`][c~embedded-alloc~docs] provides a heap. Initialize it before constructing heap-backed values, and select a `critical-section` implementation appropriate for the chip and runtime:

```rust,ignore
#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use cortex_m_rt::entry;
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // The allocator must be initialized before the first allocation.
    unsafe {
        embedded_alloc::init!(HEAP, 4096);
    }

    let mut samples = Vec::new();
    samples.push(42_u16);

    loop {}
}
```

If fixed capacity is enough, prefer the [[no-std_no-alloc | No Alloc]] patterns instead of adding a heap.

## Provide a panic handler for a bare-metal binary

A bare-metal binary does not get the startup code or panic handler supplied by `std`. Use the entry-point macro and linker setup for the selected HAL or runtime, then provide a panic handler. This isolated sketch is intentionally not compiled:

```rust,ignore
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}
```

For board setup, startup, and probe workflows, see [[embedded | Embedded Systems]].

## Check that a crate builds without `std`

Install a target supported by the selected HAL, then build with default features disabled. For a Cortex-M4F target, for example:

```powershell
rustup target add thumbv7em-none-eabihf
cargo build --no-default-features --target thumbv7em-none-eabihf
```

Check the dependency feature tree as well: one dependency that enables `std` by default can prevent the no-std build. The workspace's checksum library has no dependencies and a `std` feature enabled by default. From `later/crates`, compile only its library without that feature, or run its separate host example:

```sh
cargo build -p no_std --lib --no-default-features
cargo build -p no_std --lib --no-default-features --target thumbv7em-none-eabihf
cargo run -p no_std --example no_std
```

A consumer can select the core-only library with `no_std = { path = "../no_std", default-features = false }` in its dependencies. The host example below uses `std` for printing; the library does not supply a panic handler or entry point, because those belong to the final binary. The bare-metal target command requires the target installation described above.

## Core-only example {#no-std}

```rust,editable
{{#include ../../../crates/cats/no_std/examples/no_std/checksum.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[no-std_no-alloc | No Alloc]].
- [[embedded | Embedded Systems]].
- [[memory-management | Memory Management]].
- [[development_tools | Development Tools]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [no_std: write](https://github.com/john-cd/rust_howto/issues/426)
</div>
