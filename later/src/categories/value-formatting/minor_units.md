<!-- TODO Reviewed ??-??-2026. -->

# Display Money Stored in Minor Units

Store amounts in integer minor units when the currency and scale are already known. For example, US dollars have 100 cents per dollar. The published [Number Formatting](https://github.com/john-cd/rust_howto/blob/c9d457b66ed12377a6ff5e0e1377af71cc9dde0b/bk/src/categories/value-formatting/number_formatting.md) chapter covers standard numeric display and grouping.

## Format a Signed Amount {#number-formatting}

Separate the sign before splitting the amount into whole and fractional parts. The example uses `unsigned_abs` so the smallest `i64` is representable as a magnitude. This recipe uses two fractional digits. Other currencies can use a different scale. Keep the currency alongside the amount in application data.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/number_formatting.rs:example}}
```

[`itoa`][c~itoa~docs]↗{{hi:itoa}} formats integers and [`ryu`][c~ryu~docs]↗{{hi:ryu}} formats floating-point values into caller-provided buffers. They are useful when a low-allocation conversion matters, and are also used inside serialization libraries. For ordinary application output, start with `format!`, `write!`, or the `Display` trait.

## Preserve Numeric Meaning

Do not use a rounded display string as the source for further arithmetic. Keep the original numeric value and apply rounding at the boundary where the domain requires it. For money, use an integer minor-unit representation or a decimal type such as [`rust_decimal`][c~rust_decimal~docs]↗{{hi:rust_decimal}}; attach a currency and define its rounding rule explicitly.

For localized output, use a locale-aware formatter instead of hard-coding commas, periods, or currency symbols. [`num-format`][c~num-format~docs]↗{{hi:num-format}} handles locale-specific grouping, while ICU provides broader locale data for numbers, dates, and currencies. See [Internationalization](https://github.com/john-cd/rust_howto/blob/c9d457b66ed12377a6ff5e0e1377af71cc9dde0b/bk/src/categories/internationalization/index.md) and [Localization](https://github.com/john-cd/rust_howto/blob/c9d457b66ed12377a6ff5e0e1377af71cc9dde0b/bk/src/categories/localization/index.md) for choosing and applying a locale.

## Related Topics {#related-topics .skip}

- [Identifier case conversion](case_conversion.md).
- [Internationalization](https://github.com/john-cd/rust_howto/blob/c9d457b66ed12377a6ff5e0e1377af71cc9dde0b/bk/src/categories/internationalization/index.md) and [Localization](https://github.com/john-cd/rust_howto/blob/c9d457b66ed12377a6ff5e0e1377af71cc9dde0b/bk/src/categories/localization/index.md).
- [[additional_numeric_types | Additional Numeric Types]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
