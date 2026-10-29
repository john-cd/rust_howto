<!-- TODO Reviewed ??-??-2026 -->

# Batteries-included Frameworks

Choose a batteries-included framework or architecture when you want common application components and conventions provided up front. Loco and Cot are application frameworks; Rust on Nails is a full-stack architecture and project template, not a framework crate. Compare their current releases, Rust version requirements, generated structure, and included services before adopting one.

{{#include _batteries-included_frameworks.incl.md}}

## Loco {#loco}

[![loco-rs][c~loco-rs~docs~badge]][c~loco-rs~docs]{{hi:loco-rs}}{{hi:HTTP server}}
[![loco-rs~crates.io][c~loco-rs~crates.io~badge]][c~loco-rs~crates.io]
[![loco-rs~repo][c~loco-rs~repo~badge]][c~loco-rs~repo]
[![loco-rs~lib.rs][c~loco-rs~lib.rs~badge]][c~loco-rs~lib.rs]
[![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`loco`][c~loco-rs~docs]↗{{hi:loco}} is an Axum-based, Rails-inspired application framework. Its CLI generates a project structure for routes, database models, background jobs, mailers, authentication, and tasks. The example implements `Hooks::boot` by delegating to `create_app`; its route lives under `/home/hello`. Build the optional integration with `--features loco-support`. To run a complete service, put the hooks and configuration in a generated Loco application and start it with `cargo loco start`.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/loco/main.rs:example}}
```

## Cot {#cot}

[![cot][c~cot~docs~badge]][c~cot~docs] [![cot~crates.io][c~cot~crates.io~badge]][c~cot~crates.io] [![cot~repo][c~cot~repo~badge]][c~cot~repo]{{hi:cot}}{{hi:HTTP server}}

[Cot][c~cot~docs]↗ is a batteries-included framework built on Axum. It provides an ORM and migrations, an admin panel, templates, authentication, sessions, and project tooling. Use it when those integrated features fit your application; consult the [Cot guide][cot~guide]↗ for current setup steps and feature requirements.

## Rust on Nails {#rust-on-nails}

[![rust-on-nails~repo][rust-on-nails~repo~badge]][rust-on-nails~repo] [Rust on Nails][rust-on-nails~website]↗{{hi:Rust on Nails}}

Rust on Nails is an opinionated full-stack architecture and project template. It documents how to combine established Rust components into an application and development workflow; use its guide and source repository to review the current stack and setup.

## References {#references .skip}

- [Loco guide][c~loco-rs~website]↗ and [Loco repository][c~loco-rs~repo]↗.
- [Cot guide][cot~guide]↗ and [Cot repository][c~cot~repo]↗.
- [Rust on Nails website][rust-on-nails~website]↗ and [source repository][rust-on-nails~repo]↗.
- [Building a SaaS with Rust and Next.js][blog~building-a-saas-with-rust-and-next-js]↗.

## Related Topics {#related-topics .skip}

- [HTTP server index][p~web-servers] for framework and server options.
- [Other web frameworks][p~other-http-frameworks] for Rocket, Poem, Salvo, and Warp.
- [Axum][ex~web-programming_http-server~axum], which underlies Loco and Cot.
- [Database access][p~database] for drivers, ORMs, and migrations.
- [Authentication][p~authentication] for identity and access-control recipes.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
