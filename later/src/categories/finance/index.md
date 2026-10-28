<!-- TODO Reviewed ??-??-2026 -->

# Finance

Finance software covers financial records, quantitative models, and market-facing applications. Keep monetary amounts tied to an explicit currency, and check the rules and data rights of each financial institution or provider you use.

{{#include index.incl.md}}

## Representing Money

Use [`rust_decimal`][c~rust_decimal~docs]↗ for decimal monetary amounts and [`iso_currency`][c~iso_currency~docs]↗ for ISO 4217 currency codes. Store the amount and currency separately, and define rounding at conversion and settlement boundaries; a decimal type does not choose a business rounding policy for you.

Floating-point values are common for statistical calculations and pricing models. Avoid them for ledger values that require exact decimal arithmetic.

See [[additional_numeric_types | Additional Numeric Types]].

For `.xlsx` import and export, see [[excel | Excel Spreadsheets]] for [`calamine`][c~calamine~docs]↗ and [`rust_xlsxwriter`][c~rust_xlsxwriter~docs]↗.

## Related Topics

- [[date-and-time | Date and Time]].
- [[statistics | Statistics]].
- [[visualization | Visualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
