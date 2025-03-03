# Virtualization

[![cat-virtualization][cat-virtualization-badge]][cat-virtualization]{{hi:Virtualization}}

Creation and management of virtual environments and resources of any form, including containerization systems.

The Rust virtualization ecosystem is still under development, but there's growing interest and activity.

## Containerization

See [[containerization | Containerization]].

## Hypervisor / Virtual Machine Monitors (VMMs)

- `Firecracker`: A lightweight VMM designed for running microVMs.
- `CrosVM`: A VMM for Chrome OS, focusing on security and simplicity.
- Cloud Hypervisor: `cloud-hypervisor` is a VMM specifically designed for cloud workloads.

## Virtualization Frameworks

These crates provide low-level interfaces for interacting with hypervisors like KVM. These are often used to build higher-level VMMs.

- `rust-vmm`: A collection of virtualization components for building custom VMMs.
- `vmm-sys-util`: Utilities for system programming in VMMs.
- `kvm-ioctls`: Safe wrappers over KVM ioctls for interacting with the Linux Kernel-based Virtual Machine.

## Device Emulation

- `virtio-devices`: Implementation of Virtio device models.
- `vhost`: Libraries for implementing vhost-based backends.

## Kernel and Bootloaders

- `linux-loader`: A library for loading Linux kernels.
- `bootparam`: Utilities for working with Linux boot parameters.

## Memory Management

- `vm-memory`: Abstractions for managing guest memory in VMMs.
- `guest-memory`: Safe and efficient guest memory management.

## Virtual Networking

- `net_util`: Utilities for network device emulation.
- `vhost-user-backend`: A library for implementing vhost-user backends.
- `slirp` (bindings), [`tun-tap`][c-tun_tap]⮳{{hi:tun-tap}}

`slirp` provides a user-mode network stack. [`tun-tap`][c-tun_tap]⮳{{hi:tun-tap}} allows creating virtual network interfaces.

## Virtual Storage

Virtual storage solutions are an area where development is ongoing.

- `block-util`: Utilities for block device emulation.
- `virtio-blk`: Virtio block device implementation.

| Image Formats (Virtual Disks) | (Often uses existing image libraries) | Working with virtual disk image formats (e.g., qcow2, vmdk) often involves using existing image processing libraries or creating custom parsers. |

## Security

- `seccomp`: A library for managing seccomp filters in VMMs.
- `sandbox`: Tools for sandboxing virtualized environments.

## Code Examples

{{#include virtualization.incl.md}}

## Key Considerations

- The Rust virtualization ecosystem is under active development. Some areas are more mature than others. Low-level VMM interaction is relatively well-supported, but higher-level frameworks are still emerging.
- Rust's performance, safety, and concurrency features make it well-suited for virtualization.
- Virtualization requires in-depth security and Rust's memory safety is a significant advantage.
- Rust is also relevant for virtualization on embedded systems.
- Integrating with existing virtualization technologies (e.g., KVM, QEMU) is a common approach.

## Related Topics

| Topic | Rust Crates | Notes |
|---|---|---|
| Sandboxing with [[wasm | WASM]] | [`wasmi`][c-wasmi]⮳{{hi:wasmi}}, [`wasi-rs`][c-wasi]⮳{{hi:wasi-rs}} | [`wasmi`][c-wasmi]⮳{{hi:wasmi}} is a WebAssembly interpreter, useful for sandboxing untrusted code. [`wasi-rs`][c-wasi]⮳{{hi:wasi-rs}} provides bindings for the WebAssembly System Interface (WASI). |
| [[emulators | Emulators]] | [`qemu-rs`][c-qemu]⮳{{hi:qemu-rs}} (bindings) | [`qemu-rs`][c-qemu]⮳{{hi:qemu-rs}} provides bindings to QEMU, a powerful emulator that can be used for virtualization. |
| FFI | | See [[development-tools_ffi | Development Tools: FFI]], [[external-ffi-bindings | External FFI Bindings]]. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/968)
need in-depth review
cover

https://github.com/rust-vmm/community
https://github.com/rust-vmm/vmm-sys-util
https://github.com/rust-vmm/kvm
https://crates.io/crates/kvm-bindings
https://github.com/rust-vmm/kvm/tree/main/kvm-ioctls

https://github.com/firecracker-microvm/firecracker/
https://firecracker-microvm.github.io/
https://lib.rs/gh/firecracker-microvm/firecracker/jailer

https://github.com/google/crosvm

https://github.com/hyperlight-dev/hyperlight

https://en.wikipedia.org/wiki/QEMU

Mention https://lib.rs/crates/polkavm (in Emulators) (smart contracts)

Mention `virt` / `libvirt` C lib?
</div>
