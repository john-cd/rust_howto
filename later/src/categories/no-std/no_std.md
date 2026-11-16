<!-- TODO Reviewed ??-??-2026. -->

# Bare-metal Startup and Allocation

See the [No Standard Library chapter](https://john-cd.com/rust_howto/categories/no-std/no_std.html) for the checksum library and Cargo feature setup.

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

## Provide a panic handler for a bare-metal binary {#no-std}

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

## Related Topics {#related-topics .skip}

- [[no-std_no-alloc | No Alloc]].
- [[embedded | Embedded Systems]].
- [[memory-management | Memory Management]].
- [[development_tools | Development Tools]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
