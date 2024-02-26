# Cross-compilation

{{#include cross_compilation.incl.md}}

## Cross

[![cross-github][cross-github-badge]][cross-github]  [![cat-compilers][cat-compilers-badge]][cat-compilers]

[cross][cross-github]⮳ builds your Rust project for different target operating systems and architectures. It requires `rustup` and `Docker` or `Podman`.

```sh
cargo install cross --git https://github.com/cross-rs/cross

# Optionally, if you have cargo-binstall, you can install via pre-built binary
cargo binstall cross
```

`cross` has the exact same CLI as Cargo but relies on Docker or Podman.

```sh
cross build --target aarch64-unknown-linux-gnu

cross test --target mips64-unknown-linux-gnuabi64

cross run --target aarch64-unknown-linux-gnu
```

[![cross-wiki][cross-wiki-badge]][cross-wiki]

Examples: [![cross-toml-example][cross-toml-example-badge]][cross-toml-example]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
