# Configure Code Based on Compiler Support

{{#include build_cache.incl.md}}

## `autocfg` {#autocfg}

[![autocfg][c~autocfg~docs~badge]][c~autocfg~docs] [![autocfg~crates.io][c~autocfg~crates.io~badge]][c~autocfg~crates.io] [![autocfg~repo][c~autocfg~repo~badge]][c~autocfg~repo] [![autocfg~lib.rs][c~autocfg~lib.rs~badge]][c~autocfg~lib.rs]{{hi:autocfg}}{{hi:Autoconf}}{{hi:Build}}{{hi:Rustc}} [![cat~development-tools::build-utils][cat~development-tools::build-utils~badge]][cat~development-tools::build-utils]{{hi:Build Utils}}

Automatic cfg for Rust compiler features

A Rust library for build scripts to automatically configure code based on compiler support. Code snippets are dynamically tested to see if the rustc will accept them, rather than hard-coding specific version support.

```rust,editable
{{#include ../../../crates/cats/development_tools_build_utils/examples/autocfg/autocfg.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1166)
</div>
