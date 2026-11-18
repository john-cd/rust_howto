<!-- TODO Reviewed ??-??-2026. -->

# Number Formatting

Formatting turns a numeric value into text for people to read. Keep that display step separate from parsing, arithmetic, and serialization: formatted text can use locale-specific separators and is usually a poor interchange format.

## Use Rust's Formatting Syntax

The standard formatting macros cover common display needs without an extra crate. For floating-point values, precision controls the number of digits after the decimal point in the rendered text; it does not change the stored value. Width and alignment are useful for reports, but they do not add locale-specific grouping.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/number_formatting/number_formatting.rs:example}}
```

[`itoa`][c~itoa~docs]↗{{hi:itoa}} formats integers and [`ryu`][c~ryu~docs]↗{{hi:ryu}} formats floating-point values into caller-provided buffers. They are useful when a low-allocation conversion matters, and are also used inside serialization libraries. For ordinary application output, start with `format!`, `write!`, or the `Display` trait.

## Preserve Numeric Meaning

Do not use a rounded display string as the source for further arithmetic. Keep the original numeric value and apply rounding at the boundary where the domain requires it. For money, use an integer minor-unit representation or a decimal type such as [`rust_decimal`][c~rust_decimal~docs]↗{{hi:rust_decimal}}; attach a currency and define its rounding rule explicitly.

For localized output, use a locale-aware formatter instead of hard-coding commas, periods, or currency symbols. [`num-format`][c~num-format~docs]↗{{hi:num-format}} handles locale-specific grouping, while ICU provides broader locale data for numbers, dates, and currencies. See [Internationalization](../internationalization/index.md) and [Localization](../localization/index.md) for choosing and applying a locale.

## Related Topics {#related-topics .skip}

- [Identifier case conversion](case_conversion.md).
- [Internationalization](../internationalization/index.md) and [Localization](../localization/index.md).
- [[additional_numeric_types | Additional Numeric Types]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
