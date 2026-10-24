<!-- TODO Reviewed ??-??-2026 -->

# Cloud

Rust appears in cloud computing in two ways: teams build applications with Rust and run
them on cloud platforms, and Rust is used to build parts of the cloud infrastructure
itself. Use this page to find the relevant walkthroughs and examples.

## Run Rust applications on cloud services

### AWS

The [AWS chapter][p~aws]↗ covers the Rust SDK for AWS services and an AWS Lambda
function. Its recipe table links to both examples.

{{#include aws.incl.md}}

### Azure and Google Cloud

Rust client libraries are also available for Azure and Google Cloud. Check each
project's current service coverage, Rust version support, and stability before choosing
an SDK.

- [Azure SDK for Rust][azure-sdk-for-rust~repo]↗.
- [Google Cloud Rust client libraries][google-cloud-rust~repo]↗.

## Deploy Rust applications

The [Rust-native cloud development chapter][ex~cloud~shuttle]↗ includes a Shuttle
deployment example. See the [Shuttle examples][shuttle-examples~repo]↗ for additional
starting points. [Fly.io's Rust hosting guide][fly~rust~hosting~guide]↗ describes a
separate deployment option.

{{#include rust_native_cloud_development.incl.md}}

## Integrate a distributed application runtime

[Dapr][dapr~rust~sdk~docs]↗ provides building blocks for distributed applications
that run across cloud and edge environments. Its current Rust SDK documentation marks
the client library as alpha, so check its status and API stability before adopting it.

## Build cloud infrastructure with Rust

Rust is also used in infrastructure that runs or supports cloud applications. Examples
include:

- Container runtimes such as [`Youki`][youki~repo]↗{{hi:Youki}}.
- Service meshes such as [`Linkerd`][linkerd~website]↗{{hi:Linkerd}}.
- MicroVMs such as [`Firecracker`][firecracker~repo]↗{{hi:Firecracker}}.
- Container-focused operating systems such as [`Bottlerocket`][c~bottlerocket~docs]↗{{hi:Bottlerocket}}.

Browse the [Rust cloud-native project catalog][rust-cloud-native~website]↗ and
[[written-in-rust | Written in Rust]] for more examples.

## Related topics

- [[architecture | Software Architecture]] and system design.
- [[data-processing | Data Processing]] and storage.
- [[devops | DevOps]] and delivery.
- [[network-programming | Networking]] and service communication.
- [[virtualization | Virtualization]] and isolation.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[edit](https://github.com/john-cd/rust_howto/issues/579)
[content audit: content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1904)
</div>
