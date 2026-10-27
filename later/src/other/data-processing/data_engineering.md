<!-- TODO Reviewed ??-??-2026 -->

# Data Engineering

{{#include data_engineering.incl.md}}

## Process Columnar Data with `arrow` {#arrow}

[![arrow][c~arrow~docs~badge]][c~arrow~docs]{{hi:arrow}}
[![arrow~crates.io][c~arrow~crates.io~badge]][c~arrow~crates.io]
[![arrow~repo][c~arrow~repo~badge]][c~arrow~repo]
[![arrow~lib.rs][c~arrow~lib.rs~badge]][c~arrow~lib.rs]

[`arrow`][c~arrow~docs]↗{{hi:arrow}} is the official Rust implementation of `Apache Arrow`{{hi:In-memory}}. These examples build typed arrays and record batches, read and write CSV and newline-delimited JSON, and convert Serde-compatible Rust rows to and from Arrow.

### Build typed arrays and record batches {#arrow-arrays}

```rust,editable
{{#include ../../../crates/other/examples/data_processing/arrow/arrays.rs:example}}
```

### Read and write CSV with Arrow {#arrow-csv}

```rust,editable
{{#include ../../../crates/other/examples/data_processing/arrow/csv_io.rs:example}}
```

### Read and write JSON with Arrow {#arrow-json}

```rust,editable
{{#include ../../../crates/other/examples/data_processing/arrow/json_io.rs:example}}
```

### Convert Serde rows to and from Arrow {#arrow-serde}

```rust,editable
{{#include ../../../crates/other/examples/data_processing/arrow/serde_convert.rs:example}}
```

### Build columns with the `arrow-array` sub-crate {#arrow-sub-crates}

[![arrow-array][c~arrow-array~docs~badge]][c~arrow-array~docs]{{hi:arrow-array}} [![arrow-cast][c~arrow-cast~docs~badge]][c~arrow-cast~docs]{{hi:arrow-cast}}

Many projects depend on the smaller `arrow-array`, `arrow-cast` and `arrow-ipc` crates instead of the umbrella `arrow` crate, which shortens compile times and reduces binary size. `RecordBatch::try_from_iter` builds a batch from named columns and infers the schema, including nullability, from the arrays.

```rust,editable
{{#include ../../../crates/other/examples/data_processing/arrow/arrow_build.rs:example}}
```

### Save record batches as Arrow IPC files {#arrow-ipc}

[![arrow-ipc][c~arrow-ipc~docs~badge]][c~arrow-ipc~docs]{{hi:arrow-ipc}}

The Arrow IPC _file_ format supports random access and memory mapping. The IPC _stream_ format has no footer and suits sockets and pipes. Use Arrow IPC to exchange columnar data between processes and Arrow-compatible clients such as Python's `pyarrow`; use Parquet instead when compression and long-term storage matter more than zero-copy reads.

```rust,editable
{{#include ../../../crates/other/examples/data_processing/arrow/arrow_ipc_file.rs:example}}
```

## Query In-memory Data with `datafusion` {#datafusion}

[![datafusion][c~datafusion~docs~badge]][c~datafusion~docs]{{hi:datafusion}}
[![datafusion~crates.io][c~datafusion~crates.io~badge]][c~datafusion~crates.io]
[![datafusion~repo][c~datafusion~repo~badge]][c~datafusion~repo]
[![datafusion~lib.rs][c~datafusion~lib.rs~badge]][c~datafusion~lib.rs]

[`datafusion`][c~datafusion~docs]↗{{hi:datafusion}} is an extensible in-process SQL query engine built on Apache Arrow. It can query CSV, Parquet, JSON, and Avro data and can serve as the query layer for a database, data pipeline, or domain-specific engine.

```rust,editable
{{#include ../../../crates/other/examples/datafusion/datafusion.rs:example}}
```

## Distributed SQL with `ballista` {#ballista}

[`ballista`][c~ballista~docs]↗{{hi:ballista}} is a distributed query engine built on Apache Arrow and DataFusion. It extends DataFusion's query model across a cluster; choose it when a workload needs distributed execution rather than a single-process engine.

## Process DataFrames with Polars {#polars}

[![polars][c~polars~docs~badge]][c~polars~docs]{{hi:polars}}

Polars provides eager and lazy APIs for columnar data. This example filters inline sales rows, groups and sums revenue by region, and sorts the result. From `later/crates`, run it with `cargo run -p other --example polars --features polars`.

```rust,editable
{{#include ../../../crates/other/examples/polars/polars.rs:example}}
```

## DataFrame processing {#dataframes}

For in-memory table operations, see [[dataframes | Dataframes]], which covers Polars. This page focuses on Arrow interchange and query engines rather than repeating DataFrame recipes.

## Data apps with Spice.ai {#spice-ai}

[Spice.ai][spice.ai~website]↗ is a Rust-based query, search, and inference runtime for data applications. Its SQL and Arrow Flight interfaces connect applications to federated data sources; it is a runtime to deploy, not a Rust crate to add as a library dependency.

## Query JSON documents with JSONPath {#jsonpath}

[`serde_json_path`][c~serde_json_path~docs]↗ provides JSONPath queries over `serde_json` values. [`jsonpath-rust`][c~jsonpath-rust~docs] is another option; compare supported query features and error behavior against the JSONPath version your application needs.

## Visualize results {#visualization}

Use a visualization crate after selecting and transforming data; see [[visualization | Visualization]] for plotting and charting options.

## Related Topics {#related-topics .skip}

- [[data_processing | Data and ETL]].
- [[cloud | Cloud]].
- [[development_tools | Development Tools]].
- [[web-programming | Web Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
