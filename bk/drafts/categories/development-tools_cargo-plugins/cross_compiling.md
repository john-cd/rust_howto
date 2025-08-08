# Cross-compilation

{{#include cross_compiling.incl.md}}

| Topic | Rust Crates |
|---|---|
| Cross-Compilation Tools | [`cross`][c~cross~docs]↗{{hi:cross}} simplifies cross-compilation by using Docker containers. This is the most common and recommended approach. |
| Target Management (within [Cargo][p~cargo]) | `cargo build --target <target_triple>`: [Cargo][p~cargo] itself supports cross-compilation by specifying the target architecture. [`cross`][c~cross~docs]↗{{hi:cross}} just makes this easier. You'll still need to configure targets in your project. |
| Other Cross-Compilation Helpers (less common or more specialized) | Often, cross-compilation involves dealing with C/C++ dependencies. The [`cc`][c~cc~docs]↗{{hi:cc}} crate, used in build scripts, can be helpful. |

## Cross-compile Using `zig` as the Linker {#zig}

[![cargo-zigbuild][c~cargo-zigbuild~docs~badge]][c~cargo-zigbuild~docs] [![cargo-zigbuild~crates.io][c~cargo-zigbuild~crates.io~badge]][c~cargo-zigbuild~crates.io] [![cargo-zigbuild~github][c~cargo-zigbuild~github~badge]][c~cargo-zigbuild~github] [![cargo-zigbuild~lib.rs][c~cargo-zigbuild~lib.rs~badge]][c~cargo-zigbuild~lib.rs]{{hi:cargo-zigbuild}}{{hi:Cargo}}{{hi:Zig}}

Compile [Cargo][p~cargo] project with zig as linker.

```sh
cargo install --locked cargo-zigbuild
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/923)
</div>
