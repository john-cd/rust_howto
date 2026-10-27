<!-- TODO Reviewed ??-??-2026 -->

# Virtualization

[![cat~virtualization][cat~virtualization~badge]][cat~virtualization]{{hi:Virtualization}}

Creation and management of virtual environments and resources of any form, including containerization systems.

The Rust virtualization ecosystem is still under development, but there's growing interest and activity.

## Pages

- [[virtualization | Virtualization]]: virtual machines, hypervisors, `virt` (libvirt) and Firecracker.
- [[containers | Containers]], [[containerization | Containerization]] and [[using_containers | Using Containers]]: build, run and manage containers.

## Virtual Machine Monitors and Building Blocks

| Project                                                              | Role             | Notes                                                                  |
| -------------------------------------------------------------------- | ---------------- | ---------------------------------------------------------------------- |
| [Firecracker][firecracker~repo]↗{{hi:Firecracker}}                   | VMM              | Lightweight microVMs; see also its [`jailer`][jailer~lib.rs]↗.         |
| [`crosvm`][crosvm~repo]↗{{hi:crosvm}}                                | VMM              | Chrome OS VMM focused on security.                                     |
| [Cloud Hypervisor][cloud-hypervisor~website]↗{{hi:cloud-hypervisor}} | VMM              | Built for cloud workloads.                                             |
| [Hyperlight][hyperlight~repo]↗{{hi:Hyperlight}}                      | Micro-VM library | Runs functions in hypervisor-isolated micro-VMs.                       |
| [`rust-vmm`][rust-vmm~community~repo]↗{{hi:rust-vmm}}                | Components       | Shared crates for building custom VMMs.                                |
| [`kvm-ioctls`][rust-vmm~kvm~repo]↗{{hi:kvm-ioctls}}                  | KVM wrapper      | Safe wrappers over the KVM API.                                        |
| [`kvm-bindings`][c~kvm-bindings~crates.io]↗{{hi:kvm-bindings}}       | KVM FFI          | `bindgen`-generated KVM bindings.                                      |
| [`vm-memory`][c~vm-memory~docs]↗{{hi:vm-memory}}                     | Guest memory     | Abstractions for a virtual machine's guest memory.                     |
| [`vmm-sys-util`][rust-vmm~vmm-sys-util~repo]↗{{hi:vmm-sys-util}}     | Utilities        | Helpers shared by `rust-vmm` components.                               |
| [`virt`][c~virt~docs]↗{{hi:virt}}                                    | libvirt bindings | Manage QEMU/KVM and other hypervisors through `libvirt`.               |
| [`tun-tap`][c~tun-tap~docs]↗{{hi:tun-tap}}                           | Networking       | Virtual network interfaces.                                            |
| [`polkavm`][c~polkavm~crates.io]↗{{hi:polkavm}}                      | Emulator         | RISC-V VM for smart contracts; see [Emulators](../emulators/index.md). |

[`QEMU`][wikipedia~QEMU]↗{{hi:QEMU}} and [`KVM`][c~kvm~docs]↗{{hi:KVM}} remain the usual integration points. Rust's memory safety is a strong advantage for security-sensitive virtualization code.

## Code Examples

{{#include virtualization.incl.md}}

## Related Topics

| Topic                                    | Rust Crates                                                                   | Notes                                                             |
| ---------------------------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| Sandboxing with [Wasm](../wasm/index.md) | [`wasmi`][c~wasmi~docs]↗{{hi:wasmi}}, [`wasi-rs`][c~wasi~docs]↗{{hi:wasi-rs}} | `wasmi` interprets WebAssembly; `wasi-rs` provides WASI bindings. |
| [Emulators](../emulators/index.md)       | [`qemu-rs`][c~qemu~docs]↗{{hi:qemu-rs}}                                       | QEMU bindings for emulation and virtualization.                   |
| FFI                                      |                                                                               | See [Development Tools: FFI](../development-tools_ffi/index.md).  |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
