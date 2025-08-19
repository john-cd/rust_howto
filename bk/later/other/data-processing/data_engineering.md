# Data Engineering

{{#include data_engineering.incl.md}}

## Process Columnar Data with `arrow` {#arrow}

[![arrow][c~arrow~docs~badge]][c~arrow~docs]{{hi:arrow}}
[![arrow~crates.io][c~arrow~crates.io~badge]][c~arrow~crates.io]
[![arrow~github][c~arrow~github~badge]][c~arrow~github]
[![arrow~lib.rs][c~arrow~lib.rs~badge]][c~arrow~lib.rs]

[`arrow`][c~arrow~docs]↗{{hi:arrow}} is the official Rust implementation of `Apache Arrow`{{hi:In-memory}}

```rust,editable
{{#include ../../../crates/other/examples/data_processing/arrow.rs:example}}
```

## Query In-memory Data with `datafusion` {#datafusion}

[![datafusion][c~datafusion~docs~badge]][c~datafusion~docs]{{hi:datafusion}}
[![datafusion~crates.io][c~datafusion~crates.io~badge]][c~datafusion~crates.io]
[![datafusion~github][c~datafusion~github~badge]][c~datafusion~github]
[![datafusion~lib.rs][c~datafusion~lib.rs~badge]][c~datafusion~lib.rs]

[![datafusion][c~datafusion~docs~badge]][c~datafusion~docs]↗{{hi:datafusion}} is the `Apache Arrow DataFusion` {{i:SQL}} Query Engine.

{{i:Apache DataFusion}} is an in-memory query engine that uses Apache Arrow as the memory model

[`datafusion`][c~datafusion~docs]↗{{hi:datafusion}} offers SQL and Dataframe APIs, excellent performance, built-in support for CSV, Parquet, JSON, and Avro, plus extensive customization. DataFusion is great for building projects such as domain specific query engines, new database platforms and data pipelines, query languages and more.

```rust,editable
{{#include ../../../crates/other/examples/data_processing/datafusion.rs:example}}
```

## `databend` {#databend}

`databend`

Modern alternative to Snowflake. Cost-effective and simple for massive-scale analytics. [datafuselabs databend][databend~github]↗{{hi:Analytics}}

## `rerun` {#rerun}

[![rerun~website][c~rerun~website~badge]][c~rerun~website] [![rerun][c~rerun~docs~badge]][c~rerun~docs] [![rerun~crates.io][c~rerun~crates.io~badge]][c~rerun~crates.io] [![rerun~github][c~rerun~github~badge]][c~rerun~github] [![rerun~lib.rs][c~rerun~lib.rs~badge]][c~rerun~lib.rs]{{hi:rerun}}{{hi:Mesh}}{{hi:Plotting}}{{hi:Point-cloud}}{{hi:Robotics}}{{hi:Visualization}} [![cat~visualization][cat~visualization~badge]][cat~visualization]{{hi:Visualization}} [![cat~computer-vision][cat~computer-vision~badge]][cat~computer-vision]{{hi:Computer vision}}

[`rerun`][c~rerun~github]↗ visualizes streams of multi-modal data. (see also [[science_robotics | Robotics]]).

Log images, point clouds, etc, and visualize them effortlessly.

## Other Frameworks {#skip}

- Big [Data Processing][p~data-processing] for the AI Era: [LakeSail][lakesail~website]↗.

## References {#references}

- Rust Data Engineering course By Alfredo Deza et al., O'Reilly{{hi:Data engineering}}.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data_engineering: organize / edit; [`ballista`][c~ballista~docs]↗{{hi:ballista}}; [`spice.ai`][spice.ai~website]](https://github.com/john-cd/rust_howto/issues/589)

[jsonschema~crates.io]: https://crates.io/crates/jsonschema
[jsonpath~crates.io]: https://crates.io/crates/jsonpath-rust
[datawithrust~website]: https://datawithrust.com
[plotters~docs]: https://docs.rs/plotters
[polars~website]: https://docs.pola.rs
[ndarray~github]: https://github.com/rust-ndarray/ndarray
</div>
