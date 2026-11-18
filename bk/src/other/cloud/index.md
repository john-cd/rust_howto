<!-- TODO Reviewed ??-??-2026. -->

# Cloud

Rust appears in cloud computing in two ways: teams build applications with Rust and run
them on cloud platforms, and Rust is used to build parts of the cloud infrastructure
itself. Use this page to find the relevant walkthroughs and examples.

## Run Rust applications on cloud services

### AWS

The [AWS chapter][p~aws]↗ covers the Rust SDK for AWS services and an AWS Lambda
function. It also covers shared configuration, email, metrics, object storage, and messaging.

{{#include aws.incl.md}}

### Azure and Google Cloud

Choose the SDK client for the service you call, then choose how the app
authenticates on its deployment target.

| Provider | Credential and client starting point |
| --- | --- |
| AWS | Use [`aws-config`](https://docs.rs/aws-config/) with a service crate such as [`aws-sdk-s3`](https://docs.rs/aws-sdk-s3/). See [shared AWS configuration](aws.md#aws-shared-config) for the runnable recipe. |
| Azure | Use [`azure_identity`](https://docs.rs/azure_identity/) for credentials and select the service client from the [Azure SDK][azure-sdk-for-rust~repo]. |
| Google Cloud | Use [`google-cloud-auth`](https://docs.rs/google-cloud-auth/) and the relevant [Google Cloud service client][google-cloud-rust~repo]. |

The Azure and Google entries are SDK pointers. Check the target service,
permissions, and supported credential source before adopting a client.

## Deploy Rust applications

Use the [deployment guide][ex~cloud~shuttle] to choose a container, function runtime, WebAssembly host, or Kubernetes controller based on your workload.

{{#include rust_native_cloud_development.incl.md}}

## Operate Kubernetes

Use [`kube`][cloud~kube] for Kubernetes clients and controllers. A controller watches resources and reconciles the cluster with the state they describe.

## Integrate a distributed application runtime {#dapr}

[Dapr][dapr~rust~sdk~docs]↗ provides building blocks for distributed applications
that run across cloud and edge environments. Its current Rust SDK documentation marks
the client library as alpha, so check its status and API stability before adopting it. The [Rust SDK examples][dapr-rust~examples]↗ show how a Rust application calls into the runtime.

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
