# Embedded Systems

[![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded systems}}

Crates that are primarily useful on embedded devices or without an operating system.

## Key Considerations

- Often no operating systems.
- Low-power design to conserve energy.
- Real-time constraints.
- Concurrency: management of multiple tasks.

## Embassy

{{#include embassy.incl.md}}

## Embedded Frameworks/HALs (Hardware Abstraction Layers)

{{#include hals.incl.md}}

## Peripheral Access Crates (PACs)

{{#include pacs.incl.md}}

## Real-Time Operating Systems (RTOS)

{{#include rtos.incl.md}}

## Interfacing with Sensors (e.g. I2C)

{{#include sensors.incl.md}}

## Flash Programming

{{#include flash.incl.md}}

## Data Acquisition and Calibration

FIXME

## Memory-mapped I/O

FIXME
Microcontrollers commonly interact with peripherals via memory-mapped I/O.

## Interrupts

FIXME
Microcontrollers respond to events via interrupts.

## Useful Crates for Embedded Systems Programming

{{#include useful_crates_embedded.incl.md}}

## Related Topics

| Topic | Rust Crates |
|---|---|
| [`no-std`]( ){{hi: }} Environments | `core` (part of the standard library) provides the bare minimum for writing code without the standard library. `alloc` provides memory allocation APIs for no-std environments. See [[no_std | No Std]] and [[no-std_no-alloc | No Alloc]]. |
| Memory Management (in no-std) | [`alloc`](https://doc.rust-lang.org/alloc)↗{{hi:alloc}} provides allocation APIs. [`wee_alloc`][c~wee_alloc~docs]↗{{hi:wee_alloc}} is a small and efficient allocator for embedded systems. See [[memory-management | Memory Management]] and [[memory_usage_analysis | Memory Usage Analysis]]. |
| Concurrency | [`atomic`][c~atomic~docs]↗{{hi:atomic}}: Provides atomic types for safe concurrency in embedded systems. See [[concurrency | Concurrency]]. |
| Debugging | [`OpenOCD`][openocd~website]↗{{hi:OpenOCD}}, [`GDB`][c~gdb~docs]↗{{hi:GDB}} are common debugging tools for embedded systems. See [[development-tools_debugging | Debugging]]. |
| Cross-compiling Rust Code for Commonly Used Target Architectures (e.g., ARM, RISC-V) | See [[cross_compilation | Cross Compilation]] and [[cross_compiling | Cross Compiling]]. |

## References

- [Embedded devices working group][rust-lang~embedded-devices-working-group]↗.
- [Rust Raspberry Pi OS tutorials][rust-raspberrypi-OS-tutorials~github]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; cover](https://github.com/john-cd/rust_howto/issues/346)

- [awesome-embedded-rust: Curated list of resources for Embedded and Low-level development in the Rust programming language][awesome-embedded-rust~github]
- [Are We RTOS Yet?][arewertosyet~website]

## Libs

- [embassy: Modern embedded framework, using Rust and async.][embassy~github]
- [rust-iot-platform: A high-performance IoT development platform built with Rust, designed for multi-protocol support and real-time data processing. This platform supports MQTT, WebSockets (WS), TCP, and CoAP protocols, making it highly flexible for diverse IoT applications.][rust-iot-platform~github]
- [stm32-rs: Embedded Rust device crates for STM32 microcontrollers][stm32-rs~github]
- [rtic: Real-Time Interrupt-driven Concurrency (RTIC) framework for ARM Cortex-M microcontrollers][c~rtic~github]

## Panic Handling {#skip}

- [better-panic: A Python-inspired `panic` handler for rust][better-panic~github]
- [No-Panic Rust: A Nice Technique for Systems Programming][blog~no-panic]
- [no-panics-whatsoever (crates.io)][c~no-panics-whatsoever~crates.io]

</div>
