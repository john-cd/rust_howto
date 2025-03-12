# Cloud

This chapter covers the use of the Rust language in Cloud computing.

## Implement Rust applications on the AWS Cloud

{{#include aws.incl.md}}

It is similarly possible to use Azure, GCP (Google Cloud Platform), and other cloud providers.
All major Cloud platforms offer a Rust SDK:

- [Azure SDK for Rust](https://github.com/Azure/azure-sdk-for-rust).
- [Google Cloud Platform Rust [Experimental] Client Libraries](https://github.com/googleapis/google-cloud-rust).

## Rust-native Cloud platforms that offer first-class support for Rust

{{#include rust_native_cloud_development.incl.md}}

## Related Topics

### Rust as the basis for Cloud-Native Technologies

Rust is commonly used to develop reliable and efficient cloud infrastructure components, such as [[virtualization | Virtualization]], [[network-programming | networking]] services and storage systems. That includes core cloud-native technologies, such as:

- Container runtimes (e.g., `Youki`),
- Service meshes (e.g., `Linkerd`),
- MicroVMs (e.g., `Firecracker`),
- Operating systems optimized for containers (e.g., `Bottlerocket`).

Refer to:

- [[written-in-rust | Written in Rust]].
- [rust-cloud-native.github.io](https://rust-cloud-native.github.io)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[edit](https://github.com/john-cd/rust_howto/issues/579)
</div>
