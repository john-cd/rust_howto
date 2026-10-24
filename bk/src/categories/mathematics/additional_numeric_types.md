<!-- TODO Reviewed ??-??-2026 -->
# Additional Numeric Types

{{#include additional_numeric_types.incl.md}}

## Abstract over Different Number Types {#abstracting-over-number-types}

[![num-traits][c~num-traits~docs~badge]][c~num-traits~docs] [![num-traits~crates.io][c~num-traits~crates.io~badge]][c~num-traits~crates.io] [![num-traits~repo][c~num-traits~repo~badge]][c~num-traits~repo] [![num-traits~lib.rs][c~num-traits~lib.rs~badge]][c~num-traits~lib.rs]{{hi:num-traits}}{{hi:Numerics}}{{hi:Mathematics}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~science][cat~science~badge]][cat~science] [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`num-traits`][c~num-traits~docs] provides traits such as `Zero`, `One`, and `NumCast` for generic [mathematics][p~mathematics]. Generic code can use these traits to create numeric identities and convert values without depending on one concrete type; the example demonstrates them with `i32`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/num_traits.rs:example}}
```

## Use Big Integers {#big-integers}

### `num` {#num}

[![num][c~num~docs~badge]][c~num~docs] [![num~crates.io][c~num~crates.io~badge]][c~num~crates.io] [![num~repo][c~num~repo~badge]][c~num~repo] [![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]{{hi:num}}{{hi:Numerics}}{{hi:Bignum}}{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`num`][c~num~docs] is a meta-crate that re-exports numeric types and traits, including big integers, rationals, complex numbers, and generic numeric traits. Its `BigInt` type supports calculations beyond the fixed range of built-in integer types.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/big_integers.rs:example}}
```

### Use Big Integers with `num-bigint` {#num-bigint}

[![num-bigint][c~num-bigint~docs~badge]][c~num-bigint~docs] [![num-bigint~crates.io][c~num-bigint~crates.io~badge]][c~num-bigint~crates.io] [![num-bigint~repo][c~num-bigint~repo~badge]][c~num-bigint~repo] [![num-bigint~lib.rs][c~num-bigint~lib.rs~badge]][c~num-bigint~lib.rs]{{hi:num-bigint}}{{hi:Bignum}}{{hi:Mathematics}}{{hi:Numerics}} [![cat~science][cat~science~badge]][cat~science] [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`num-bigint`][c~num-bigint~docs] provides arbitrary-precision signed and unsigned integers. Use it when an integer may exceed the range of `i128` or `u128`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/num_bigint.rs:example}}
```

### Work with Arbitrary-precision Integers, Rationals, Floats, and Complex Numbers with `rug` {#rug}

[![rug][c~rug~docs~badge]][c~rug~docs] [![rug~crates.io][c~rug~crates.io~badge]][c~rug~crates.io] [![rug~repo][c~rug~repo~badge]][c~rug~repo] [![rug~lib.rs][c~rug~lib.rs~badge]][c~rug~lib.rs]{{hi:rug}}{{hi:Bignum}}{{hi:GMP}}{{hi:Mathematics}}{{hi:Numerics}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}}

[`rug`][c~rug~docs] provides arbitrary-precision integers, rationals, floating-point values, and [complex numbers][p~complex-numbers] through GMP, MPFR, and MPC. Choose it when those arbitrary-precision types or their numerical behavior fit your application and its native-library build requirements.

```rust,editable,noplayground
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rug.rs:example}}
```

## Use Fixed-Precision Decimal Numbers {#big-decimal}

[![rust_decimal][c~rust_decimal~docs~badge]][c~rust_decimal~docs] [![rust_decimal~crates.io][c~rust_decimal~crates.io~badge]][c~rust_decimal~crates.io] [![rust_decimal~repo][c~rust_decimal~repo~badge]][c~rust_decimal~repo] [![rust_decimal~lib.rs][c~rust_decimal~lib.rs~badge]][c~rust_decimal~lib.rs]{{hi:rust_decimal}}{{hi:Decimal}}{{hi:Financial}}{{hi:Number}}{{hi:Precision}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~mathematics][cat~mathematics~badge]][cat~mathematics]{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science]

[`rust_decimal`][c~rust_decimal~docs] stores a bounded 96-bit integer mantissa with a decimal scale and sign, making it useful for financial values and other base-10 calculations. Values within its range can represent decimal fractions exactly; division or values beyond that range still require an explicit rounding and overflow policy.

```rust,editable,noplayground
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rust_decimal.rs:example}}
```

## Sort Floats {#sortable-floats}

[![ordered-float][c~ordered-float~docs~badge]][c~ordered-float~docs] [![ordered-float~crates.io][c~ordered-float~crates.io~badge]][c~ordered-float~crates.io] [![ordered-float~repo][c~ordered-float~repo~badge]][c~ordered-float~repo] [![ordered-float~lib.rs][c~ordered-float~lib.rs~badge]][c~ordered-float~lib.rs]{{hi:ordered-float}}{{hi:f32}}{{hi:f64}}{{hi:Sorting}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}} [![cat~science][cat~science~badge]][cat~science]

[`ordered-float`][c~ordered-float~docs] provides wrappers that implement total ordering for floating-point values so they can be used in ordered collections. `OrderedFloat` allows NaN and orders it after ordinary values; use `NotNan` when construction should reject NaN. For sorting alone, the standard library also provides `total_cmp`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/ordered_float.rs:example}}
```

