<!-- TODO Reviewed ??-??-2026 -->

# Complex Numbers

{{#include complex_numbers.incl.md}}

## Create Complex Numbers {#creating-complex-numbers}

[![num][c~num~docs~badge]][c~num~docs]{{hi:num}}
[![num~crates.io][c~num~crates.io~badge]][c~num~crates.io]
[![num~repo][c~num~repo~badge]][c~num~repo]
[![num~lib.rs][c~num~lib.rs~badge]][c~num~lib.rs]
[![cat~science][cat~science~badge]][cat~science]
[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}
[![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The following example creates complex numbers of type [`num::complex::Complex`][c~num::complex::Complex~docs]↗{{hi:num::complex::Complex}}. The real and imaginary components of a complex number must have the same type:

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/create_complex.rs:example}}
```

## Add Complex Numbers {#adding-complex-numbers}

This example adds complex numbers with the `+` operator, as with built-in numeric types. Both components of a complex number must have the same type. Multiplication and division are also supported, but are outside this chapter's example scope.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/add_complex.rs:example}}
```

## Use Mathematical Functions on Complex Numbers {#mathematical-functions}

The complex exponential demonstrates Euler's identity: mathematically, `e^(2πi) = 1`. With floating-point values, the printed result is approximately `1+0i`; the small imaginary component comes from rounding. The [`Complex` API][c~num::complex::Complex~docs]↗{{hi:num::complex::Complex}} also provides [`exp`][c~num::complex::Complex::exp~docs], [`sin`][c~num::complex::Complex::sin~docs], [`norm`][c~num::complex::Complex::norm~docs], [`arg`][c~num::complex::Complex::arg~docs], [`to_polar`][c~num::complex::Complex::to_polar~docs], and [`from_polar`][c~num::complex::Complex::from_polar~docs]. This chapter demonstrates only `exp`; the other listed methods are outside its example scope.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/complex_numbers/mathematical_functions.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[additional_numeric_types | Additional Numeric Types]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
