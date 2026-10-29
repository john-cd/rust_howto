<!-- TODO Reviewed ??-??-2026 -->

# Rust-native Cloud Development

{{#include rust_native_cloud_development.incl.md}}

Rust-native cloud development uses Rust APIs, build tools, or WebAssembly components to describe application behavior and integrate it with a hosting environment. A Rust service can also run on an ordinary VM or container platform; a Rust-first SDK does not remove the need to manage configuration, credentials, resource limits, and deployment lifecycle.

## Evaluate a Rust-first Platform {#shuttle}

Shuttle historically combined Rust framework integrations with resource provisioning. Its [official shutdown notice][shuttle~shutdown] says hosted operations are ceasing: the Community migration deadline was the beginning of 2026 and Pro projects stopped on January 16, 2026. The old free-tier claim and deployment recommendation are therefore obsolete. Do not choose the historical hosted service for a new application.

The retained source illustrates how the former Shuttle runtime wrapped an Axum router. It is a historical integration example, not a currently verified deployment recipe. Its runtime test is ignored because it needs the provider runtime; the ordinary Axum handler should be tested independently when migrating an application.

```rust,ignore
{{#include ../../../crates/other/examples/cloud/shuttle.rs:example}}
```

Keep business logic and request handlers independent of a provider entry point. For a new service, start with the [Axum chapter][cloud~axum], then package it for a supported container or serverless target. Select that target from its current runtime, region, persistence, pricing, and operational requirements.

## Run WebAssembly in the Cloud

[Spin][cloud~spin] provides an application model for WebAssembly components, including Rust SDK support and local development commands. [wasmCloud][cloud~wasmcloud] combines components with host capabilities and distributed application operation. Their component interfaces and deployment models differ; test the exact capability providers and host configuration the application needs.

See [standalone Wasm runtimes](../../categories/wasm/wasm_standalone_runtimes.md) for embedding and WASI, and distinguish a core Wasm module from a component with WIT-defined interfaces.

## Write Serverless Functions

Use the [AWS Lambda examples](aws.md) for AWS's Rust runtime and event handling. [Cloudflare Workers' Rust guide][cloud~workers-rust] uses the `workers-rs` SDK and compiles Rust to WebAssembly for the Workers environment. Audit filesystem, networking, async, and dependency assumptions for that runtime rather than treating it as a native Linux process.

Keep credentials in the hosting environment's supported secret mechanism. Verify local handler behavior separately from deployment permissions and remote service access.

## Build Kubernetes Operators

The [`kube` project][cloud~kube] supplies Kubernetes clients and controller-runtime facilities in Rust. Controllers reconcile desired and observed state, so their operations need to tolerate retries and partial progress. See the [Kubernetes example](kubernetes.md) for custom resource definitions and controller integration.

## Related Topics {#related-topics .skip}

- [Cloud overview](index.md) and [AWS](aws.md).
- [Containers](../../categories/virtualization/containers.md).
- [WebAssembly runtimes](../../categories/wasm/wasm_standalone_runtimes.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
