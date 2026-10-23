<!-- TODO Reviewed ??-??-2026 -->
# Virtualization

{{#include virtualization.incl.md}}

Virtualization lets a host provide isolated computing environments. A virtual machine (VM) runs a guest operating system with a virtual hardware interface; a container packages an application and its user-space files while sharing the host kernel. Choose the boundary that matches the workload's kernel, compatibility, and isolation needs.

## Virtual machines and hypervisors {#virtual-machines}

A hypervisor or virtual machine monitor (VMM) creates and manages VMs. It presents virtual CPUs, memory, storage, and network devices to a guest operating system. Hardware virtualization support and a compatible host operating system are usually required.

### Firecracker microVMs {#firecracker}

[Firecracker][firecracker~repo] is an open-source VMM for lightweight microVMs. It combines a minimal virtual device model with a separate guest kernel, aiming for VM isolation with a smaller resource footprint than a general-purpose VM. Firecracker runs on Linux hosts and uses KVM for guest vCPUs; its design documentation describes the architecture and recommends starting production workloads through its jailer.

Firecracker is an example of a VMM built in Rust. Read the [Firecracker design document][firecracker-design] before deploying it, and check its current host and kernel requirements.

### Other VMMs and components

The [rust-vmm project][rust-vmm~community~repo] maintains Rust components for building virtual machine monitors. [Cloud Hypervisor][cloud-hypervisor~website] is another VMM focused on cloud workloads. QEMU can emulate hardware and use KVM acceleration when available; see [[emulators | Emulators]] for related tools.

## Containers {#containers}

Containers isolate application processes and package the files they need, while containers on a host share its operating-system kernel. A VM instead runs a guest operating system with its own kernel. The [Docker explanation of containers and VMs][container-vm-comparison] gives a concise comparison. Containers and VMs can also be combined: a VM can host a container runtime and multiple application containers.

{{#include containers.incl.md}}

### Docker {#docker}

- [Docker][docker~website]↗.
- [Docker's Rust guide][docker~rust-guide]↗.
- [Docker Desktop][docker~desktop~website]↗.

### Docker Compose {#docker-compose}

[Docker Compose][docker~compose~website]↗ describes a multi-container application in a Compose file. To scaffold a Rust project, you can run cargo init and docker init in its directory, then review the generated Dockerfile and compose.yaml before using them.

~~~sh
cargo init .
docker init
~~~

Build and start the services with `docker compose up --build`. If the deployment host uses a different CPU architecture, select the target platform when building the image, such as `docker build --platform=linux/amd64 -t myapp .`. See [[cloud | Cloud]] for deployment context.

For container APIs and runtimes written in Rust, see [[containerization | Containerization]]. For containers in integration tests, see [[using_containers | Using Containers]].

## Rust virtualization components

Rust code can implement VMMs and the components around them, such as device models, guest-memory handling, and host control APIs. The [rust-vmm project][rust-vmm~community~repo] provides reusable components; a complete VMM still needs a host integration, guest boot path, device and network model, lifecycle management, and a threat model. Start with the project documentation and the target hypervisor's requirements before choosing individual crates.

## WebAssembly runtime sandboxing {#virt}

A WebAssembly runtime runs a module inside a constrained execution environment. It is useful for sandboxing portable modules, but it does not virtualize a complete operating system or replace a VM's guest-kernel boundary. Limit the host functions and resources made available to untrusted modules.

The example below loads a small WebAssembly module with wasmi and calls its exported function.

~~~rust,editable
{{#include ../../../crates/cats/virtualization/examples/virtualization/virtualization1.rs:example}}
~~~

## Choose an isolation boundary

| Need | Starting point | Boundary to understand |
| --- | --- | --- |
| Package an application and its user-space dependencies | Container image and runtime | Containers share a host kernel; configure kernel isolation and resource limits. |
| Run a guest operating system or a different kernel | A VM or microVM managed by a hypervisor | The guest has its own kernel; the host still needs a secure VMM and lifecycle controls. |
| Run portable, constrained modules | A WebAssembly runtime such as wasmi | The runtime exposes selected host capabilities; it does not provide a guest operating system. |

## Related Topics {#related-topics .skip}

- [[virtualization | Virtualization overview]].
- [[containers | Rust + Containers]].
- [[containerization | Containerization]].
- [[using_containers | Using Containers]].
- [[emulators | Emulators]].
- [[wasm | WebAssembly]].
- [[cloud | Cloud]].
- [[os_linux-apis | Linux APIs]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
