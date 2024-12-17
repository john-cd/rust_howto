# Reduce compilation duration

{{#include reduce_compilation_duration.incl.md}}

Rust compile times{{hi:Compile times}} can be long.

## Measure build times {#build-time}

[![cargo-website][c-cargo-website-badge]][c-cargo-website] [![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

```sh
time cargo build
```

```sh
cargo build --timings
```

## Optimize compilation levels {#optimization-levels}

[![cargo-website][c-cargo-website-badge]][c-cargo-website] [![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

In `config.toml`

```toml
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies, but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

## Use dynamic linking {#dynamic-linking}

[![cargo-add-dynamic][c-cargo_add_dynamic-badge]][c-cargo_add_dynamic] [![cargo-add-dynamic-crates.io][c-cargo_add_dynamic-crates.io-badge]][c-cargo_add_dynamic-crates.io] [![cargo-add-dynamic-github][c-cargo_add_dynamic-github-badge]][c-cargo_add_dynamic-github] [![cargo-add-dynamic-lib.rs][c-cargo_add_dynamic-lib.rs-badge]][c-cargo_add_dynamic-lib.rs]{{hi:cargo-add-dynamic}}{{hi:Cargo}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

```sh
cargo install cargo-add-dynamic
cargo add-dynamic polars --features csv-file,lazy,list,describe,rows,fmt,strings,temporal
cargo build
```

[Speeding up incremental Rust compilation with dylibs][blog-speeding-up-incremental-rust-compilation]⮳

## Compile incrementally {#incremental-compilation}

[![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

From-scratch builds with incremental compilation{{hi:Incremental compilation}} enabled adds about 15–20% overhead compared to disabled. The initial build needs to write out more intermediate state in order for later incremental builds to take advantage of it. In a CI{{hi:CI}} situation, it would be extremely unusual for there to be a later incremental build within the same job. The jobs are not making changes to source code and rebuilding. However, workflows that cache the target directory across runs might be benefiting from incremental compilation.

## Reference

- [Eight solutions for troubleshooting your Rust build times][blog-rust-build-times]⮳
- [How I improved my Rust compile times by seventy-five percent][blog-how-i-improved-my-rust-compile-times-by-seventy-five-percent]⮳
- [Rust compilation time][blog-rust-compilation-time]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[reduce_compilation_duration: review (P2)](https://github.com/john-cd/rust_howto/issues/245)
TODO P2 fix reference section
</div>
