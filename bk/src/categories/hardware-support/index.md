# Hardware Support

[![cat-hardware-support][cat-hardware-support-badge]][cat-hardware-support]

Interface with specific CPU{{hi:CPU}} or other hardware{{hi:Hardware}} features.

For embedded systems, HAL crates are essential. For operating system interaction, you'll often use system calls. For external peripherals, you'll need to find or create crates that provide access to the specific device. Writing device drivers is a complex topic and requires a deep understanding of the operating system and hardware.

## Processor Management

{{#include processor.incl.md}}

## Peripheral Devices

{{#include peripherals.incl.md}}

## See also

### Memory Management

See [[memory-management | Memory Management]].

### Operating System Interaction - System Calls

- `std::os::unix` (Linux, macOS): Provides access to Unix-specific system calls.
- `std::os::windows` (Windows): Provides access to Windows-specific system calls.

See [[os_freebsd-apis | FreeBSD APIs]], [[os_linux-apis | Linux APIs]], [[os_macos-apis | macOS APIs]], [[os_unix-apis | Unix APIs]], [[os_windows-apis | Windows APIs]] and [[rust_os | Rust OSes]].

### `no_std` Environments

- `core`: (Standard library) Provides the bare minimum for writing code without the standard library.
- `alloc`: (Standard library) Provides memory allocation APIs for `no-std` environments.

### Embedded Systems

See [[embedded | Embedded]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 expand hardware-support](https://github.com/john-cd/rust_howto/issues/70)

- Memory-mapped I/O: How you interact with hardware registers in embedded systems.
- Interrupts: How hardware signals the CPU in embedded systems.
- System calls: How user-space programs request services from the operating system kernel.
- Device drivers: Software that interfaces between the operating system and hardware.
- Hardware protocols: How devices communicate with each other.

</div>
