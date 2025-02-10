# Cross-compilation

{{#include cross_compilation.incl.md}}

## Cross-compile for multiple target OSes and architectures {#cross-compilation}

[![cross][c-cross-badge]][c-cross]{{hi:cross}} [![cross-github][c-cross-github-badge]][c-cross-github] [![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

[`cross`][c-cross-github]{{hi:cross}}⮳ builds your Rust project for different target operating systems and architectures. It requires [`rustup`][rustup-website]{{hi:rustup}}⮳ and [`Docker`][docker-website]{{hi:docker}}⮳ or [`Podman`][podman-website]{{hi:podman}}⮳.

```sh
cargo install cross --git https://github.com/cross-rs/cross

# Optionally, if you have cargo-binstall, you can install via pre-built binary
cargo binstall cross
```

[`cross`][c-cross]{{hi:cross}}⮳ has the exact same CLI as [`cargo`][c-cargo]{{hi:cargo}}⮳ but relies on [`docker`][docker-website]{{hi:docker}}⮳ or [`podman`][podman-website]{{hi:podman}}⮳.

```sh
cross build --target aarch64-unknown-linux-gnu

cross test --target mips64-unknown-linux-gnuabi64

cross run --target aarch64-unknown-linux-gnu
```

[![cross-wiki][c-cross-wiki-badge]][c-cross-wiki]

Examples: [![cross-toml-example][c-cross-toml-example-badge]][c-cross-toml-example]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cross_compilation: expand (P2)](https://github.com/john-cd/rust_howto/issues/240)
</div>
