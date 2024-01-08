# Cross-compilation

## Cross

[cross](https://github.com/cross-rs/cross)⮳ builds your Rust project for different target operating systems and architectures.
It requires `rustup` and `Docker` or `Podman`.

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

[cross Wiki]( https://github.com/cross-rs/cross/wiki/Getting-Started )⮳

[Example Cross.toml file]( https://github.com/cross-rs/wiki_assets/blob/main/Configuration/Cross.toml )⮳

{{#include ../links.md}}
