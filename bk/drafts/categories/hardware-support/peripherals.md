<!-- TODO Reviewed ??-??-2026 -->

# Peripheral Devices

Peripheral access usually goes through an operating-system driver. Choose a crate for the device's protocol or class, then check its supported platforms, required native libraries, and permissions before depending on it. User-space access is not the same as writing a kernel driver.

{{#include peripherals.incl.md}}

## USB {#usb .skip}

[`nusb`][c~nusb~docs]↗{{hi:nusb}} provides cross-platform, pure-Rust USB access with blocking and asynchronous APIs. [`rusb`][c~rusb~docs]↗{{hi:rusb}} wraps `libusb` and is another option when its ecosystem or platform support fits your device. Both expose low-level device operations; for standard USB classes such as HID, serial, audio, and mass storage, prefer the operating system's existing class driver and a class-specific API when available.

On Windows, `nusb` uses WinUSB, so the device or interface must be associated with that driver. On Linux, USB device-node permissions may require a narrowly scoped `udev` rule. Avoid broad rules that grant every local user access to every USB device.

## Serial Communication {#serial-communication .skip}

[`serialport`][c~serialport~docs]↗{{hi:serialport}} provides a cross-platform API for enumerating and opening serial ports, configuring line settings, and reading or writing bytes. Serial device names and permissions differ by operating system; on Linux, a user may need membership in the device's configured group or a `udev` rule. For devices that expose a standard USB serial interface, use the system driver rather than talking to USB endpoints directly.

## Human Interface Devices {#human-interface-devices .skip}

[`hidapi`][c~hidapi~docs]↗{{hi:hidapi}} offers access to USB and Bluetooth HID devices such as keyboards, controllers, and sensors. The operating system may reserve some devices or restrict which reports an application can send; consult the device and platform documentation before relying on a particular report or permission model.

## GPIO and Embedded I/O {#gpio-and-embedded-io .skip}

For Linux userspace GPIO, [`gpio-cdev`][c~gpio-cdev~docs]↗{{hi:gpio-cdev}} uses the kernel's GPIO character-device interface. [`linux-embedded-hal`][c~linux-embedded-hal~docs]↗{{hi:linux-embedded-hal}} adapts Linux device interfaces for [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}-based drivers. These interfaces still depend on the board's kernel drivers, available device nodes, and permissions. For bare-metal firmware, use the board or chip's peripheral access crate (PAC) or hardware abstraction layer (HAL) instead.

## Networking {#networking .skip}

Applications normally use the operating system's network stack through [`std::net`][c~std::net~docs]↗{{hi:std::net}} or asynchronous libraries such as [`tokio`][c~tokio~docs]↗{{hi:tokio}}. Raw access to network interfaces, packet capture, and interface configuration are platform-specific and may require elevated permissions; use a focused OS API crate rather than accessing a network adapter directly.

## Graphics Cards (GPUs) {#graphics-cards .skip}

[`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} and APIs such as [`vulkano`][c~vulkano~docs]↗{{hi:vulkano}} provide supported graphics and compute interfaces over the platform's GPU drivers. Use those APIs for rendering or compute work instead of accessing device registers or GPU memory directly; see [[graphics | Graphics]].

## General Hardware Interaction {#general-hardware-interaction .skip}

On Unix-like systems, [`nix`][c~nix~docs]↗{{hi:nix}} and [`rustix`][c~rustix~docs]↗{{hi:rustix}} provide APIs for selected operating-system calls; `nix` also exposes `ioctl` helpers behind its `ioctl` feature. An `ioctl` request is a platform- and driver-specific command, so use an existing safe wrapper when possible and follow the device driver's documented contract when one is necessary.

For specialized devices, look for a maintained crate that implements the device's protocol or wraps its OS interface. Check whether it uses the in-kernel driver, a vendor library, or direct user-space access, since those choices affect portability and permissions.

### Device Drivers {#device-drivers .skip}

Kernel drivers run with privileged access and must follow the target operating system's driver model, synchronization rules, and hardware specifications. They are separate from ordinary Rust applications and often require unsafe operations at the hardware boundary. For Linux kernel development, see the [Rust for Linux driver documentation][p~linux]; APIs and supported kernel versions are tied to the kernel tree being built.

### Hardware Access (User Space) {#hardware-access .skip}

In user space, an application normally opens an OS-provided device interface or device node and communicates through the operations that its driver exposes. On Linux, this can include files under `/dev`, with `read`, `write`, or a driver-specific `ioctl`; access is controlled by device permissions and system policy. On Windows, applications use device interfaces and the associated driver APIs. Prefer a crate that encapsulates those details, request only the permissions the application needs, and handle unavailable devices and permission errors explicitly.

## Related Topics {#related-topics .skip}

- [Embedded Rust][p~embedded].
- [Operating-system APIs][p~os].
- [Linux kernel drivers][p~linux].
- [Asynchronous Rust][p~asynchronous].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  Expanded the peripheral guide with USB, serial, HID, GPIO, networking, GPU, driver, and user-space access guidance, including platform permission boundaries and local topic references.
</div>
