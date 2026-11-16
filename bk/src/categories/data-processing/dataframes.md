<!-- TODO Reviewed ??-??-2026. -->

# DataFrames {#dataframes}

{{#include dataframes.incl.md}}

## Filter, project, aggregate, and join with Polars {#polars}

[![Polars][c~polars~docs~badge]][c~polars~docs]{{hi:polars}} [![Polars Book][c~polars~book~badge]][c~polars~book] [![Polars GitHub][c~polars~repo~badge]][c~polars~repo]

This host example creates a small DataFrame in memory, filters rows lazily,
selects columns, sums revenue by region, sorts the summary, and joins regional
targets. Schema and value assertions keep the example self-checking.

```rust,editable
{{#include ../../../crates/cats/data_processing/examples/dataframes/dataframes.rs:example}}
```

From `bk/crates`, run the example without any input files:

```console
cargo run --locked -p data_processing --example dataframes
```

## Scan CSV and Parquet lazily {#dataframe-scans}

For file-backed work, Polars can scan CSV and Parquet sources into lazy queries.
Enable the corresponding Polars features in the manifest; the example package
enables `lazy` only and uses inline data. Lazy queries run when
`collect()` is called, allowing the planner to optimize operations such as
filtering and column selection. See the [Polars Rust lazy guide][c~polars~lazy~guide] and
[lazy sources and sinks guide][c~polars~sources~guide].

## DataFusion alternative {#datafusion}

Polars provides a high-level DataFrame API for local transformations.
[Apache Arrow][arrow~website] defines a columnar in-memory format and
[Apache Parquet][parquet~website] a columnar file format. Use Arrow APIs when
interoperability and explicit columnar data structures are central. Use
[DataFusion][c~datafusion~docs] when an embeddable SQL and query engine over
Arrow data better fits the application.

## Interactive and larger-scale workflows {#interactive-and-larger-scale}

For interactive exploration, the [Evcxr Jupyter kernel][c~evcxr_jupyter~readme~repo]
lets you evaluate Rust code in notebook cells; the command-line example above
remains a reproducible way to rerun the same transformation.

[LakeSail][lakesail~website] and [ClickHouse][c~clickhouse~website] are options
to explore when analytics need distributed processing or a service-backed
database. They address larger system needs than this local Polars example.

## Related Topics {#related-topics .skip}

- [Data Processing](index.md).
- [DataFusion crate][c~datafusion~docs].
- [Polars crate][c~polars~docs].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
