# Cloud

This chapter covers the use of the Rust language in Cloud computing.

## Implement Rust Applications on the AWS Cloud

{{#include aws.incl.md}}

It is similarly possible to use Azure, GCP (Google Cloud Platform), and other cloud providers.
All major Cloud platforms offer a Rust SDK:

- [Azure SDK for Rust][azure-sdk-for-rust~github]↗.
- [Google Cloud Platform Rust [Experimental] Client Libraries][google-cloud-rust~github]↗.

## Rust-native Cloud Platforms that Offer First-class Support for Rust

{{#include rust_native_cloud_development.incl.md}}

## Related Topics

### Rust as the Basis for Cloud-Native Technologies

Rust is commonly used to develop reliable and efficient cloud infrastructure components, such as [[virtualization | Virtualization]], [[network-programming | networking]] services and storage systems. That includes core cloud-native technologies, such as:

- Container runtimes (e.g., [`Youki`][youki~github]↗{{hi:Youki}}),
- Service meshes (e.g., [`Linkerd`][linkerd~website]↗{{hi:Linkerd}}),
- MicroVMs (e.g., `Firecracker`),
- Operating systems optimized for containers (e.g., [`Bottlerocket`][c~bottlerocket~docs]↗{{hi:Bottlerocket}}).

Refer to:

- [[written-in-rust | Written in Rust]].
- [rust-cloud-native.github.io][rust-cloud-native~website]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[edit](https://github.com/john-cd/rust_howto/issues/579)
</div>
