# Containerization

## Docker Container Integration {#skip}

- [Shiplift](https://crates.io/crates/shiplift): A high-level API for interacting with the Docker daemon, managing containers, images, and networks.
- [Bollard](https://crates.io/crates/bollard): An asynchronous Rust client library for the Docker API, offering extensive features for container management.
- [Dockworker](https://crates.io/crates/dockworker): A Docker client library for managing containers and images.

## Kubernetes and Orchestration {#skip}

- [Kube](https://crates.io/crates/kube): A Kubernetes client for managing containerized applications in Kubernetes clusters.

## Container Internals {#skip}

While [`runc`][c-runc]⮳{{hi:runc}} itself is written in Go, Rust crates like `oci-spec` are used for working with OCI (Open Container Initiative) specifications, which are fundamental to containerization. Many container-related tools are being developed in Rust.

### Container Runtime {#skip}

'containerd' is an industry-standard container runtime. It is available as a daemon for Linux and Windows, which can manage the complete container lifecycle of its host system: image transfer and storage, container execution and supervision, low-level storage and network attachments, etc.

### Container Image Management {#skip}

- [OCI Spec](https://crates.io/crates/oci-spec): A crate for working with the Open Container Initiative (OCI) specifications.

### Networking for Containers {#skip}

- [Netavark](https://crates.io/crates/netavark): A container network stack written in Rust, designed for Podman.

### Container Build Tools {#skip}

Mention https://github.com/moby/buildkit

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1225)
need in-depth review
cover [podman](https://github.com/containers/podman)
cover [containerd-rust-extensions](https://github.com/containerd/rust-extensions) A collection of Rust crates to extend containerd.
</div>
