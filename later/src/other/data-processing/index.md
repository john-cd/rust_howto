<!-- TODO Reviewed ??-??-2026. -->

# Data and ETL

Data work ranges from reading local files and shaping tables to building pipelines,
query engines, and analytics systems. Start with a recipe page, then explore the
project links for tools that support larger workflows.

Choose an entry point by the data you have and the output you need. The first sections cover local transformations and queries. The later sections cover recurring pipelines, storage, and operations.

## Import and transform files

The [CSV chapter](csv.md) shows command-line operations for inspecting, filtering,
splitting, and joining delimited files.

{{#include csv.incl.md}}

## Work with tabular data

The [dataframes chapter](dataframes.md) uses Polars for columnar, dataframe-style
operations over structured data.

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

[Vector][vector-observability~repo]↗ is a production example of an observability data pipeline with sources, transforms, and sinks.

### Further reading and tools

- [AWS Lambda with DuckDB and Delta Lake][blog~aws-lambda-duckdb-and-delta-lake]↗.
- [Elusion][c~elusion~lib.rs]↗, a Rust data utility.
- [Data with Rust][datawithrust~website]↗.
- [A community discussion about Rust data-engineering projects][reddit~data-engineering]↗.

### Explore Data Interactively

Use [Evcxr Jupyter][evcxr~jupyter~guide]↗ to explore small Rust data examples interactively. See [LakeSail][lakesail~website]↗ for a Spark-compatible engine and the [ClickHouse Rust client][clickhouse~rust-client~guide]↗ for a service-based OLAP workflow. [Awesome Rust Data Science][awesome-rust-datascience~repo]↗ collects libraries by task.

## Related topics

- [[cloud | Cloud]] deployment.
- [[web-programming | Web Programming]].
- [[development_tools | Development Tools]].
- [[visualization | Visualization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
