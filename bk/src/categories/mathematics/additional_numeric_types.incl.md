<!-- TODO Reviewed ??-??-2026 -->

Choose a numeric representation for the range, precision, and operations your problem needs. Use [`num-bigint`][c~num-bigint~docs] for integers that can exceed the built-in types, [`rust_decimal`][c~rust_decimal~docs] for bounded base-10 values such as financial amounts, [`num-rational`][c~num-rational~docs] for exact fractions, [`fixed`][c~fixed~docs] when the number of fractional bits is fixed, and [`half`][c~half~docs] for compact IEEE 754 binary16 storage when its lower precision is acceptable. For arbitrary-precision rationals, floats, or complex numbers backed by GMP, MPFR, and MPC, consider [`rug`][c~rug~docs].

| Representation | Recommended crate | Use it when |
|-----------------|-------------------|-------------|
| Arbitrary-precision integer | [`num-bigint`][c~num-bigint~docs] | Built-in integer ranges are too small. |
| Bounded decimal | [`rust_decimal`][c~rust_decimal~docs] | Base-10 scale and predictable decimal rounding matter. |
| Rational number | [`num-rational`][c~num-rational~docs] | Keep fractions exact as numerator and denominator. |
| Fixed-point number | [`fixed`][c~fixed~docs] | The binary fractional precision is known in advance. |
| Half-precision float | [`half`][c~half~docs] | Reduce storage with IEEE 754 binary16 when reduced precision is acceptable. |
| Recipe | Crates | Categories |
|--------|--------|------------|
| [Abstract over Different Number Types][ex~mathematics~abstracting-over-number-types] | [![num-traits][c~num-traits~docs~badge]][c~num-traits~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] |
| [Use Big Integers][ex~mathematics~big-integers] | [![num][c~num~docs~badge]][c~num~docs] [![num-bigint][c~num-bigint~docs~badge]][c~num-bigint~docs] [![rug][c~rug~docs~badge]][c~rug~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] [![cat~science][cat~science~badge]][cat~science] |
| [Use Fixed-Precision Decimal Numbers][ex~mathematics~big-decimal] | [![rust_decimal][c~rust_decimal~docs~badge]][c~rust_decimal~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] |
| [Sort Floats][ex~mathematics~sortable-floats] | [![ordered-float][c~ordered-float~docs~badge]][c~ordered-float~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] |
| [Use Big Integers with `num-bigint`][ex~mathematics~num-bigint] | [![num-bigint][c~num-bigint~docs~badge]][c~num-bigint~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] [![cat~science][cat~science~badge]][cat~science] |
| [`num`][ex~mathematics~num] | [![num][c~num~docs~badge]][c~num~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] [![cat~science][cat~science~badge]][cat~science] |
| [Work with Arbitrary-precision Integers, Rationals, Floats, and Complex Numbers with `rug`][ex~mathematics~rug] | [![rug][c~rug~docs~badge]][c~rug~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] [![cat~science][cat~science~badge]][cat~science] |
| [`typenum`][ex~mathematics~typenum] | [![typenum][c~typenum~docs~badge]][c~typenum~docs]{{hi:typenum}} | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] |
| [Rational Numbers][ex~mathematics~rational-numbers] | [![num-rational][c~num-rational~docs~badge]][c~num-rational~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] |
| [Fixed-Point Numbers][ex~mathematics~fixed-point] | [![fixed][c~fixed~docs~badge]][c~fixed~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] |
| [Half-Precision Floats][ex~mathematics~half-precision] | [![half][c~half~docs~badge]][c~half~docs] | [![cat~mathematics][cat~mathematics~badge]][cat~mathematics] |
