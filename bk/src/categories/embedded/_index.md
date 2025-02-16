# Embedded

[![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded systems}}

Crates that are primarily useful on embedded devices or without an operating system.

## Embassy

{{#include embassy.incl.md}}

## See also

- [Embedded devices working group][embedded-devices-working-group]⮳
- [Rust Raspberry Pi OS tutorials][rust-raspberrypi-OS-tutorials-github]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[embedded/_index: write; cover: (P2)](https://github.com/john-cd/rust_howto/issues/346)

## `portable-atomic` {#portable-atomic}

[![portable-atomic][c-portable_atomic-badge]][c-portable_atomic] [![portable-atomic-crates.io][c-portable_atomic-crates.io-badge]][c-portable_atomic-crates.io] [![portable-atomic-github][c-portable_atomic-github-badge]][c-portable_atomic-github] [![portable-atomic-lib.rs][c-portable_atomic-lib.rs-badge]][c-portable_atomic-lib.rs]{{hi:portable-atomic}}{{hi:Atomic}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]{{hi:Hardware support}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}}

   `portable-atomic` provides portable atomic types, including support for 128-bit atomics, atomic float, etc.

{#example portable-atomic}

## `embedded-hal` {#embedded-hal}

[![embedded-hal][c-embedded_hal-badge]][c-embedded_hal] [![embedded-hal-crates.io][c-embedded_hal-crates.io-badge]][c-embedded_hal-crates.io] [![embedded-hal-github][c-embedded_hal-github-badge]][c-embedded_hal-github] [![embedded-hal-lib.rs][c-embedded_hal-lib.rs-badge]][c-embedded_hal-lib.rs]{{hi:embedded-hal}}{{hi:Hal}}{{hi:Io}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]{{hi:Hardware support}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

      `embedded-hal` is a Hardware Abstraction Layer (HAL) for embedded systems.

## `postcard` {#`postcard`}

[![postcard][c-postcard-badge]][c-postcard] [![postcard-crates.io][c-postcard-crates.io-badge]][c-postcard-crates.io] [![postcard-github][c-postcard-github-badge]][c-postcard-github] [![postcard-lib.rs][c-postcard-lib.rs-badge]][c-postcard-lib.rs]{{hi:postcard}}{{hi:Cobs}}{{hi:Framing}}{{hi:Serde}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[`postcard`][c-postcard]⮳{{hi:postcard}} is a `no_std` and `serde` compatible message library for Rust.

## `strum` {#strum}

[![strum][c-strum-badge]][c-strum] [![strum-crates.io][c-strum-crates.io-badge]][c-strum-crates.io] [![strum-github][c-strum-github-badge]][c-strum-github] [![strum-lib.rs][c-strum-lib.rs-badge]][c-strum-lib.rs]{{hi:strum}}{{hi:Enum}}{{hi:Macros}}{{hi:Proc-macros}}{{hi:String}} [![cat-parsing][cat-parsing-badge]][cat-parsing]{{hi:Parsing tools}} [![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

   `strum` provides helpful macros for working with enums and strings.

## `smoltcp` {#smoltcp}

[![smoltcp-website][c-smoltcp-website-badge]][c-smoltcp-website] [![smoltcp][c-smoltcp-badge]][c-smoltcp] [![smoltcp-crates.io][c-smoltcp-crates.io-badge]][c-smoltcp-crates.io] [![smoltcp-github][c-smoltcp-github-badge]][c-smoltcp-github] [![smoltcp-lib.rs][c-smoltcp-lib.rs-badge]][c-smoltcp-lib.rs]{{hi:smoltcp}}{{hi:Ethernet}}{{hi:Ip}}{{hi:Network}}{{hi:Tcp}}{{hi:Udp}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}}

   `smoltcp` is a TCP/IP stack designed for bare-metal, real-time systems without a heap.

## `rppal`

[![rppal][c-rppal-badge]][c-rppal] [![rppal-crates.io][c-rppal-crates.io-badge]][c-rppal-crates.io] [![rppal-github][c-rppal-github-badge]][c-rppal-github] [![rppal-lib.rs][c-rppal-lib.rs-badge]][c-rppal-lib.rs]{{hi:rppal}}{{hi:Embedded-hal}}{{hi:Embedded-hal-impl}}{{hi:Hal}}{{hi:Pi}}{{hi:Raspberry}} [![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]{{hi:Hardware support}} [![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded development}}

   `rppal` is an interface for the Raspberry Pi's GPIO, I2C, PWM, SPI and UART peripherals.

</div>
