# Crate Registries

{{#include crate_registries.incl.md}}

In Rust, a library or executable program is called a crate{{hi:Crate}}. Crates are compiled using the Rust compiler, [![`rustc`][book~rustc~badge]][book~rustc] [`rustc`][book~rustc]{{hi:rustc}}⮳.

## Crate Registries {#crate-registries}

[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]

The Rust community's crate registry: [`crates.io`][crates.io~website]{{hi:crates.io}}⮳.

Alternative to [`crates.io`][crates.io~website]⮳{{hi:crates.io}}: [`lib.rs`][lib-rs]{{hi:lib.rs}}⮳.

## Access crates.io APIs with `crates_io` {#crates_io}

[![crates-io][c~crates_io~docs~badge]][c~crates_io~docs] [![crates-io~crates.io][c~crates_io~crates.io~badge]][c~crates_io~crates.io] [![crates-io~github][c~crates_io~github~badge]][c~crates_io~github] [![crates-io~lib.rs][c~crates_io~lib.rs~badge]][c~crates_io~lib.rs]{{hi:crates-io}}

Helpers for interacting with crates.io.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/crate_registries/crates_io.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[write; expand NOW](https://github.com/john-cd/rust_howto/issues/294)
</div>
