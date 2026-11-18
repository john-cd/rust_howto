<!-- TODO Reviewed ??-??-2026. -->

# Data and ETL

Data work ranges from reading local files and shaping tables to building pipelines,
query engines, and analytics systems. Start with a recipe page, then explore the
project links for tools that support larger workflows.

## Choose a Tool by Task

| Need | Common crates | Start here |
| --- | --- | --- |
| Validate or transform rows in a delimited file | [`csv`](https://docs.rs/csv/) and [`serde`](https://docs.rs/serde/) | Stream rows in the [CSV recipe](csv.md#read-and-write-csv-files). |
| Filter, group, or join a table | [`polars`](https://docs.rs/polars/) | The [published dataframe chapter](https://github.com/john-cd/rust_howto/blob/db7033a53e8c5f6cfdc66ad0a9c89105319ebe30/bk/src/categories/data-processing/dataframes.md) covers lazy file scans. |
| Exchange typed columns with another program | [`arrow`](https://docs.rs/arrow/) | Use [Arrow IPC](data_engineering.md#arrow-ipc) when the reader consumes Arrow batches. |
| Store compressed columnar data on disk | [`parquet`](https://docs.rs/parquet/) | Use Parquet for file-backed analytics that read a subset of columns. |
| Run SQL inside a Rust process | [`datafusion`](https://docs.rs/datafusion/) | Start with the [in-memory DataFusion example](data_engineering.md#datafusion) before adding file sources. |
| Read or write bytes in object storage | [`object_store`](https://docs.rs/object_store/) or [`opendal`](https://docs.rs/opendal/) | Keep storage credentials and connection options separate from format decoding and queries. |

## Import and transform files

The [CSV chapter](csv.md) covers typed streaming in Rust and command-line
operations for inspecting, filtering, splitting, and joining delimited files.

{{#include csv.incl.md}}

## Work with tabular data

The published dataframe chapter linked above uses Polars for columnar
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
