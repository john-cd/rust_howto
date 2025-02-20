# Cross-compilation

{{#include cross_compiling.incl.md}}

## Cross-compile using `zig` as the linker {#zig}

[![cargo-zigbuild][c-cargo_zigbuild-badge]][c-cargo_zigbuild] [![cargo-zigbuild-crates.io][c-cargo_zigbuild-crates.io-badge]][c-cargo_zigbuild-crates.io] [![cargo-zigbuild-github][c-cargo_zigbuild-github-badge]][c-cargo_zigbuild-github] [![cargo-zigbuild-lib.rs][c-cargo_zigbuild-lib.rs-badge]][c-cargo_zigbuild-lib.rs]{{hi:cargo-zigbuild}}{{hi:Cargo}}{{hi:Zig}}

Compile [Cargo][p-cargo] project with zig as linker.

```sh
cargo install --locked cargo-zigbuild
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P1 review](https://github.com/john-cd/rust_howto/issues/923)

Cross-Compilation Tool:

cross: Simplifies cross-compilation by using Docker containers. This is the most common and recommended approach.
Target Management (within Cargo):

cargo build --target <target_triple>: Cargo itself supports cross-compilation by specifying the target architecture. cross just makes this easier. You'll still need to configure targets in your project.
Other Cross-Compilation Helpers: (Less common or more specialized)

(Often, cross-compilation involves dealing with C/C++ dependencies. The cc crate, used in build scripts, can be helpful)

</div>
