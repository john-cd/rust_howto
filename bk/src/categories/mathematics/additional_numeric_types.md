# Additional Numeric Types

{{#include additional_numeric_types.incl.md}}

## Abstract over Different Number Types {#abstracting-over-number-types}

[![num-traits][c~num_traits~docs~badge]][c~num_traits~docs] [![num-traits~crates.io][c~num_traits~crates.io~badge]][c~num_traits~crates.io] [![num-traits~github][c~num_traits~github~badge]][c~num_traits~github] [![num-traits~lib.rs][c~num_traits~lib.rs~badge]][c~num_traits~lib.rs]{{hi:num-traits}}{{hi:Numerics}}{{hi:Mathematics}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Numeric [traits][p~traits] for generic [mathematics][p~mathematics]. [Traits][p~traits] like Number, Add, etc that allow you write [functions][p~functions] that are generic over the specific numeric type

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/num_traits.rs:example}}
```

## Use Big Integers {#big-integers}

### `num` {#skip1}

[![num][c~num~docs~badge]][c~num~docs] [![num~crates.io][c~num~crates.io~badge]][c~num~crates.io] [![num~github][c~num~github~badge]][c~num~github] [![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]{{hi:num}}{{hi:Numerics}}{{hi:Bignum}}{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`num`][c~num~docs]⮳{{hi:num}} provides a collection of numeric types and [traits][p~traits] for Rust, including bigint, complex, rational, range [iterators][p~iterators], generic integers, and more! Calculation for integers exceeding 128 bits are possible with [`num::BigInt`][c~num::BigInt~docs]{{hi:num::BigInt}}⮳.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/big_integers.rs:example}}
```

### `num-bigint` {#skip2}

[![num-bigint][c~num_bigint~docs~badge]][c~num_bigint~docs] [![num-bigint~crates.io][c~num_bigint~crates.io~badge]][c~num_bigint~crates.io] [![num-bigint~github][c~num_bigint~github~badge]][c~num_bigint~github] [![num-bigint~lib.rs][c~num_bigint~lib.rs~badge]][c~num_bigint~lib.rs]{{hi:num-bigint}}{{hi:Bignum}}{{hi:Mathematics}}{{hi:Numerics}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`num-bigint`][c~num_bigint~docs]⮳{{hi:num-bigint}} is a big integer implementation for Rust. "It's not the fastest, but it's part of the trusted [`num`][c~num~docs]⮳{{hi:num}} library."

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/num_bigint.rs:example}}
```

### `rug` {#skip3}

[![rug][c~rug~docs~badge]][c~rug~docs] [![rug~crates.io][c~rug~crates.io~badge]][c~rug~crates.io] [![rug~github][c~rug~github~badge]][c~rug~github] [![rug~lib.rs][c~rug~lib.rs~badge]][c~rug~lib.rs]{{hi:rug}}{{hi:Bignum}}{{hi:Gmp}}{{hi:Math}}{{hi:Numerics}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}

[`rug`][c~rug~docs]⮳{{hi:rug}} offers arbitrary-precision integers, rational, floating-point and [complex numbers][p~complex-numbers] based on GMP, MPFR and MPC. LGPL licensed. Wrapper for GMP. Much faster than [`num-bigint`][c~num_bigint~docs]⮳{{hi:num-bigint}}.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rug.rs:example}}
```

## Use Big Decimals {#big-decimal}

[![rust_decimal][c~rust_decimal~docs~badge]][c~rust_decimal~docs] [![rust_decimal~crates.io][c~rust_decimal~crates.io~badge]][c~rust_decimal~crates.io] [![rust_decimal~github][c~rust_decimal~github~badge]][c~rust_decimal~github] [![rust_decimal~lib.rs][c~rust_decimal~lib.rs~badge]][c~rust_decimal~lib.rs]{{hi:rust_decimal}}{{hi:Decimal}}{{hi:Financial}}{{hi:Fixed}}{{hi:Number}}{{hi:Precision}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

`big-decimal` is a decimal number implementation written in pure Rust suitable for financial and fixed-precision calculations. The binary representation consists of a 96 bit integer number, a scaling factor used to specify the decimal fraction and a 1 bit sign.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rust_decimal.rs:example}}
```

## Sort Floats {#sortable-floats}

[![ordered-float][c~ordered_float~docs~badge]][c~ordered_float~docs] [![ordered-float~crates.io][c~ordered_float~crates.io~badge]][c~ordered_float~crates.io] [![ordered-float~github][c~ordered_float~github~badge]][c~ordered_float~github] [![ordered-float~lib.rs][c~ordered_float~lib.rs~badge]][c~ordered_float~lib.rs]{{hi:ordered-float}}{{hi:F32}}{{hi:F64}}{{hi:No_std}}{{hi:Ord}}{{hi:Sort}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}} [![cat~science][cat~science~badge]][cat~science]{{hi:Science}}

[`ordered-float`][c~ordered_float~docs]⮳{{hi:ordered-float}} provides wrappers for total ordering on floats. Float types that don't allow `NaN` and are therefore orderable. You can also use the `total_cmp` method from the standard library like `sort_by(|a, b| a.total_cmp(&b))`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/ordered_float.rs:example}}
```

## `typenum` {#typenum}

[![typenum][c~typenum~docs~badge]][c~typenum~docs] [![typenum~crates.io][c~typenum~crates.io~badge]][c~typenum~crates.io] [![typenum~github][c~typenum~github~badge]][c~typenum~github] [![typenum~lib.rs][c~typenum~lib.rs~badge]][c~typenum~lib.rs]{{hi:typenum}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Typenum is a Rust library for type-level numbers evaluated at compile time. It currently supports bits, unsigned integers, and signed integers. It also provides a type-level array of type-level numbers, but its implementation is incomplete.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/typenum.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[additional_numeric_types: write](https://github.com/john-cd/rust_howto/issues/407)
</div>
