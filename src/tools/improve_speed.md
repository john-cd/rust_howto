# Reduce compilation duration

Rust compile times can be long.

## Measuring build times

```sh
time cargo build
```

```sh
cargo build --timings
```

## Optimization levels

In `config.toml`

```toml
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies, but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

## Dynamic linking

```sh
cargo install cargo-add-dynamic
cargo add-dynamic polars --features csv-file,lazy,list,describe,rows,fmt,strings,temporal
cargo build
```

[Speeding up incremental Rust compilation with dylibs]( https://robert.kra.hn/posts/2022-09-09-speeding-up-incremental-rust-compilation-with-dylibs/ )

## Incremental Compilation

From-scratch builds with incremental compilation enabled adds about 15–20% overhead compared to disabled. The initial build needs to write out more intermediate state in order for later incremental builds to take advantage of it. In a CI situation, it would be extremely unusual for there to be a later incremental build within the same job. The jobs are not making changes to source code and rebuilding. However, workflows that cache the target directory across runs might be benefiting from incremental compilation.

## Reference

[8 Solutions for Troubleshooting Your Rust Build Times]( https://jondot.medium.com/8-steps-for-troubleshooting-your-rust-build-times-2ffc965fd13e )

{{#include ../links.md}}
