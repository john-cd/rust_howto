# Embedded Systems

[![cat-embedded][cat-embedded-badge]][cat-embedded]{{hi:Embedded systems}}

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
| `no-std` Environments | `core` (part of the standard library) provides the bare minimum for writing code without the standard library. `alloc` provides memory allocation APIs for no-std environments. See [[no_std | No Std]] and [[no-std_no-alloc | No Alloc]]. |
| Memory Management (in no-std) | `alloc` provides allocation APIs. [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} is a small and efficient allocator for embedded systems. See [[memory-management | Memory Management]] and [[memory_usage_analysis | Memory Usage Analysis]]. |
| Concurrency | [`atomic`][c-atomic]⮳{{hi:atomic}}: Provides atomic types for safe concurrency in embedded systems. See [[concurrency | Concurrency]]. |
| Debugging | `OpenOCD`, [`GDB`][c-gdb]⮳{{hi:GDB}} are common debugging tools for embedded systems. See [[development-tools_debugging | Debugging]]. |
| Cross-compiling Rust Code for Commonly Used Target Architectures (e.g., ARM, RISC-V) | See [[cross_compilation | Cross Compilation]] and [[cross_compiling | Cross Compiling]]. |

## References

- [Embedded devices working group][embedded-devices-working-group]⮳.
- [Rust Raspberry Pi OS tutorials][rust-raspberrypi-OS-tutorials-github]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; cover](https://github.com/john-cd/rust_howto/issues/346)
</div>
