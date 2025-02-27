# Peripheral Devices

## USB

[`rusb`][c-rusb]⮳{{hi:rusb}} is a library for accessing USB devices.

## Serial Communication

[`serialport`][c-serialport]⮳{{hi:serialport}} works with serial ports.

## Networking

Standard networking crates like `std::net` or crates like [`tokio`][c-tokio]⮳{{hi:tokio}} for asynchronous networking are used for network hardware interaction.

## Graphics Cards (GPUs)

[`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}, [`vulkano`][c-vulkano]⮳{{hi:vulkano}}: These crates are used for interacting with GPUs (see the [[graphics | Graphics]] section).

## General Hardware Interaction

[`ioctl`][c-ioctl]⮳{{hi:ioctl}}: A crate for performing 'ioctl' (input/output control) operations on file descriptors. Often used for interacting with device drivers or hardware.

For specialized hardware, you'll often need to find or create crates that provide access to the device's API or protocol.

## See also

- [[embedded | Embedded Systems]]
- [[os | OS]]

### Device Drivers

Writing device drivers in Rust is complex and often involves unsafe code and close interaction with the operating system kernel. You'll typically need to work with platform-specific driver APIs.

### Hardware Access (User Space)

Accessing hardware from user space (without a driver) is often restricted for security reasons. It might involve using system calls or interacting with specific device files. See [[os | OS]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 write
</div>
