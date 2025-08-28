# Containerization

{{#include containerization.incl.md}}

## Docker Container Integration {#docker-container .skip}

- [Shiplift][c~shiplift~crates.io]↗: A high-level API for interacting with the Docker daemon, managing containers, images, and networks.
- [Bollard][c~bollard~crates.io]↗: An asynchronous Rust client library for the Docker API, offering extensive features for container management.
- [Dockworker][c~dockworker~crates.io]↗: A Docker client library for managing containers and images.

## Kubernetes and Orchestration {#kubernetes .skip}

- [Kube][c~kube~crates.io]↗: A Kubernetes client for managing containerized applications in Kubernetes clusters.

## Container Internals {#container-internals .skip}

While [`runc`][c~runc~docs]↗{{hi:runc}} itself is written in Go, Rust crates like [`oci-spec`][c~oci-spec~crates.io]↗{{hi:oci-spec}} are used for working with OCI (Open Container Initiative) specifications, which are fundamental to containerization. Many container-related tools are being developed in Rust.

### Container Runtime {#container-runtime .skip}

'containerd' is an industry-standard container runtime. It is available as a daemon for Linux and Windows, which can manage the complete container lifecycle of its host system: image transfer and storage, container execution and supervision, low-level storage and network attachments, etc.

### Container Image Management {#container-image .skip}

- [OCI Spec][c~oci-spec~crates.io]↗: A crate for working with the Open Container Initiative (OCI) specifications.

### Networking for Containers {#networking-for-containers .skip}

- [Netavark][c~netavark~crates.io]↗: A container network stack written in Rust, designed for Podman.

### Container Build Tools {#container-build-tools .skip}

Mention https://github.com/moby/buildkit

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1225)
need in-depth review.
cover [podman][podman~repo]↗.
cover [containerd-rust-extensions][c~runc~repo]↗: A collection of Rust crates to extend containerd.

- [cgroups-rs][c~cgroups~crates.io]↗.

</div>
