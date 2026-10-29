<!-- TODO Reviewed ??-??-2026 -->

# Finance

Finance software covers financial records, quantitative models, and market-facing applications. Keep monetary amounts tied to an explicit currency, and check the rules and data rights of each financial institution or provider you use.

{{#include index.incl.md}}

## Representing Money

Use [`rust_decimal`][c~rust_decimal~docs]↗ for decimal monetary amounts and [`iso_currency`][c~iso_currency~docs]↗ for ISO 4217 currency codes. Store the amount and currency separately, and define rounding at conversion and settlement boundaries; a decimal type does not choose a business rounding policy for you.

Floating-point values are common for statistical calculations and pricing models. Avoid them for ledger values that require exact decimal arithmetic.

See [[additional_numeric_types | Additional Numeric Types]].

For `.xlsx` import and export, see [[excel | Excel Spreadsheets]] for [`calamine`][c~calamine~docs]↗ and [`rust_xlsxwriter`][c~rust_xlsxwriter~docs]↗.

### Money with a Currency Attached {#rusty-money}

[![rusty-money][c~rusty-money~docs~badge]][c~rusty-money~docs]{{hi:rusty-money}} [![rusty-money~crates.io][c~rusty-money~crates.io~badge]][c~rusty-money~crates.io] [![rusty-money~repo][c~rusty-money~repo~badge]][c~rusty-money~repo] [![rusty-money~lib.rs][c~rusty-money~lib.rs~badge]][c~rusty-money~lib.rs]{{hi:Money}}{{hi:Currency}}

[`rusty-money`][c~rusty-money~docs]↗ builds on `rust_decimal`: a `Money` value holds a decimal amount and a reference to its currency, so the two cannot drift apart. It includes the ISO 4217 currency table and formats amounts by the currency's conventions. `Money::add` and the other checked operations return an error when the currencies differ or the result overflows, instead of silently producing a number. Display rounds to the currency's minor unit (half-even) without changing the stored amount; round explicitly where your accounting rules require it.

```rust,editable
{{#include ../../../crates/cats/finance/examples/money/rusty_money.rs:example}}
```

## Related Topics

- [[date-and-time | Date and Time]].
- [[statistics | Statistics]].
- [[visualization | Visualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
