<!-- TODO Reviewed ??-??-2026 -->

# Format Values for Display to a User, Potentially Adapting the Display to Various Languages and Regions

[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

{{#include number_formatting.incl.md}}

Use Rust's standard formatting syntax for padding, precision, exponent notation, and integer radix when output is not locale-sensitive. Use [`num-format`][c~num-format~docs]↗ for grouped integers with an explicit locale. For full decimal conventions such as localized decimal separators and currency, use ICU4X; for translated messages, Fluent provides the `NUMBER()` function. See [the Fluent recipe issue #797](https://github.com/john-cd/rust_howto/issues/797) rather than repeating its message-formatting example here.

## Format Numbers with Standard Rust Syntax and Locale Grouping {#standard-and-locale-formatting}

This test-backed example covers alignment, precision, scientific notation, hexadecimal and binary output, plus English and German thousands separators.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/standard_and_localized.rs:example}}
```

For locale-aware decimal formatting with ICU4X, see [Internationalization](../internationalization/internationalization.md#internationalization). For translated messages that contain numbers, see [Translate Messages and Test Localized Behavior](../internationalization/internationalization.md#translate-messages-and-test-localized-behavior), which discusses Fluent.

## Convert Floating Point to String Quickly with `ryu` {#ryu}

[![ryu][c~ryu~docs~badge]][c~ryu~docs] [![ryu~crates.io][c~ryu~crates.io~badge]][c~ryu~crates.io] [![ryu~repo][c~ryu~repo~badge]][c~ryu~repo] [![ryu~lib.rs][c~ryu~lib.rs~badge]][c~ryu~lib.rs]{{hi:ryu}}{{hi:Float}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}} [![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

Fast floating point to string conversion.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/ryu.rs:example}}
```

## Convert Identifiers Between Case Styles {#case-conversion}

[`heck`][c~heck~docs]↗{{hi:heck}} converts identifiers between common case conventions.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/case_conversion/case_conversion.rs:example}}
```

## Convert Integers to String Quickly with `itoa` {#itoa}

[![itoa][c~itoa~docs~badge]][c~itoa~docs] [![itoa~crates.io][c~itoa~crates.io~badge]][c~itoa~crates.io] [![itoa~repo][c~itoa~repo~badge]][c~itoa~repo] [![itoa~lib.rs][c~itoa~lib.rs~badge]][c~itoa~lib.rs]{{hi:itoa}}{{hi:Integer}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}} [![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

Fast integer primitive to string conversion.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/itoa.rs:example}}
```

## Format a Byte Count for Display with `humansize` {#humansize}

[![humansize][c~humansize~docs~badge]][c~humansize~docs]{{hi:humansize}} [![humansize~crates.io][c~humansize~crates.io~badge]][c~humansize~crates.io] [![humansize~repo][c~humansize~repo~badge]][c~humansize~repo] [![humansize~lib.rs][c~humansize~lib.rs~badge]][c~humansize~lib.rs] [![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]

[`humansize::format_size`][c~humansize~docs]↗{{hi:humansize}} formats a number of bytes as text. `DECIMAL` uses powers of 1000 (`kB`, `MB`), and `BINARY` uses powers of 1024 (`KiB`, `MiB`). `format_size_i` accepts signed values, such as a change in memory use.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/humansize_example.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[internationalization | Internationalization]].
- [[localization | Localization]].
- [[text-processing | Text Processing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [`num-format`][c~num-format~docs]↗{{hi:num-format}}, [`icu`][c~icu~docs]↗{{hi:icu}}, and [`fluent`][c~fluent~docs]↗{{hi:fluent}} address grouped integers, locale-aware decimal output, and translated messages, respectively.
  [value-formatting: choose crates, write](https://github.com/john-cd/rust_howto/issues/490)
</div>
