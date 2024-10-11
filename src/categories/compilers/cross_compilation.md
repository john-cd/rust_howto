# Cross-compilation

{{#include cross_compilation.incl.md}}

## Cross

[![cross-github][c-cross-github-badge]][c-cross-github]  [![cat-compilers][cat-compilers-badge]][cat-compilers]

{{hi:cross}}[`cross`][c-cross-github]⮳ builds your Rust project for different target operating systems and architectures. It requires {{hi:rustup}}[`rustup`][rustup-website]⮳ and {{hi:Docker}}[`Docker`][docker-website]⮳ or {{hi:Podman}}[`Podman`][podman-website]⮳.

```sh
cargo install cross --git https://github.com/cross-rs/cross

# Optionally, if you have cargo-binstall, you can install via pre-built binary
cargo binstall cross
```

{{hi:cross}}[`cross`][c-cross]⮳ has the exact same CLI as {{hi:cargo}}[`cargo`][c-cargo]⮳ but relies on {{hi:docker}}[`docker`][docker-website]⮳ or {{hi:podman}}[`podman`][podman-website]⮳.

```sh
cross build --target aarch64-unknown-linux-gnu

cross test --target mips64-unknown-linux-gnuabi64

cross run --target aarch64-unknown-linux-gnu
```

[![cross-wiki][c-cross-wiki-badge]][c-cross-wiki]

Examples: [![cross-toml-example][c-cross-toml-example-badge]][c-cross-toml-example]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
