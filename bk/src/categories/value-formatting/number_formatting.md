<!-- TODO Reviewed ??-??-2026. -->

# Number Formatting

[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

{{#include number_formatting.incl.md}}

Use Rust formatting syntax for padding, precision, exponents, and integer radix. Use [`num-format`][c~num-format~docs] for grouped integers with an explicit locale. Use ICU4X for locale-specific decimal or currency formatting.

From `bk/crates`, run the examples with the command for the output you need:

```console
cargo run --locked -p value_formatting --example number_formatting -- standard-and-localized
cargo run --locked -p value_formatting --example number_formatting -- humansize
cargo run --locked -p value_formatting --example number_formatting -- minor-units
cargo run --locked -p value_formatting --example number_formatting -- itoa
cargo run --locked -p value_formatting --example number_formatting -- ryu
```

## Format Numbers with Standard Rust Syntax and Locale Grouping {#standard-and-locale-formatting}

This test-backed example covers alignment, precision, scientific notation, hexadecimal and binary output, plus English and German thousands separators.

```rust,ignore,noplayground
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/standard_and_localized.rs:example}}
```

## Display Money Stored in Minor Units {#minor-units}

Store an amount as an integer when the currency and scale are known. US dollars use 100 cents per dollar. Keep the currency with the amount and use the numeric value for calculations. Apply the domain's rounding rule when converting to minor units.

Separate the sign before splitting the magnitude into whole and fractional parts. `unsigned_abs` also handles `i64::MIN`. The example uses a period and two fractional digits. For localized currency output, use a formatter that knows the locale and currency.

```rust,ignore,noplayground
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/minor_units.rs:example}}
```

## Format a Byte Count for Display with `humansize` {#humansize}

{{#badges humansize value-formatting=}}

[`humansize::format_size`][c~humansize~docs]↗{{hi:humansize}} formats a number of bytes as text. `DECIMAL` uses powers of 1000 (`kB`, `MB`), and `BINARY` uses powers of 1024 (`KiB`, `MiB`). `format_size_i` accepts signed values, such as a change in memory use.

```rust,ignore,noplayground
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/humansize_example.rs:example}}
```

## Convert Integers to String Quickly with `itoa` {#itoa}

{{#badges itoa no-std="No standard library" no-std::no-alloc="No dynamic allocation" value-formatting="Value formatting"}}{{hi:Integer}}

Fast integer primitive to string conversion.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/itoa.rs:example}}
```

## Convert Floating Point to String Quickly with `ryu` {#ryu}

{{#badges ryu no-std="No standard library" no-std::no-alloc="No dynamic allocation" value-formatting="Value formatting"}}{{hi:Float}}

Fast floating point to string conversion.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/ryu.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Value Formatting](index.md).
- [Additional Numeric Types](../mathematics/additional_numeric_types.md) for decimal arithmetic.
- [Strings](../data-structures/strings.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [`num-format`][c~num-format~docs]↗{{hi:num-format}}, [`icu`][c~icu~docs]↗{{hi:icu}}, and [`fluent`][c~fluent~docs]↗{{hi:fluent}} address grouped integers, locale-aware decimal output, and translated messages, respectively.
</div>
