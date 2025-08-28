# Additional Numeric Types

{{#include additional_numeric_types.incl.md}}

## Abstract over Different Number Types {#abstracting-over-number-types}

[![num-traits][c~num-traits~docs~badge]][c~num-traits~docs] [![num-traits~crates.io][c~num-traits~crates.io~badge]][c~num-traits~crates.io] [![num-traits~repo][c~num-traits~repo~badge]][c~num-traits~repo] [![num-traits~lib.rs][c~num-traits~lib.rs~badge]][c~num-traits~lib.rs]{{hi:num-traits}}{{hi:Numerics}}{{hi:Mathematics}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~science][cat~science~badge]][cat~science] [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Numeric [traits][p~traits] for generic [mathematics][p~mathematics]. [Traits][p~traits] like `Number`, `Add`, etc. that allow writing [functions][p~functions] that are generic over the specific numeric type:

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/num_traits.rs:example}}
```

## Use Big Integers {#big-integers}

### `num` {#num}

[![num][c~num~docs~badge]][c~num~docs] [![num~crates.io][c~num~crates.io~badge]][c~num~crates.io] [![num~repo][c~num~repo~badge]][c~num~repo] [![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]{{hi:num}}{{hi:Numerics}}{{hi:Bignum}}{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`num`][c~num~docs]↗{{hi:num}} provides a collection of numeric types and [traits][p~traits] for Rust, including bigint, complex, rational, range [iterators][p~iterators], generic integers, and more! Calculation for integers exceeding 128 bits are possible with [`num::BigInt`][c~num::BigInt~docs]↗{{hi:num::BigInt}}.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/big_integers.rs:example}}
```

### Use Big Integers with `num-bigint` {#num-bigint}

[![num-bigint][c~num-bigint~docs~badge]][c~num-bigint~docs] [![num-bigint~crates.io][c~num-bigint~crates.io~badge]][c~num-bigint~crates.io] [![num-bigint~repo][c~num-bigint~repo~badge]][c~num-bigint~repo] [![num-bigint~lib.rs][c~num-bigint~lib.rs~badge]][c~num-bigint~lib.rs]{{hi:num-bigint}}{{hi:Bignum}}{{hi:Mathematics}}{{hi:Numerics}} [![cat~science][cat~science~badge]][cat~science] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`num-bigint`][c~num-bigint~docs]↗{{hi:num-bigint}} is a big integer implementation for Rust. "It's not the fastest, but it's part of the trusted [`num`][c~num~docs]↗{{hi:num}} library."

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/num_bigint.rs:example}}
```

### Work with Arbitrary-precision Integers, Rationals, Floats, and Complex Numbers with `rug` {#rug}

[![rug][c~rug~docs~badge]][c~rug~docs] [![rug~crates.io][c~rug~crates.io~badge]][c~rug~crates.io] [![rug~repo][c~rug~repo~badge]][c~rug~repo] [![rug~lib.rs][c~rug~lib.rs~badge]][c~rug~lib.rs]{{hi:rug}}{{hi:Bignum}}{{hi:GMP}}{{hi:Mathematics}}{{hi:Numerics}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}

[`rug`][c~rug~docs]↗{{hi:rug}} offers arbitrary-precision integers, rational, floating-point and [complex numbers][p~complex-numbers] based on GMP, MPFR and MPC. LGPL licensed. Wrapper for GMP. Much faster than [`num-bigint`][c~num-bigint~docs]↗{{hi:num-bigint}}.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rug.rs:example}}
```

## Use Big Decimals {#big-decimal}

[![rust_decimal][c~rust_decimal~docs~badge]][c~rust_decimal~docs] [![rust_decimal~crates.io][c~rust_decimal~crates.io~badge]][c~rust_decimal~crates.io] [![rust_decimal~repo][c~rust_decimal~repo~badge]][c~rust_decimal~repo] [![rust_decimal~lib.rs][c~rust_decimal~lib.rs~badge]][c~rust_decimal~lib.rs]{{hi:rust_decimal}}{{hi:Decimal}}{{hi:Financial}}{{hi:Number}}{{hi:Precision}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science]

`big-decimal` is a decimal number implementation written in pure Rust suitable for financial and fixed-precision calculations. The binary representation consists of a 96 bit integer number, a scaling factor used to specify the decimal fraction and a 1 bit sign.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rust_decimal.rs:example}}
```

## Sort Floats {#sortable-floats}

[![ordered-float][c~ordered-float~docs~badge]][c~ordered-float~docs] [![ordered-float~crates.io][c~ordered-float~crates.io~badge]][c~ordered-float~crates.io] [![ordered-float~repo][c~ordered-float~repo~badge]][c~ordered-float~repo] [![ordered-float~lib.rs][c~ordered-float~lib.rs~badge]][c~ordered-float~lib.rs]{{hi:ordered-float}}{{hi:f32}}{{hi:f64}}{{hi:Sorting}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}} [![cat~science][cat~science~badge]][cat~science]

[`ordered-float`][c~ordered-float~docs]↗{{hi:ordered-float}} provides wrappers for total ordering on floats. Float types that don't allow `NaN` and are therefore orderable. You can also use the `total_cmp` method from the standard library like `sort_by(|a, b| a.total_cmp(&b))`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/ordered_float.rs:example}}
```

## `typenum` {#typenum}

[![typenum][c~typenum~docs~badge]][c~typenum~docs] [![typenum~crates.io][c~typenum~crates.io~badge]][c~typenum~crates.io] [![typenum~repo][c~typenum~repo~badge]][c~typenum~repo] [![typenum~lib.rs][c~typenum~lib.rs~badge]][c~typenum~lib.rs]{{hi:typenum}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Typenum is a Rust library for type-level numbers evaluated at compile time. It currently supports bits, unsigned integers, and signed integers. It also provides a type-level array of type-level numbers, but its implementation is incomplete.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/typenum.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[additional_numeric_types: write](https://github.com/john-cd/rust_howto/issues/407)
</div>
