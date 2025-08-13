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

FIXME.

## Memory-mapped I/O

FIXME.
Microcontrollers commonly interact with peripherals via memory-mapped I/O.

## Interrupts

FIXME.
Microcontrollers respond to events via interrupts.

## Useful Crates for Embedded Systems Programming

{{#include useful_crates_embedded.incl.md}}

## Related Topics

| Topic | Rust Crates |
|---|---|
| [`no-std`][book~rust-reference~no_std]{{hi:no-std}} Environments | `core` (part of the standard library) provides the bare minimum for writing code without the standard library. `alloc` provides memory allocation APIs for no-std environments. See [[no_std | No Std]] and [[no-std_no-alloc | No Alloc]]. |
| Memory Management (in no-std) | [`alloc`][c~alloc~docs]↗{{hi:alloc}} provides allocation APIs. [`wee_alloc`][c~wee_alloc~docs]↗{{hi:wee_alloc}} is a small and efficient allocator for embedded systems. See [[memory-management | Memory Management]] and [[memory_usage_analysis | Memory Usage Analysis]]. |
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

- [awesome-embedded-rust][awesome-embedded-rust~github]↗: Curated list of resources for Embedded and Low-level development in the Rust programming language,
- [Are We RTOS Yet?][arewertosyet~website]↗.

## Libs

- [embassy][embassy~github]↗: Modern embedded framework, using Rust and async.
- [rust-iot-platform][rust-iot-platform~github]↗: A high-performance IoT development platform built with Rust, designed for multi-protocol support and real-time data processing. This platform supports MQTT, WebSockets (WS), TCP, and CoAP protocols, making it highly flexible for diverse IoT applications.
- [stm32-rs][stm32-rs~github]↗: Embedded Rust device crates for STM32 microcontrollers.
- [rtic][c~rtic~github]↗: Real-Time Interrupt-driven Concurrency (RTIC) framework for ARM Cortex-M microcontrollers.

## Panic Handling {#skip}

- [better-panic][better-panic~github]↗: A Python-inspired `panic` handler for rust.
- [No-Panic Rust][blog~no-panic]↗: A Nice Technique for Systems Programming.
- [no-panics-whatsoever (crates.io)][c~no-panics-whatsoever~crates.io]↗.

</div>
