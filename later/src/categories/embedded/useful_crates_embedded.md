<!-- TODO Reviewed ??-??-2026 -->

# Useful Crates for Embedded Systems Programming

{{#include useful_crates_embedded.incl.md}}

Embedded projects have different constraints from desktop programs: some targets have no allocator, limited flash and RAM, strict power budgets, or no operating system. Choose crates around the exact chip and execution model, and check each crate's `no_std`, allocation, target, and `embedded-hal` compatibility before adding it.

## Bounded Collections with `heapless` {#heapless}

[`heapless`][c~heapless~docs]↗{{hi:heapless}} provides collection types with capacity fixed in the type or value, so an application can use queues, maps, and vectors without a global allocator. The bound makes memory use visible, but it also means the application must decide what happens when a collection fills: reject input, overwrite old data, or apply backpressure.

Use a bounded queue between an interrupt or producer and a worker only after choosing its capacity from the maximum burst and service rate. Do not treat a fixed capacity as proof that data can never be lost.

The example below combines `heapless` with `postcard`: it encodes messages into fixed-size buffers, queues them in a four-slot `Deque`, and rejects new messages when the queue is full. It runs on the host; the same code compiles for `no_std` targets.

```rust,editable
{{#include ../../../crates/cats/embedded/examples/useful_crates_embedded/main.rs:example}}
```

## `postcard` {#postcard}

{{#badges postcard embedded="Embedded development" no-std="No standard library"}}{{hi:Cobs}}{{hi:Framing}}{{hi:Serde}}

[`postcard`][c~postcard~docs]↗{{hi:postcard}} is a [`#![no_std]`][book~rust-reference~no_std] and [`serde`][c~serde~docs]↗{{hi:serde}} compatible message library for Rust.

It is useful for compact messages in constrained systems. Define a stable message schema when data persists across firmware versions, and include framing, versioning, and integrity checks when the transport or storage format requires them.

## `smoltcp` {#smoltcp}

[![smoltcp~website][c~smoltcp~website~badge]][c~smoltcp~website] {{#badges smoltcp}} {{hi:Ethernet}} {{hi:Ip}} {{hi:Network}} {{hi:Tcp}} {{hi:Udp}} {{#categories network-programming="Network programming",embedded="Embedded development"}}

[`smoltcp`][c~smoltcp~docs]↗{{hi:smoltcp}} is a TCP/IP stack designed for bare-metal, real-time systems without a heap.

The following host-side example checks whether IPv4 addresses belong to a configured subnet. It exercises `smoltcp`'s address primitives without opening a socket or requiring a network device.

```rust,noplayground
{{#include ../../../crates/cats/embedded/examples/smoltcp/main.rs:example}}
```

## `portable-atomic` {#portable-atomic}

{{#badges portable-atomic concurrency=Concurrency hardware-support="Hardware support" embedded="Embedded development" no-std="No standard library" no-std::no-alloc="No dynamic allocation"}}{{hi:Atomic}}

[`portable-atomic`][c~portable-atomic~docs]↗{{hi:portable-atomic}} provides atomic types for targets with limited native atomic support. Available widths and optional types depend on the target and enabled features.

Atomic widths and operations depend on the target. Check the crate's target support and critical-section configuration; an atomic type does not make a multi-step protocol or peripheral access safe by itself.

## Compact Target Logging with `defmt` {#defmt}

[`defmt`][c~defmt~docs]↗{{hi:defmt}} is a logging framework designed for embedded targets. Select a compatible transport and host-side decoder as part of the development setup, and keep log volume bounded so diagnostics do not consume the timing budget. Avoid logging secrets or data that should not leave the device.

## Flash Storage with `embedded-storage` {#embedded-storage}

The [`embedded-storage`][c~embedded-storage~docs]↗ traits give drivers and storage algorithms a common interface to non-volatile storage. See [Flash Programming](flash.md) for erase, alignment, partition, and power-loss considerations.

## Device Programming with `probe-rs` {#probe-rs}

[`probe-rs`][c~probe-rs~docs]↗{{hi:probe-rs}} provides tools and libraries for detecting, flashing, and debugging supported chips with a debug probe. Check target support and the board's debug connection before choosing a probe; see [Flash Programming](flash.md) for the firmware download workflow.

## Other Useful Crates {#other-useful-crates .skip}

- [`embedded-hal-async`]: Async peripheral traits for compatible HALs and drivers.
- [`embedded-hal-nb`]: Polling peripheral traits based on non-blocking operations.
- [`panic-halt`][c~panic-halt~docs]↗{{hi:panic-halt}}: A minimal panic handler that halts the processor; choose panic behavior deliberately for the product.

## Related Topics {#related-topics .skip}

- [Hardware Abstraction Layers](hals.md) and [sensors](sensors.md) provide peripheral and device-driver interfaces.
- [Real-Time Operating Systems](rtos.md) explain task scheduling and timing constraints.
- [Flash Programming](flash.md) covers firmware downloads and application storage.
- [No-std development](../no-std/index.md) covers allocator-free Rust targets.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  write
</div>
