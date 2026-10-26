<!-- TODO Reviewed ??-??-2026 -->

# Dataframes

{{#include dataframes.incl.md}}

## Create and Query DataFrames {#dataframes}

### Filter, Project, Aggregate, and Join with Polars {#polars}

[![polars][c~polars~docs~badge]][c~polars~docs]{{hi:polars}} [![`polars` Book][c~polars~book~badge]][c~polars~book] [![`polars` GitHub][c~polars~repo~badge]][c~polars~repo]{{hi:Dataframes}}

Use `df!` to build a small DataFrame in memory. This example filters and selects columns lazily, groups rows by region, sums revenue, sorts the result, and joins the totals to a target table. Tests verify both the aggregate value and joined row count.

```rust,editable
{{#include ../../../crates/other/examples/polars/polars.rs:example}}
```

## Scan CSV and Parquet Lazily {#dataframe-scans}

Use `LazyCsvReader` for CSV input and `LazyFrame::scan_parquet` for Parquet input; the latter requires Polars' `parquet` feature in addition to `lazy`. Lazy scans defer execution until `collect()` and let query planning push filters and column selection toward the source. The tested example above uses inline data, so it needs no input files.

See the [Polars Rust lazy guide][c~polars~lazy~guide] and [lazy source and sink guide][c~polars~sources~guide] for API details.

## DataFusion Alternative {#datafusion}

DataFusion also offers SQL and a Rust DataFrame API over Apache Arrow. See the [DataFusion example][ex~data-processing~datafusion] when a query engine or SQL interface is a better fit.

## Related Topics {#related-topics .skip}

- [[data_processing | Data and ETL]].
- [[python | Python]].
- [[mathematics | Mathematics]].
- [[visualization | Visualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
