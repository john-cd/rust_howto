<!-- TODO Reviewed ??-??-2026. -->

# Data Engineering

{{#include data_engineering.incl.md}}

## Process Columnar Data with `arrow` {#arrow}

{{#badges arrow}}

[`arrow`][c~arrow~docs]↗{{hi:arrow}} is the official Rust implementation of `Apache Arrow`{{hi:In-memory}}. These examples build typed arrays and record batches, read and write CSV and newline-delimited JSON, and convert Serde-compatible Rust rows to and from Arrow.

From `bk/crates`, use `--help` to list the recipes or omit the subcommand to run them all. The examples use inline data.

```console
cargo run --locked -p data_processing --features arrow --example data_processing -- --help
cargo run --locked -p data_processing --features arrow --example data_processing -- ipc
```

### Build typed arrays and record batches {#arrow-arrays}

```rust,ignore,noplayground
{{#include ../../../crates/other/data_processing/examples/data_processing/arrow/arrays.rs:example}}
```

### Read and write CSV with Arrow {#arrow-csv}

CSV carries text and headers. Pass the expected schema when you need the reader to preserve types. To process rows directly with Serde, see [CSV Processing](../../categories/encoding/csv.md).

```rust,ignore,noplayground
{{#include ../../../crates/other/data_processing/examples/data_processing/arrow/csv_io.rs:example}}
```

### Read and write JSON with Arrow {#arrow-json}

```rust,ignore,noplayground
{{#include ../../../crates/other/data_processing/examples/data_processing/arrow/json_io.rs:example}}
```

### Convert Serde rows to and from Arrow {#arrow-serde}

```rust,ignore,noplayground
{{#include ../../../crates/other/data_processing/examples/data_processing/arrow/serde_convert.rs:example}}
```

### Build columns with the `arrow-array` sub-crate {#arrow-sub-crates}

[![arrow-array][c~arrow-array~docs~badge]][c~arrow-array~docs]{{hi:arrow-array}} [![arrow-cast][c~arrow-cast~docs~badge]][c~arrow-cast~docs]{{hi:arrow-cast}}

Projects can depend on individual crates such as `arrow-array`, `arrow-cast` and `arrow-ipc` instead of the umbrella `arrow` crate when they need only those components. Measure compile time and binary size for the actual dependency and feature set. `RecordBatch::try_from_iter` builds a batch from named columns and infers the schema, including nullability, from the arrays.

```rust,ignore,noplayground
{{#include ../../../crates/other/data_processing/examples/data_processing/arrow/arrow_build.rs:example}}
```

### Save record batches as Arrow IPC files {#arrow-ipc}

[![arrow-ipc][c~arrow-ipc~docs~badge]][c~arrow-ipc~docs]{{hi:arrow-ipc}}

The Arrow IPC _file_ format supports random access and memory mapping. The IPC _stream_ format has no footer and suits sockets and pipes. Use Arrow IPC to exchange columnar data between processes and Arrow-compatible clients such as Python's `pyarrow`; use Parquet instead when compression and long-term storage matter more than zero-copy reads.

```rust,ignore,noplayground
{{#include ../../../crates/other/data_processing/examples/data_processing/arrow/arrow_ipc_file.rs:example}}
```

## Query In-memory Data with `datafusion` {#datafusion}

{{#badges datafusion}}

[`datafusion`][c~datafusion~docs]↗{{hi:datafusion}} is an extensible in-process SQL query engine built on Apache Arrow. This example registers an in-memory Arrow batch, then checks that SQL and the DataFrame API produce the same grouped sales totals. Both APIs build a query plan, and `collect().await` executes it and gathers the output batches.

From `bk/crates`, run:

```console
cargo run --locked -p data_processing --features datafusion --example datafusion
```

The example enables SQL and checks its expected results before exiting. When you pass batches from your own Arrow code to DataFusion, use compatible Arrow crate versions. DataFusion re-exports its Arrow types under `datafusion::arrow`. DataFusion also supports file sources such as CSV and JSON, and optional features add Parquet and Avro support.

```rust,ignore,noplayground
{{#include ../../../crates/other/data_processing/examples/datafusion/datafusion.rs:example}}
```

## Distributed SQL with `ballista` {#ballista}

[`ballista`][c~ballista~docs]↗{{hi:ballista}} is a distributed query engine built on Apache Arrow and DataFusion. It extends DataFusion's query model across a cluster. Start with DataFusion for an in-process workload. Ballista also requires scheduler and executor services.

## Data apps with Spice.ai {#spice-ai}

[Spice.ai][spice.ai~docs]↗ is a Rust-based query, search, and inference runtime for data applications. Its SQL and Arrow Flight interfaces connect applications to federated data sources; run it as a separate service when you need a federated query endpoint.

## Query JSON documents with JSONPath {#jsonpath}

[`serde_json_path`][c~serde_json_path~docs]↗ provides JSONPath queries over `serde_json` values. [`jsonpath-rust`][c~jsonpath-rust~docs] is another option; compare supported query features and error behavior against the JSONPath version your application needs.

## Related Topics {#related-topics .skip}

- [Data Processing](index.md) for choosing a format or query engine.
- [DataFrames](dataframes.md) for Polars filters, aggregations, and joins.
- [CSV Processing](../../categories/encoding/csv.md) for reading and writing individual rows.
- [Web Programming](../../categories/web-programming/index.md) for exposing results over HTTP.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
