# Additional numeric types

{{#include additional_numeric_types.incl.md}}

## Abstracting over different number types {#abstracting-over-number-types}

[![num-traits][c-num_traits-badge]][c-num_traits]{{hi:num-traits}}
[![num-traits-crates.io][c-num_traits-crates.io-badge]][c-num_traits-crates.io]
[![num-traits-github][c-num_traits-github-badge]][c-num_traits-github]
[![num-traits-lib.rs][c-num_traits-lib.rs-badge]][c-num_traits-lib.rs]
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}
[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Traits like Number, Add, etc that allow you write functions that are generic over the specific numeric type

## Big Integers {#big-integers}

### `num`

[![num][c-num-badge]][c-num]{{hi:num}}
[![num-crates.io][c-num-crates.io-badge]][c-num-crates.io]
[![num-github][c-num-github-badge]][c-num-github]
[![num-lib.rs][c-num-lib.rs-badge]][c-num-lib.rs]
[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Calculation for integers exceeding 128 bits are possible with [`num::BigInt`][c-num::BigInt]{{hi:num::BigInt}}⮳.

```rust
{{#include ../../../deps/tests/cats/mathematics/big_integers.rs:example}}
```

### `num-bigint`

[![num-bigint][c-num_bigint-badge]][c-num_bigint]{{hi:num-bigint}}
[![num-bigint-crates.io][c-num_bigint-crates.io-badge]][c-num_bigint-crates.io]
[![num-bigint-github][c-num_bigint-github-badge]][c-num_bigint-github]
[![num-bigint-lib.rs][c-num_bigint-lib.rs-badge]][c-num_bigint-lib.rs]
[![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}}
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}

It's not the fastest, but it's part of the trusted num library.

### `rug`

[![rug][c-rug-badge]][c-rug]{{hi:rug}}
[![rug-crates.io][c-rug-crates.io-badge]][c-rug-crates.io]
[![rug-github][c-rug-github-badge]][c-rug-github]
[![rug-lib.rs][c-rug-lib.rs-badge]][c-rug-lib.rs]
[![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}
[![cat-mathematics][cat-mathematics-badge]][cat-mathematics]{{hi:Mathematics}}

LGPL licensed. Wrapper for GMP. Much faster than num-bigint

## Big decimal {#big-decimal}

[![rust_decimal][c-rust_decimal-badge]][c-rust_decimal]{{hi:rust_decimal}}
[![rust_decimal-crates.io][c-rust_decimal-crates.io-badge]][c-rust_decimal-crates.io]
[![rust_decimal-github][c-rust_decimal-github-badge]][c-rust_decimal-github]
[![rust_decimal-lib.rs][c-rust_decimal-lib.rs-badge]][c-rust_decimal-lib.rs]
[![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}
[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}
[![cat-mathematics][cat-mathematics-badge]][cat-mathematics]{{hi:Mathematics}}

The binary representation consists of a 96 bit integer number, a scaling factor used to specify the decimal fraction and a 1 bit sign.

## Sortable Floats {#sortable-floats}

[![ordered-float][c-ordered_float-badge]][c-ordered_float]{{hi:ordered-float}}
[![ordered-float-crates.io][c-ordered_float-crates.io-badge]][c-ordered_float-crates.io]
[![ordered-float-github][c-ordered_float-github-badge]][c-ordered_float-github]
[![ordered-float-lib.rs][c-ordered_float-lib.rs-badge]][c-ordered_float-lib.rs]
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-science][cat-science-badge]][cat-science]{{hi:Science}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

Float types that don't allow NaN and are therefore orderable. You can also use the total_cmp method from the standard library like .sort_by(|a, b| a.total_cmp(&b)).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
