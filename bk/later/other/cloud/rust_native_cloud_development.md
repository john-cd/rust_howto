# Rust-native Cloud Platforms that Offer First-class Support for Rust

{{#include rust_native_cloud_development.incl.md}}

While it is common to deploy Rust applications to Cloud services like [AWS][p-aws], Azure and GCP, a new breed of [Cloud][p-cloud] services specific tailored to the needs of Rust developers is emerging.

## Deploy Rust Code on `shuttle.dev` {#shuttle}

[shuttle.rs][shuttle-rs-website]{{hi:shuttle.rs}}⮳ [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

`Shuttle.dev` is a Rust-native cloud development platform that simplifies backend development and deployment using Rust. Here's a summary of its key capabilities:

- Infrastructure-as-Code: You can define your infrastructure directly within your Rust code using annotations (no infrastructure files). `Shuttle` handles provisioning resources like [databases][p-databases], secrets, and storage.
- Rapid Deployment: your Rust applications can be deployed quickly and easily to the [cloud][p-cloud] with minimal configuration.
- Framework Support: Shuttle is compatible with popular Rust frameworks ([`Axum`][c-axum]⮳{{hi:Axum}}, `Actix Web`, [`Rocket`][c-rocket]⮳{{hi:Rocket}}, [`Warp`][c-warp]⮳{{hi:Warp}}, [`Tower`][c-tower]⮳{{hi:Tower}} and more). The Discord Bot building frameworks `Serenity` and `Poise` are also officially supported.
- Out-of-the-box logging support.

It offers a free tier with access to essential features for individual developers and small projects.

```rust,editable
{{#include ../../../crates/other/examples/cloud/shuttle.rs:example}}
```

## References

- [`shuttle.dev`][shuttle-dev-website]⮳{{hi:shuttle}}
- [docs.shuttle.dev][docs-shuttle-dev]{{hi:shuttle.dev}}
- [Shuttle examples][shuttle-examples-github]

## Related Topics

- [[aws | AWS]].
- [[development_tools | Development Tools]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/576)
</div>
