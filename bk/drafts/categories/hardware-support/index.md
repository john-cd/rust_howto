# Hardware Support

[![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]

Interface with specific CPU{{hi:CPU}} or other hardware{{hi:Hardware}} features.

For operating system interaction, you'll often use system calls. For external peripherals, you'll need to find or create crates that provide access to the specific device. For embedded systems, HAL crates are essential.

## Processor Management

{{#include processor.incl.md}}

## Peripheral Devices

{{#include peripherals.incl.md}}

## Memory-mapped I/O

Interact with hardware registers in embedded systems.

FIXME

## Interrupts

Hardware signals the CPU through interrupts.

FIXME

## Device Drivers

Software that interfaces between the operating system and hardware.

## Hardware Protocols

FIXME how devices communicate with each other

## Related Topics

### Memory Management

See [[memory-management | Memory Management]].

### Operating System Interaction - System Calls

User-space programs request services from the operating system kernel via system calls.

- `std::os::unix` provides access to Unix-specific system calls.
- `std::os::windows` provides access to Windows-specific system calls.

See [[os_freebsd-apis | FreeBSD APIs]], [[os_linux-apis | Linux APIs]], [[os_macos-apis | macOS APIs]], [[os_unix-apis | Unix APIs]], [[os_windows-apis | Windows APIs]] and [[rust_os | Rust OSes]].

### `no_std` Environments

- `core` provides the bare minimum for writing code without the standard library.
- `alloc` provides memory allocation APIs for `no-std` environments.

See [[no_std | no `std`]].

### Embedded Systems

See [[embedded | Embedded]] Systems.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand hardware-support, write missing sections NOW](https://github.com/john-cd/rust_howto/issues/70)
</div>
