# Reduce compilation duration

{{#include reduce_compilation_duration.incl.md}}

Rust compile times{{hi:Compile times}} can be long.

## Measuring build times {#build-time}

[![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

```sh
time cargo build
```

```sh
cargo build --timings
```

## Optimization levels {#optimization-levels}

[![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

In `config.toml`

```toml
# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies, but not for our code:
[profile.dev.package."*"]
opt-level = 3
```

## Dynamic linking {#dynamic-linking}

[![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

```sh
cargo install cargo-add-dynamic
cargo add-dynamic polars --features csv-file,lazy,list,describe,rows,fmt,strings,temporal
cargo build
```

[Speeding up incremental Rust compilation with dylibs][blog-speeding-up-incremental-rust-compilation]⮳

## Incremental Compilation {#incremental-compilation}

[![cat-compilers][cat-compilers-badge]][cat-compilers]{{hi:Compilers}}

From-scratch builds with incremental compilation{{hi:Incremental compilation}} enabled adds about 15–20% overhead compared to disabled. The initial build needs to write out more intermediate state in order for later incremental builds to take advantage of it. In a CI{{hi:CI}} situation, it would be extremely unusual for there to be a later incremental build within the same job. The jobs are not making changes to source code and rebuilding. However, workflows that cache the target directory across runs might be benefiting from incremental compilation.

### Incremental Computation {#incremental-computation}

### Salsa

[![salsa-github][c-salsa-github-badge]][c-salsa-github]  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[Salsa][c-salsa-website]{{hi:salsa}}⮳

[Salsa (GitHub)][c-salsa-github]⮳ is a framework for on-demand, incremental computation.

Salsa is a Rust framework for writing incremental, on-demand programs -- these are programs that want to adapt to changes in their inputs, continuously producing a new output that is up-to-date.

## Reference

- [8 Solutions for Troubleshooting Your Rust Build Times][blog-rust-build-times]⮳
- [how-i-improved-my-rust-compile-times-by-seventy-five-percent][blog-how-i-improved-my-rust-compile-times-by-seventy-five-percent]⮳
- [rust-compilation-time][blog-rust-compilation-time]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: fix reference section
</div>
