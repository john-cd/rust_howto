<!-- TODO Reviewed ??-??-2026 -->

# Batteries-included Frameworks

{{#include batteries-included_frameworks.incl.md}}

## `loco` {#loco}

[![loco-rs][c~loco-rs~docs~badge]][c~loco-rs~docs]{{hi:loco-rs}}{{hi:HTTP server}}
[![loco-rs~crates.io][c~loco-rs~crates.io~badge]][c~loco-rs~crates.io]
[![loco-rs~repo][c~loco-rs~repo~badge]][c~loco-rs~repo]
[![loco-rs~lib.rs][c~loco-rs~lib.rs~badge]][c~loco-rs~lib.rs]
[![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:HTTP server}}

[`loco`][c~loco-rs~docs]↗{{hi:loco}} is a batteries-included framework that provides application hooks, route registration, and startup support. This example implements `Hooks::boot` by delegating to `create_app`; the routes live under `/home/hello`. Build the optional integration with `--features loco-support`. To run a complete service, place the hooks and configuration in a Loco application and start it with `cargo loco start`.

```rust,editable
{{#include ../../../crates/cats/web_programming_http_server/examples/loco/main.rs:example}}
```

## Rust on Nails {#rust-on-nails}

[Rust on Nails][rust-on-nails~website]↗{{hi:Rust on Nails}}.

## References {#references .skip}

- [Building a SaaS with Rust and Next.js][blog~building-a-saas-with-rust-and-next-js]↗.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [batteries-included_frameworks: write](https://github.com/john-cd/rust_howto/issues/509)
</div>
