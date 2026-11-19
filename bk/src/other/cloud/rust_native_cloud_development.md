<!-- TODO Reviewed ??-??-2026. -->

# Deploy Rust Applications

{{#include rust_native_cloud_development.incl.md}}

Start with what the host needs to run. A web service listens for requests, an event handler runs when invoked, and a controller watches for changes. Each deployment model gives the application a different entry point and way to obtain configuration and credentials.

## Choose a Deployment Model {#shuttle}

| Workload | Starting point | Check before choosing |
| --- | --- | --- |
| Long-running HTTP service | Native binary in a container or VM | Port binding, health checks, shutdown, and image platform. |
| Event handler | Provider function runtime | Event schema, timeout, retries, and permissions. |
| WebAssembly component | Host that implements the required WIT interfaces | Host capabilities, interface versions, and deployment limits. |
| Kubernetes controller | A local cluster and the Kubernetes API | CRD compatibility, service-account permissions, and cleanup. |

Keep business logic and request handlers independent of a provider entry point. For a new service, start with the [Axum chapter][cloud~axum], then package it for a supported container or serverless target. Select that target from its current runtime, region, persistence, pricing, and operational requirements.

For a local container workflow, follow [Using Containers](../../categories/virtualization/using_containers.md). For an HTTP service, bind to the interface the host can reach, read the port from its configuration, and verify a health endpoint through the mapped port. A working request from the host checks both the server and the container's port mapping.
For historical context, Shuttle paired Axum integration with managed hosting.
See its [shutdown notice][shuttle~shutdown] when maintaining an existing project.

## Run WebAssembly in the Cloud

[Spin][cloud~spin] provides an application model for WebAssembly components, including Rust SDK support and local development commands. [wasmCloud][cloud~wasmcloud] combines components with host capabilities and distributed application operation. Their component interfaces and deployment models differ; test the exact capability providers and host configuration the application needs.

See [standalone Wasm runtimes](../../categories/wasm/wasm_standalone_runtimes.md) for embedding and WASI, and distinguish a core Wasm module from a component with WIT-defined interfaces.

## Write Serverless Functions

Use the [local Lambda walkthrough][ex~cloud~lambda] for AWS's Rust runtime and event handling. [Cloudflare Workers' Rust guide][cloud~workers-rust] uses the `workers-rs` SDK and compiles Rust to WebAssembly for the Workers environment. Audit filesystem, networking, async, and dependency assumptions for that runtime rather than treating it as a native Linux process.

Keep credentials in the hosting environment's supported secret mechanism. Verify local handler behavior separately from deployment permissions and remote service access.

## Build Kubernetes Operators

The [`kube` project][cloud~kube] supplies Kubernetes clients and controller-runtime facilities in Rust. Controllers reconcile desired and observed state, so their operations need to tolerate retries and partial progress. The [kube-rs controller guide][cloud~kube-controller] covers watches, ownership, and error policies.

## Related Topics {#related-topics .skip}

- [Cloud overview](index.md) and [AWS](aws.md).
- [Containers](../../categories/virtualization/containers.md).
- [WebAssembly runtimes](../../categories/wasm/wasm_standalone_runtimes.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
