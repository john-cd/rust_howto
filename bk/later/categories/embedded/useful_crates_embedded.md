# Useful Crates for Embedded Systems Programming

{{#include useful_crates_embedded.incl.md}}

## `postcard` {#postcard}

[![postcard][c~postcard~docs~badge]][c~postcard~docs] [![postcard~crates.io][c~postcard~crates.io~badge]][c~postcard~crates.io] [![postcard~repo][c~postcard~repo~badge]][c~postcard~repo] [![postcard~lib.rs][c~postcard~lib.rs~badge]][c~postcard~lib.rs]{{hi:postcard}}{{hi:Cobs}}{{hi:Framing}}{{hi:Serde}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`postcard`][c~postcard~docs]↗{{hi:postcard}} is a [`#![no_std]`][book~rust-reference~no_std] and [`serde`][c~serde~docs]↗{{hi:serde}} compatible message library for Rust.

## `smoltcp` {#smoltcp}

[![smoltcp~website][c~smoltcp~website~badge]][c~smoltcp~website] [![smoltcp][c~smoltcp~docs~badge]][c~smoltcp~docs] [![smoltcp~crates.io][c~smoltcp~crates.io~badge]][c~smoltcp~crates.io] [![smoltcp~repo][c~smoltcp~repo~badge]][c~smoltcp~repo] [![smoltcp~lib.rs][c~smoltcp~lib.rs~badge]][c~smoltcp~lib.rs]{{hi:smoltcp}}{{hi:Ethernet}}{{hi:Ip}}{{hi:Network}}{{hi:Tcp}}{{hi:Udp}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}}

[`smoltcp`][c~smoltcp~docs]↗{{hi:smoltcp}} is a TCP/IP stack designed for bare-metal, real-time systems without a heap.

{{#example smoltcp}}

## `portable-atomic` {#portable-atomic}

[![portable-atomic][c~portable-atomic~docs~badge]][c~portable-atomic~docs] [![portable-atomic~crates.io][c~portable-atomic~crates.io~badge]][c~portable-atomic~crates.io] [![portable-atomic~repo][c~portable-atomic~repo~badge]][c~portable-atomic~repo] [![portable-atomic~lib.rs][c~portable-atomic~lib.rs~badge]][c~portable-atomic~lib.rs]{{hi:portable-atomic}}{{hi:Atomic}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~hardware-support][cat~hardware-support~badge]][cat~hardware-support]{{hi:Hardware support}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}}

[`portable-atomic`][c~portable-atomic~docs]↗{{hi:portable-atomic}} provides portable atomic types, including support for 128-bit atomics, atomic float, etc.

## Other Useful Crates {#other-useful-crates .skip}

- [`nb`][c~nb~docs]↗{{hi:nb}}: Non-blocking I/O helpers.
- [`defmt`][c~defmt~docs]↗{{hi:defmt}}: A logging framework designed for embedded systems.
- [`panic-halt`][c~panic-halt~docs]↗{{hi:panic-halt}}: Halts the microcontroller on panic.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1201)
</div>
