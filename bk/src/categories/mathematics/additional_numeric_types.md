<!-- TODO Reviewed ??-??-2026 -->

# Additional Numeric Types

{{#include additional_numeric_types.incl.md}}

## Abstract over Different Number Types {#abstracting-over-number-types}

{{#badges num-traits algorithms=Algorithms science= no-std="No standard library"}}{{hi:Numerics}}{{hi:Mathematics}}

[`num-traits`][c~num-traits~docs] provides traits such as `Zero`, `One`, and `NumCast` for generic [mathematics][p~mathematics]. Generic code can use these traits to create numeric identities and convert values without depending on one concrete type; the example demonstrates them with `i32`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/num_traits.rs:example}}
```

## Use Big Integers {#big-integers}

### `num` {#num}

{{#badges num science= algorithms=Algorithms no-std="No standard library" data-structures="Data structures"}}{{hi:Numerics}}{{hi:Bignum}}{{hi:Mathematics}}

[`num`][c~num~docs] is a meta-crate that re-exports numeric types and traits, including big integers, rationals, complex numbers, and generic numeric traits. Its `BigInt` type supports calculations beyond the fixed range of built-in integer types.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/big_integers.rs:example}}
```

### Use Big Integers with `num-bigint` {#num-bigint}

{{#badges num-bigint science= algorithms=Algorithms data-structures="Data structures"}}{{hi:Bignum}}{{hi:Mathematics}}{{hi:Numerics}}

[`num-bigint`][c~num-bigint~docs] provides arbitrary-precision signed and unsigned integers. Use it when an integer may exceed the range of `i128` or `u128`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/num_bigint.rs:example}}
```

### Work with Arbitrary-precision Integers, Rationals, Floats, and Complex Numbers with `rug` {#rug}

{{#badges rug api-bindings="API bindings" mathematics=Mathematics}}{{hi:Bignum}}{{hi:GMP}}{{hi:Mathematics}}{{hi:Numerics}}

[`rug`][c~rug~docs] provides arbitrary-precision integers, rationals, floating-point values, and [complex numbers][p~complex-numbers] through GMP, MPFR, and MPC. Choose it when those arbitrary-precision types or their numerical behavior fit your application and its native-library build requirements.

```rust,editable,noplayground
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rug.rs:example}}
```

## Use Fixed-Precision Decimal Numbers {#big-decimal}

{{#badges rust_decimal data-structures="Data structures" mathematics=Mathematics science=}}{{hi:Decimal}}{{hi:Financial}}{{hi:Number}}{{hi:Precision}}

[`rust_decimal`][c~rust_decimal~docs] stores a bounded 96-bit integer mantissa with a decimal scale and sign, making it useful for financial values and other base-10 calculations. Values within its range can represent decimal fractions exactly; division or values beyond that range still require an explicit rounding and overflow policy.

```rust,editable,noplayground
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/rust_decimal.rs:example}}
```

## Sort Floats {#sortable-floats}

{{#badges ordered-float no-std="No standard library" rust-patterns="Rust patterns" science=}}{{hi:f32}}{{hi:f64}}{{hi:Sorting}}

[`ordered-float`][c~ordered-float~docs] provides wrappers that implement total ordering for floating-point values so they can be used in ordered collections. `OrderedFloat` allows NaN and orders it after ordinary values; use `NotNan` when construction should reject NaN. For sorting alone, the standard library also provides `total_cmp`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/additional_numeric_types/ordered_float.rs:example}}
```

## `typenum` {#typenum}

{{#badges typenum no-std="No standard library"}}

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
  accuracy review: mathematics
</div>
