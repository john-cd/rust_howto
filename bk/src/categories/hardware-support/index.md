# Hardware Support

[![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]

Interface with specific CPU{{hi:CPU}} or other hardware{{hi:Hardware}} features.

## Processor

{{#include processor.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 expand hardware-support/](https://github.com/john-cd/rust_howto/issues/70)

Show how to use crates to communicate with common sensors using protocols like I2C, SPI, or UART.
Handle data acquisition and calibration.
Include recipes on cross-compiling Rust code for target architectures commonly used (e.g., ARM, RISC-V).
Discuss no-std programming and memory management.

---

For embedded systems, the HAL crates are essential. For operating system interaction, you'll often use system calls. For external peripherals, you'll need to find or create crates that provide access to the specific device. Writing device drivers is a complex topic and requires a deep understanding of the operating system and hardware.

## Key Concepts

- Memory-mapped I/O: How you interact with hardware registers in embedded systems.
- Interrupts: How hardware signals the CPU in embedded systems.
- System calls: How user-space programs request services from the operating system kernel.
- Device drivers: Software that interfaces between the operating system and hardware.
- Hardware protocols: How devices communicate with each other.

## By Category

Here's a breakdown by category:

## Embedded Systems (Microcontrollers)

### HALs (Hardware Abstraction Layers)

- `embedded-hal`: A crucial crate that defines common traits for interacting with peripherals (GPIO, SPI, I2C, UART, etc.). This is essential for writing portable embedded code.
- `cortex-m`: Provides access to Cortex-M microcontroller peripherals.
- `stm32fxxx-hal`: HALs for specific STM32 microcontrollers. (Many microcontroller families have their own HAL crates.)
- `nrf52-hal`: HALs for Nordic Semiconductor nRF52 microcontrollers.
- `esp-hal`: HAL for Espressif chips.

Many other microcontroller families have their own HAL crates.

## Peripheral Access Crates (PACs)

These crates provide direct access to the microcontroller's registers. You'll often use a HAL on top of a PAC.

- `stm32fxxx-pac`: PACs for STM32 microcontrollers.

Similar PACs exist for most microcontroller families.

## `no_std` Environments

- `core`: (Standard library) Provides the bare minimum for writing code without the standard library.
- `alloc`: (Standard library) Provides memory allocation APIs for no-std environments.

## Interfacing with Sensors (e.g. I2C)

## Operating System Interaction (Linux, macOS, Windows)

### System Calls

- `std::os::unix` (Linux, macOS): Provides access to Unix-specific system calls.
- `std::os::windows` (Windows): Provides access to Windows-specific system calls.

### Device Drivers

Writing device drivers in Rust is complex and often involves unsafe code and close interaction with the operating system kernel. You'll typically need to work with platform-specific driver APIs.

### Hardware Access (User Space)

Accessing hardware from user space (without a driver) is often restricted for security reasons. It might involve using system calls or interacting with specific device files.

### Peripheral Devices (External Hardware)

#### USB

`rusb`: A library for accessing USB devices.

#### Serial Communication

`serialport`: A crate for working with serial ports.

#### Networking

Standard networking crates like `std::net` or crates like `tokio` for asynchronous networking are used for network hardware interaction.

#### Graphics Cards (GPUs)

`wgpu`, `gfx-hal`, `vulkano`: These crates are used for interacting with GPUs (see the Graphics section).

#### Other Peripherals

For specialized hardware, you'll often need to find or create crates that provide access to the device's API or protocol.

### General Hardware Interaction

`ioctl`: A crate for performing ioctl (input/output control) operations on file descriptors. Often used for interacting with device drivers or hardware.

</div>
