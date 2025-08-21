# Peripheral Devices

## USB {#skip}

[`rusb`][c~rusb~docs]↗{{hi:rusb}} is a library for accessing USB devices.

## Serial Communication {#skip}

[`serialport`][c~serialport~docs]↗{{hi:serialport}} works with serial ports.

## Networking {#skip}

Standard networking like [`std::net`][c~std::net~docs]↗{{hi:std::net}} or crates like [`tokio`][c~tokio~docs]↗{{hi:tokio}} for asynchronous networking are used for network hardware interaction.

## Graphics Cards (GPUs) {#skip}

[`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}}, [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}}: These crates are used for interacting with GPUs (see the [[graphics | Graphics]] section).

## General Hardware Interaction {#skip}

`ioctl`: A crate for performing 'ioctl' (input/output control) operations on file descriptors. Often used for interacting with device drivers or hardware.

FIXME replace by `nix`.

For specialized hardware, you'll often need to find or create crates that provide access to the device's API or protocol.

## Related Topics {#related-topics}

- [[embedded | Embedded Systems]].
- [[os | OS]].

### Device Drivers {#skip}

Writing device drivers in Rust is complex and often involves unsafe code and close interaction with the operating system kernel. You'll typically need to work with platform-specific driver APIs.

### Hardware Access (User Space) {#skip}

Accessing hardware from user space (without a driver) is often restricted for security reasons. It might involve using system calls or interacting with specific device files. See [[os | OS]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1177)
</div>