## `typenum` {#typenum}

[![typenum][c~typenum~docs~badge]][c~typenum~docs] [![typenum~crates.io][c~typenum~crates.io~badge]][c~typenum~crates.io] [![typenum~repo][c~typenum~repo~badge]][c~typenum~repo] [![typenum~lib.rs][c~typenum~lib.rs~badge]][c~typenum~lib.rs]{{hi:typenum}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`typenum`][c~typenum~docs] provides type-level numbers and operators evaluated at compile time, including signed and unsigned integers. The example computes a capacity at the type level and uses it in a fixed-size buffer's type.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/typenum.rs:example}}
```

## Rational Numbers {#rational-numbers}

[![num-rational][c~num-rational~docs~badge]][c~num-rational~docs] [![num-rational~crates.io][c~num-rational~crates.io~badge]][c~num-rational~crates.io]{{hi:num-rational}}{{hi:Rational numbers}}{{hi:Fractions}}{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science]

[`num-rational`][c~num-rational~docs] represents a rational value as a numerator and denominator, so fractions such as one third remain exact during rational arithmetic. The chosen integer type bounds the values that can be represented; use `BigRational` when those bounds are insufficient.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rational_numbers.rs:example}}
```

## Fixed-Point Numbers {#fixed-point}

[![fixed][c~fixed~docs~badge]][c~fixed~docs] [![fixed~crates.io][c~fixed~crates.io~badge]][c~fixed~crates.io]{{hi:fixed}}{{hi:Fixed point}}{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science]

[`fixed`][c~fixed~docs] provides fixed-point types whose integer and fractional bit counts are chosen in the type. Use it when a fixed binary scale is appropriate, and select a type whose range and fractional precision fit the calculation.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/fixed_point.rs:example}}
```

## Half-Precision Floats {#half-precision}

[![half][c~half~docs~badge]][c~half~docs] [![half~crates.io][c~half~crates.io~badge]][c~half~crates.io]{{hi:half}}{{hi:f16}}{{hi:Binary16}}{{hi:Mathematics}} [![cat~science][cat~science~badge]][cat~science]

[`half`][c~half~docs] provides the IEEE 754 binary16 `f16` type, which can reduce storage when its smaller range and precision are acceptable. Converting from `f32` can lose information, and arithmetic may be emulated on targets without native half-precision support.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/half_precision.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[complex_numbers | Complex Numbers]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[accuracy review: mathematics](https://github.com/john-cd/rust_howto/issues/1894)
</div>
