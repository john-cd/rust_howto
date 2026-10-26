<!-- TODO Reviewed ??-??-2026 -->

# Hardware Support

[![cat~hardware-support][cat~hardware-support~badge]][cat~hardware-support]

Rust hardware support ranges from selecting CPU-specific instructions to communicating with peripherals and embedded boards. Start with portable, safe APIs where they fit; isolate target-specific code behind platform or feature checks.

| Area                                   | Common approach                                                           | Guidance                                                                                                                                        |
| -------------------------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| CPU information and features           | [`std::thread`][c~std::thread~docs]↗ and [`std::arch`][c~std::arch~docs]↗ | Check whether a CPU feature is available before calling an architecture-specific intrinsic. See [[processor | Processor]].                      |
| SIMD                                   | Standard-library architecture intrinsics or a portable SIMD API           | Keep a scalar fallback when a program must run on CPUs without the selected instructions. See [[processor#simd | SIMD]].                        |
| USB and serial devices                 | [`rusb`][c~rusb~docs]↗ and [`serialport`][c~serialport~docs]↗             | Device access can require OS permissions, drivers, or platform-specific setup. See [[peripherals | Peripheral Devices]].                        |
| Graphics hardware                      | [`wgpu`][c~wgpu~docs]↗                                                    | Use a graphics or compute API instead of mapping a GPU directly; see [[graphics | Graphics]].                                                   |
| Embedded boards                        | `no_std` support and a board or chip HAL                                  | Use the board's peripheral abstractions and cross-compile for its target. See [[no_std | `no_std` Rust]].                                       |
| Memory-mapped registers and interrupts | A chip PAC/HAL and its documented interrupt model                         | These low-level operations require careful ownership and synchronization; avoid hand-written register access when a maintained HAL provides it. |
| OS device drivers                      | OS-specific driver interfaces and system APIs                             | Drivers may require privileged code and unsafe operations. Ordinary applications should use the OS device interface or a crate that wraps it.   |

## Processor Management

{{#include processor.incl.md}}

## Peripheral Devices

{{#include peripherals.incl.md}}

## Memory-mapped I/O

Embedded firmware can interact with device registers through a peripheral access crate (PAC) or hardware abstraction layer (HAL). Follow the device documentation for register ownership, ordering, and access width; Rust's ordinary references do not by themselves make memory-mapped I/O safe.

## Interrupts

Interrupt handlers respond to device or timer events. Keep them short and use the chip's documented interrupt and synchronization APIs when sharing state with normal code.

## Device Drivers

Device drivers translate operating-system requests into device operations. Their interfaces and privilege requirements are platform-specific; application code usually accesses hardware through an existing driver and its user-space API.

## Hardware Protocols

Hardware protocols define how devices exchange commands and data. Prefer an existing protocol implementation when available; when implementing one, handle framing, timeouts, and malformed input at the transport boundary.

## Related Topics

### Memory Management

See [[memory-management | Memory Management]].

### Operating System Interaction - System Calls

User-space programs request services from the operating system kernel via system calls.

- [`std::os::unix`][c~std::os::unix~docs]↗{{hi:std::os::unix}} provides access to Unix-specific system calls.
- [`std::os::windows`][c~std::os::windows~docs]↗{{hi:std::os::windows}} provides access to Windows-specific system calls.

See [[os_freebsd-apis | FreeBSD APIs]], [[os_linux-apis | Linux APIs]], [[os_macos-apis | macOS APIs]], [[os_unix-apis | Unix APIs]], [[os_windows-apis | Windows APIs]] and [[rust_os | Rust OSes]].

### `#![no_std]` Environments

- [`core`][c~core~docs]↗{{hi:core}} provides the bare minimum for writing code without the standard library.
- [`alloc`][c~alloc~docs]↗{{hi:alloc}} provides memory allocation APIs for `no-std`{{hi:no-std}} environments.

See [[no_std | no `std`]].

### Embedded Systems

Use `no_std` targets with the board's documented HAL and toolchain. See [[no_std | `no_std` Rust]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
expand hardware-support, write missing sections

- [Multiversion][c~multiversion~lib.rs]↗ - Rust HW library.

</div>
