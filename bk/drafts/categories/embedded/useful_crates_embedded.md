# Useful Crates for Embedded Systems Programming

{{#include useful_crates_embedded.incl.md}}

## `postcard` {#postcard}

[![postcard][c-postcard-badge]][c-postcard] [![postcard-crates.io][c-postcard-crates.io-badge]][c-postcard-crates.io] [![postcard-github][c-postcard-github-badge]][c-postcard-github] [![postcard-lib.rs][c-postcard-lib.rs-badge]][c-postcard-lib.rs]{{hi:postcard}}{{hi:Cobs}}{{hi:Framing}}{{hi:Serde}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[`postcard`][c-postcard]⮳{{hi:postcard}} is a `no_std` and [`serde`][c-serde]⮳{{hi:serde}} compatible message library for Rust.

## `smoltcp` {#smoltcp}

[![smoltcp-website][c-smoltcp-website-badge]][c-smoltcp-website] [![smoltcp][c-smoltcp-badge]][c-smoltcp] [![smoltcp-crates.io][c-smoltcp-crates.io-badge]][c-smoltcp-crates.io] [![smoltcp-github][c-smoltcp-github-badge]][c-smoltcp-github] [![smoltcp-lib.rs][c-smoltcp-lib.rs-badge]][c-smoltcp-lib.rs]{{hi:smoltcp}}{{hi:Ethernet}}{{hi:Ip}}{{hi:Network}}{{hi:Tcp}}{{hi:Udp}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}}

[`smoltcp`][c-smoltcp]⮳{{hi:smoltcp}} is a TCP/IP stack designed for bare-metal, real-time systems without a heap.

{{#example smoltcp}}

## `portable-atomic` {#portable-atomic}

[![portable-atomic][c-portable_atomic-badge]][c-portable_atomic] [![portable-atomic-crates.io][c-portable_atomic-crates.io-badge]][c-portable_atomic-crates.io] [![portable-atomic-github][c-portable_atomic-github-badge]][c-portable_atomic-github] [![portable-atomic-lib.rs][c-portable_atomic-lib.rs-badge]][c-portable_atomic-lib.rs]{{hi:portable-atomic}}{{hi:Atomic}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]{{hi:Hardware support}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}}

[`portable-atomic`][c-portable_atomic]⮳{{hi:portable-atomic}} provides portable atomic types, including support for 128-bit atomics, atomic float, etc.

## Other Useful Crates {#skip}

- [`nb`][c-nb]⮳{{hi:nb}}: Non-blocking I/O helpers.
- [`defmt`][c-defmt]⮳{{hi:defmt}}: A logging framework designed for embedded systems.
- [`panic-halt`][c-panic_halt]⮳{{hi:panic-halt}}: Halts the microcontroller on panic.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
