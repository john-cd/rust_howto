# Others

{{#include other.incl.md}}

## Search for Rust APIs {#roogle}

[`Roogle`][c~roogle~website]{{hi:roogle}}⮳ [![roogle~github][c~roogle~github~badge]][c~roogle~github] [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

Roogle is a Rust API [search][p~search] engine, which allows you to [search][p~search] functions by names and type signatures. The query can be one of the following types:

|||
|---|---|
| fn f(type) -> type | fn f(&mut HashMap<K, V>, K, V) -> Option<V> |
| fn (type) -> type | fn (&char) -> bool |
| fn(type) -> type | fn(Option<Option<T>>) -> [Option][p~option]<T> |
| (type) -> type | (&mut Vec<T>, value: T) |

## Minimize Rust Binary Sizes {#binary-minimizer}

[How to minimize Rust binary size][min-sized-rust~github]⮳.

By default, Rust optimizes for execution speed, compilation speed, and ease of debugging. This approach is suitable for most applications, as it balances [performance][p~performance] and developer productivity. However, in specific scenarios where binary size is a critical concern (e.g., embedded systems or deployment to constrained environments), Rust offers mechanisms to optimize for smaller binary sizes.

```sh
cargo build --release
```

## Generate Rust Code {#code-generators}

[Top artificial intelligence tools that can generate code to help programmers][blog~ai-tools-that-can-generate-code]⮳.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[other: expand; revise refs.incl.md NOW](https://github.com/john-cd/rust_howto/issues/305)

- [Gerust: Rust backend project generator](https://gerust.rs/)

</div>
