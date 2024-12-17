# Others

{{#include other.incl.md}}

## Search for Rust APIs {#roogle}

[`Roogle`][c-roogle-website]{{hi:roogle}}⮳ [![roogle-github][c-roogle-github-badge]][c-roogle-github] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

Roogle is a Rust API search engine, which allows you to search functions by names and type signatures. The query can be one of the following types:

|||
|---|---|
| fn f(type) -> type | fn f(&mut HashMap<K, V>, K, V) -> Option<V> |
| fn (type) -> type | fn (&char) -> bool |
| fn(type) -> type | fn(Option<Option<T>>) -> Option<T> |
| (type) -> type | (&mut Vec<T>, value: T) |

## Deploy your Rust code on `shuttle.rs` {#shuttle-rs}

[shuttle.rs][shuttle-rs-website]{{hi:shuttle.rs}}⮳ [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

```rust,editable
{{#include ../../../../deps/tests/categories/development_tools/other/shuttle.rs:example}}
```

## Minimize Rust binary sizes {#binary-minimizer}

[How to minimize Rust binary size][min-sized-rust-github]⮳

By default, Rust optimizes for execution speed, compilation speed, and ease of debugging. This approach is suitable for most applications, as it balances performance and developer productivity. However, in specific scenarios where binary size is a critical concern (e.g., embedded systems or deployment to constrained environments), Rust offers mechanisms to optimize for smaller binary sizes.

```sh
cargo build --release
```

## Generate Rust code {#code-generators}

[Top artificial intelligence tools that can generate code to help programmers][blog-ai-tools-that-can-generate-code]⮳

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[other: expand; revise refs.incl.md (P1)](https://github.com/john-cd/rust_howto/issues/305)
</div>
