<!-- TODO Reviewed ??-??-2026 -->
# Data and ETL

Data work ranges from reading local files and shaping tables to building pipelines,
query engines, and analytics systems. Start with a recipe page, then explore the
project links for tools that support larger workflows.

## Import and transform files

The [CSV chapter](csv.md) shows command-line operations for inspecting, filtering,
splitting, and joining delimited files.

{{#include csv.incl.md}}

## Work with tabular data

The [dataframes chapter](dataframes.md) uses Polars for columnar, dataframe-style
operations over structured data.

{{#include dataframes.incl.md}}

## Build data systems

The [data engineering chapter](data_engineering.md) introduces Arrow, DataFusion,
Databend, and Rerun through focused examples.

{{#include data_engineering.incl.md}}

## Explore the ecosystem

### Storage, formats, and query

- [Neon Serverless Postgres][neon~website]↗.
- [ClickHouse][clickhouse~repo]↗ for real-time analytics.
- [`cube-js`][cube~repo]↗, a semantic layer for data applications.
- [`lance`][lance~repo]↗, a columnar format for machine learning and LLM workflows.
- [`materialize`][materialize~repo]↗ for querying fast-changing data with SQL.
- [`risingwave`][risingwave~repo]↗ for streaming analytics and real-time data pipelines.
- [LakeSail][lakesail~website]↗ for big data processing.

### Pipelines and storage access

- [Apache OpenDAL][opendal~website]↗ provides a unified interface to storage services.
- [`fluvio`][fluvio~repo]↗ is a distributed stream-processing platform.
- [Hydro][hydro~website]↗ is a framework for dataflow programs.
- [`pathway`][pathway~repo]↗ is a Python framework for streaming ETL and analytics.
- [`recurring-tasks`][recurring-tasks~repo]↗ schedules periodic work in Rust applications.
- [`vector`][vector~repo]↗ is a high-performance observability data pipeline.

### Observability

- [Seq][datalust~website]↗ provides centralized structured-log search and analysis.

### Further reading and tools

- [AWS Lambda with DuckDB and Delta Lake][blog~aws-lambda-duckdb-and-delta-lake]↗.
- [Elusion][c~elusion~lib.rs]↗, a Rust data utility.
- [Data with Rust][datawithrust~website]↗.
- [A community discussion about Rust data-engineering projects][reddit~data-engineering]↗.

## Related topics

- [[cloud | Cloud]] deployment.
- [[web-programming | Web Programming]].
- [[development_tools | Development Tools]].
- [[visualization | Visualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize / edit](https://github.com/john-cd/rust_howto/issues/594)
[content audit: content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1904)
</div>
