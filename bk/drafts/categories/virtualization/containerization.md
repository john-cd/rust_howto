# Containerization

## Docker Container Integration

- [Shiplift](https://crates.io/crates/shiplift): A high-level API for interacting with the Docker daemon, managing containers, images, and networks.
- [Bollard](https://crates.io/crates/bollard): An asynchronous Rust client library for the Docker API, offering extensive features for container management.
- [Dockworker](https://crates.io/crates/dockworker): A Docker client library for managing containers and images.

## Kubernetes and Orchestration

- [Kube](https://crates.io/crates/kube): A Kubernetes client for managing containerized applications in Kubernetes clusters.

## Container Internals

While [`runc`][c-runc]⮳{{hi:runc}} itself is written in Go, Rust crates like `oci-spec` are used for working with OCI (Open Container Initiative) specifications, which are fundamental to containerization. Many container-related tools are being developed in Rust.

### Container Runtime

'containerd' is an industry-standard container runtime. It is available as a daemon for Linux and Windows, which can manage the complete container lifecycle of its host system: image transfer and storage, container execution and supervision, low-level storage and network attachments, etc.

### Container Image Management

- [OCI Spec](https://crates.io/crates/oci-spec): A crate for working with the Open Container Initiative (OCI) specifications.

### Networking for Containers

- [Netavark](https://crates.io/crates/netavark): A container network stack written in Rust, designed for Podman.

### Container Build Tools

Mention https://github.com/moby/buildkit

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
need in-depth review
cover https://github.com/containers/podman
cover https://github.com/containerd/rust-extensions A collection of Rust crates to extend containerd.
</div>
