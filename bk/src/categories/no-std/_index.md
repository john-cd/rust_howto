# No Standard Library

[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No std}}

Crates that are able to function without the Rust standard library.

{{#include no_std.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[no-std/_index: write (P2)](https://github.com/john-cd/rust_howto/issues/427)

## Key Concepts

No Standard Library: You don't have access to the standard library ([`std`][c-std]⮳{{hi:std}}), which means no file I/O, dynamic memory allocation (unless you provide it), threading, or other OS-dependent features.

- Minimal Runtime: You'll need to provide your own runtime environment, including a panic handler.
- Hardware Interaction: You'll typically interact directly with hardware peripherals.
- Memory Management: You might need to manage memory manually, especially if you need dynamic allocation.
- Concurrency: If you're working with multiple tasks or interrupts, you'll need to consider concurrency carefully.

When working in `no_std`, you'll almost always use the `core` crate. If you need dynamic allocation, you'll also use the `alloc` crate and provide an allocator. For embedded systems, you'll need to choose the appropriate HAL and PAC crates for your target microcontroller. Remember to handle panics appropriately. Consider other useful crates like `nb`, [`defmt`][c-defmt]⮳{{hi:defmt}}, and [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}} as needed.

## Key Crates

- Core Library (`core`): This is the foundation of `no_std` programming. It provides the bare minimum functionality required for Rust code, including basic types, traits, and memory management primitives. It's always used in `no_std` environments.

- Allocation (`alloc`): If you need dynamic memory allocation in `no_std`, the `alloc` crate provides the necessary APIs. It's important to note that you'll need to provide your own allocator implementation.

- Panic Handling: In `no_std` environments, you can't rely on the standard library's panic handling mechanisms. You'll need to define your own panic handler. Crates like [`panic-halt`][c-panic_halt]⮳{{hi:panic-halt}} (halts execution) or [`panic-abort`][c-panic_abort]⮳{{hi:panic-abort}} (aborts execution) can be helpful. You might also implement custom panic behavior.

Link to:

## Embedded HALs (Hardware Abstraction Layers)

These are crucial for interacting with hardware in embedded systems.

- [`embedded-hal`][c-embedded_hal]⮳{{hi:embedded-hal}}: Defines common traits for peripherals (GPIO, SPI, I2C, UART, etc.). Essential for portable embedded code.
- [`cortex-m`][c-cortex_m]⮳{{hi:cortex-m}}: Provides access to Cortex-M microcontroller peripherals.
- `stm32fxxx-hal`: HALs for specific STM32 microcontrollers. (Many microcontroller families have their own HAL crates.)
- (Many other microcontroller families have their own HAL crates.)

## Peripheral Access Crates (PACs)

These provide direct access to microcontroller registers. You'll often use a HAL on top of a PAC.

- `stm32fxxx-pac`: PACs for STM32 microcontrollers.

Similar PACs exist for most microcontroller families.

## Other Useful Crates for `no_std`

- `nb`: Non-blocking I/O helpers.
- [`defmt`][c-defmt]⮳{{hi:defmt}}: A logging framework designed for embedded systems.
- [`wee_alloc`][c-wee_alloc]⮳{{hi:wee_alloc}}: A small and efficient allocator for embedded systems or WebAssembly.
- [`atomic`][c-atomic]⮳{{hi:atomic}}: Provides atomic types for safe concurrency in embedded systems.

</div>
