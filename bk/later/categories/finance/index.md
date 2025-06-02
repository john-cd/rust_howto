# Finance

Dealing with money: accounting, trading, investments, taxes, banking and payment processing using government-backed currencies.

Date/time handling is essential. Data access depends on your data source. Trading and accounting often require custom solutions or very specific API integrations.

## Working with Currencies

`iso_currency`
[`iso4217`][c-iso4217]⮳{{hi:iso4217}} provides ISO 4217 currency codes.
[`rust-money`][c-rust_money]⮳{{hi:rust-money}} for working with monetary values and currencies.

## Quantitative Analysis

{{#include quant.incl.md}}

For advanced analysis, you'll likely need the statistics and linear algebra crates.

## Financial Modeling

Building models to analyze financial data.

## Financial Calculations

[`financial`][c-financial]⮳{{hi:financial}}: Provides a range of financial calculations (present value, future value, interest rates, etc.). A good starting point.

- Time value of money: Present value, future value, etc.
- Interest rates: Simple, compound, APR, APY.
- Financial instruments: Stocks, bonds, options, etc.

## Related Topics

## Decimal Number Handling

See [[additional_numeric_types | Additional Numeric Types]].

### Date and Time Handling

[`chrono`][c-chrono]⮳{{hi:chrono}}: A widely used crate for date and time calculations.
[`time`][c-time]⮳{{hi:time}}: A newer crate, often preferred.

See [[date-and-time | Date and Time]].

### Data Analysis and Statistics

[`statrs`][c-statrs]⮳{{hi:statrs}}: A comprehensive statistics library.
[`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}: Linear algebra crate, often used in financial modeling.
[`ndarray`][c-ndarray]⮳{{hi:ndarray}}: N-dimensional arrays, useful for handling financial data.

See [[mathematics | Mathematics]], [[linear_algebra | Linear Algebra]], and [[statistics | Statistics]].

### Data Access

There are many options depending on your data source.

Examples include [`reqwest`][c-reqwest]⮳{{hi:reqwest}} for making HTTP requests to financial APIs,
[`csv`][c-csv]⮳{{hi:csv}} for reading and writing CSV files (common for financial data), and [`serde`][c-serde]⮳{{hi:serde}} for serializing and deserializing financial data.

See [[encoding | Encoding]], [[serde | Serde]], [[csv | CSV]], [[json | JSON]], [[web-programming_http-client | Web Programming: HTTP Client]].

## Charting and Visualization

[`plotters`][c-plotters]⮳{{hi:plotters}} is a plotting library that can create charts and graphs of financial data.

See [[visualization | Visualization]].

## Trading and Market Data

Often requires specific API wrappers or libraries for the exchanges or data providers you're using. There isn't a single "trading crate."

## Risk Management

Risk management: Volatility, diversification, etc.

Often involves custom calculations and models. The statistics and linear algebra crates above can be helpful.

## Accounting

There aren't widely used, general-purpose accounting crates in Rust yet. This area is often handled with custom solutions.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/365)
Needs in-depth review.

Review / incorporate:

- [`rust_decimal`][c-rust_decimal]⮳{{hi:rust_decimal}} - ~2M downloads.
- `iso_currency` - ~19k downloads.
- [`RustQuant`][c-rustquant]⮳{{hi:RustQuant}} ~4.1k.
- `apca` mentioned in Awesome Rust.
- `stochastic-rs` mentioned in Awesome Rust.
- Mention [`financial`][c-financial]⮳{{hi:financial}} a collection of finance calculations mimicking some of Excel Financial Functions interface.
- Mention `black_scholes`.

Cover Excel data import, yahoo finance, databento, alpaca.

- [yata: Yet Another Technical Analysis library [for Rust]](https://github.com/amv-dev/yata)

</div>
