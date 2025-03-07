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

## Flash programming

{{#include flash.incl.md}}

## Useful crates for embedded systems programming

{{#include useful_crates_embedded.incl.md}}

## See also

- [Embedded devices working group][embedded-devices-working-group]⮳.
- [Rust Raspberry Pi OS tutorials][rust-raspberrypi-OS-tutorials-github]⮳.

### `no-std` Environments

`core`: (Standard library) Provides the bare minimum for writing code without the standard library.
`alloc`: (Standard library) Provides memory allocation APIs for no-std environments.

See [[no_std | No Std]] and [[no-std_no-alloc | No Alloc]]

### Memory Management (in no-std)

`alloc`: (Standard library) Provides allocation APIs.
[`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}: A small and efficient allocator for embedded systems.

See [[memory-management | Memory Management]] and [[memory_usage_analysis | Memory Usage Analysis]].

### Concurrency

[`atomic`][c-atomic]⮳{{hi:atomic}}: Provides atomic types for safe concurrency in embedded systems.

See [[concurrency | Concurrency]].

### Debugging

`OpenOCD`, [`GDB`][c-gdb]⮳{{hi:GDB}}: Common debugging tools for embedded systems.

See [[development-tools_debugging | Debugging]].

## Cross-compiling Rust code for target architectures commonly used (e.g., ARM, RISC-V)

See [[cross_compilation | Cross Compilation]] and [[cross_compiling | Cross Compiling]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; cover](https://github.com/john-cd/rust_howto/issues/346)

## Data acquisition and calibration

## Memory-mapped I/O

How you interact with peripherals.

## Interrupts

How the microcontroller responds to events.

</div>
