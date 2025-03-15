# Configure Code Based on Compiler Support

{{#include build_cache.incl.md}}

## `autocfg` {#autocfg}

[![autocfg][c-autocfg-badge]][c-autocfg] [![autocfg-crates.io][c-autocfg-crates.io-badge]][c-autocfg-crates.io] [![autocfg-github][c-autocfg-github-badge]][c-autocfg-github] [![autocfg-lib.rs][c-autocfg-lib.rs-badge]][c-autocfg-lib.rs]{{hi:autocfg}}{{hi:Autoconf}}{{hi:Build}}{{hi:Rustc}} [![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]{{hi:Build Utils}}

Automatic cfg for Rust compiler features

A Rust library for build scripts to automatically configure code based on compiler support. Code snippets are dynamically tested to see if the rustc will accept them, rather than hard-coding specific version support.

Add to your `Cargo.toml`:

```toml
[build-dependencies]
autocfg = "1.0.0" # Or latest version
```

```rust,editable
fn main() {
    let ac = autocfg::new();
    ac.emit_has_type("i128");

    // (optional) We don't need to rerun for anything external.
    autocfg::rerun_path("build.rs");
}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
