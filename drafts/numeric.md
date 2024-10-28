# Numeric types

## Abstracting over different number types

[![num-traits][c-num_traits-badge]][c-num_traits]{{hi:num-traits}}
[![num-traits-crates.io][c-num_traits-crates.io-badge]][c-num_traits-crates.io]
[![num-traits-github][c-num_traits-github-badge]][c-num_traits-github]
[![num-traits-lib.rs][c-num_traits-lib.rs-badge]][c-num_traits-lib.rs]

Traits like Number, Add, etc that allow you write functions that are generic over the specific numeric type

## Big Integers

### num-bigint

[![num-bigint][c-num_bigint-badge]][c-num_bigint]{{hi:num-bigint}}
[![num-bigint-crates.io][c-num_bigint-crates.io-badge]][c-num_bigint-crates.io]
[![num-bigint-github][c-num_bigint-github-badge]][c-num_bigint-github]
[![num-bigint-lib.rs][c-num_bigint-lib.rs-badge]][c-num_bigint-lib.rs]

It's not the fastest, but it's part of the trusted num library.

### rug

[![rug][c-rug-badge]][c-rug]{{hi:rug}}
[![rug-crates.io][c-rug-crates.io-badge]][c-rug-crates.io]
[![rug-github][c-rug-github-badge]][c-rug-github]
[![rug-lib.rs][c-rug-lib.rs-badge]][c-rug-lib.rs]

LGPL licensed. Wrapper for GMP. Much faster than num-bigint

## Big decimal

[![rust_decimal][c-rust_decimal-badge]][c-rust_decimal]{{hi:rust_decimal}}
[![rust_decimal-crates.io][c-rust_decimal-crates.io-badge]][c-rust_decimal-crates.io]
[![rust_decimal-github][c-rust_decimal-github-badge]][c-rust_decimal-github]
[![rust_decimal-lib.rs][c-rust_decimal-lib.rs-badge]][c-rust_decimal-lib.rs]

The binary representation consists of a 96 bit integer number, a scaling factor used to specify the decimal fraction and a 1 bit sign.

## Sortable Floats

[![ordered-float][c-ordered_float-badge]][c-ordered_float]{{hi:ordered-float}}
[![ordered-float-crates.io][c-ordered_float-crates.io-badge]][c-ordered_float-crates.io]
[![ordered-float-github][c-ordered_float-github-badge]][c-ordered_float-github]
[![ordered-float-lib.rs][c-ordered_float-lib.rs-badge]][c-ordered_float-lib.rs]

Float types that don't allow NaN and are therefore orderable. You can also use the total_cmp method from the standard library like .sort_by(|a, b| a.total_cmp(&b)).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
