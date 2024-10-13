# Reduce compilation duration

Rust compile times{{hi:Compile times}} can be long.

{{#include reduce_compilation_duration.incl.md}}

## Measuring build times

[![cat-compilers][cat-compilers-badge]][cat-compilers]

```sh
time cargo build
```

```sh
cargo build --timings
```

## Optimization levels

[![cat-compilers][cat-compilers-badge]][cat-compilers]

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

[![cat-compilers][cat-compilers-badge]][cat-compilers]

```sh
cargo install cargo-add-dynamic
cargo add-dynamic polars --features csv-file,lazy,list,describe,rows,fmt,strings,temporal
cargo build
```

[Speeding up incremental Rust compilation with dylibs][blog-speeding-up-incremental-rust-compilation]⮳

## Incremental Compilation

[![cat-compilers][cat-compilers-badge]][cat-compilers]

From-scratch builds with incremental compilation{{hi:Incremental compilation}} enabled adds about 15–20% overhead compared to disabled. The initial build needs to write out more intermediate state in order for later incremental builds to take advantage of it. In a CI{{hi:CI}} situation, it would be extremely unusual for there to be a later incremental build within the same job. The jobs are not making changes to source code and rebuilding. However, workflows that cache the target directory across runs might be benefiting from incremental compilation.

## Reference

[8 Solutions for Troubleshooting Your Rust Build Times][blog-rust-build-times]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
<div class="hidden">
TODO:

## Salsa

- [Salsa](https://salsa-rs.github.io/salsa/overview.html)

## Rust compile duration

- [how-i-improved-my-rust-compile-times-by-seventy-five-percent](https://benw.is/posts/how-i-improved-my-rust-compile-times-by-seventy-five-percent)
- [rust-compilation-time](https://www.williballenthin.com/post/rust-compilation-time/)

</div>
