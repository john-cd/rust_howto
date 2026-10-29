<!-- TODO Reviewed ??-??-2026 -->

# Rust and Containers

Containers package a process with its user-space dependencies while sharing the host kernel. Rust applications can be built into container images, control a Docker daemon, or provide lower-level runtime and orchestration components. Choose a page by the task:

- [Containerization](containerization.md) covers OCI runtimes, containerd extensions, cgroups, image builders, and other container infrastructure implemented in or usable from Rust.
- [Using Containers](using_containers.md) covers building application images, using Docker from Rust, and starting disposable services for integration tests.
- The [Virtualization walkthrough](virtualization.md#containers) introduces Docker and Docker Compose commands and compares containers with virtual machines.

Keep application-image and testing recipes separate from runtime implementation details. A native Rust binary still needs an image containing any dynamic libraries and runtime assets it uses; a small final image does not automatically make an application statically linked.

## Related Topics

- [Virtualization](virtualization.md).
- [Cloud deployment](../../other/cloud/index.md).
- [Containerization building blocks](containerization.md).
- [Application images and container tests](using_containers.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
