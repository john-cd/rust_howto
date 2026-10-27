<!-- TODO Reviewed ??-??-2026 -->

## OCI Runtimes and Specifications {#oci-runtimes}

The [OCI Runtime Specification][oci-runtime-spec~repo]↗ defines the bundle and lifecycle interface used by compatible container runtimes. [![youki][youki~repo~badge]][youki~repo]↗ is a Rust implementation of that interface for Linux; its upstream project documents current status, adopters, and Linux-only local builds. [`runc`][runc~repo]↗ is the widely used Go implementation.

## containerd and Rust Extensions {#containerd}

[![containerd Rust extensions][containerd~rust-extensions~repo~badge]][containerd~rust-extensions~repo]↗ provides Rust crates for containerd runtime-v2 shims, clients, logging, and snapshotters. These extend the [containerd][containerd~repo]↗ daemon; they are not a standalone container engine. Building the upstream extension workspace requires Protocol Buffers' `protoc` compiler.

## Linux Resource Controls and Namespaces {#linux-isolation}

[![cgroups-rs][c~cgroups-rs~crates.io~badge]][c~cgroups-rs~crates.io]↗ manages Linux control groups (cgroups), which group processes and apply resource limits. Its current documentation exposes both v1 and v2 hierarchy APIs; available controllers and permissions still depend on the host.

[![nix][c~nix~crates.io~badge]][c~nix~docs]↗ and [![rustix][c~rustix~crates.io~badge]][c~rustix~docs]↗ expose low-level system calls for namespace operations. Current APIs include `nix::sched::unshare` (with the `sched` feature) and `rustix::thread::unshare_unsafe` (with the `thread` feature). These crates are building blocks, not container managers: creating a usable container also involves mounts, user mappings, capabilities, networking, and cgroup configuration. Required privileges and kernel support vary by operation and host policy.

## Container Networking and Image Builds {#networking-and-builds}

[![Netavark][netavark~repo~badge]][netavark~repo]↗ is a Rust container-network stack used by Podman. [![BuildKit][buildkit~repo~badge]][buildkit~repo]↗ is a general-purpose, Dockerfile-compatible image builder; for a Rust application's Dockerfile and Rust container clients, see [[using_containers | Using Containers]].

## Podman {#podman}

[![Podman][podman~repo~badge]][podman~repo]↗ is a daemonless, OCI-compatible tool for building and managing images, containers, and pods. It can use OCI runtimes such as `runc` or `crun`; use [[using_containers | Using Containers]] for application-level workflows.

## MicroVM Isolation {#microvms}

[![Firecracker][firecracker~repo~badge]][firecracker~repo]↗ is a Rust virtual machine monitor for lightweight microVMs. It uses KVM on Linux and provides a guest-kernel boundary, so consider it when VM isolation is needed; it is not an OCI container runtime.

This page has no compiled examples: exercising runtime and namespace primitives depends on Linux kernel features, host configuration, and privileges. For running containers from Rust code or building a Rust image, see [[using_containers | Using Containers]].
