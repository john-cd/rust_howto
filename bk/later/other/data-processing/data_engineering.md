# Data Engineering

{{#include data_engineering.incl.md}}

## Process Columnar Data with `arrow` {#arrow}

[![arrow][c~arrow~docs~badge]][c~arrow~docs]{{hi:arrow}}
[![arrow~crates.io][c~arrow~crates.io~badge]][c~arrow~crates.io]
[![arrow~repo][c~arrow~repo~badge]][c~arrow~repo]
[![arrow~lib.rs][c~arrow~lib.rs~badge]][c~arrow~lib.rs]

[`arrow`][c~arrow~docs]↗{{hi:arrow}} is the official Rust implementation of `Apache Arrow`{{hi:In-memory}}

```rust,editable
{{#include ../../../crates/other/examples/data_processing/arrow.rs:example}}
```

## Query In-memory Data with `datafusion` {#datafusion}

[![datafusion][c~datafusion~docs~badge]][c~datafusion~docs]{{hi:datafusion}}
[![datafusion~crates.io][c~datafusion~crates.io~badge]][c~datafusion~crates.io]
[![datafusion~repo][c~datafusion~repo~badge]][c~datafusion~repo]
[![datafusion~lib.rs][c~datafusion~lib.rs~badge]][c~datafusion~lib.rs]

[![datafusion][c~datafusion~docs~badge]][c~datafusion~docs]↗{{hi:datafusion}} is the `Apache Arrow DataFusion` {{i:SQL}} Query Engine.

{{i:Apache DataFusion}} is an in-memory query engine that uses Apache Arrow as the memory model

[`datafusion`][c~datafusion~docs]↗{{hi:datafusion}} offers SQL and Dataframe APIs, excellent performance, built-in support for CSV, Parquet, JSON, and Avro, plus extensive customization. DataFusion is great for building projects such as domain specific query engines, new database platforms and data pipelines, query languages and more.

```rust,editable
{{#include ../../../crates/other/examples/data_processing/datafusion.rs:example}}
```

## `databend` {#databend}

`databend`

Modern alternative to Snowflake. Cost-effective and simple for massive-scale analytics. [datafuselabs databend][databend~repo]↗{{hi:Analytics}}

## `rerun` {#rerun}

[![rerun~website][c~rerun~website~badge]][c~rerun~website] [![rerun][c~rerun~docs~badge]][c~rerun~docs] [![rerun~crates.io][c~rerun~crates.io~badge]][c~rerun~crates.io] [![rerun~repo][c~rerun~repo~badge]][c~rerun~repo] [![rerun~lib.rs][c~rerun~lib.rs~badge]][c~rerun~lib.rs]{{hi:rerun}}{{hi:Mesh}}{{hi:Plotting}}{{hi:Point-cloud}}{{hi:Robotics}}{{hi:Visualization}} [![cat~visualization][cat~visualization~badge]][cat~visualization]{{hi:Visualization}} [![cat~computer-vision][cat~computer-vision~badge]][cat~computer-vision]{{hi:Computer vision}}

[`rerun`][c~rerun~repo]↗ visualizes streams of multi-modal data. (see also [[science_robotics | Robotics]]).

Log images, point clouds, etc, and visualize them effortlessly.

## Other Frameworks {#other-frameworks}

- Big [Data Processing][p~data-processing] for the AI Era: [LakeSail][lakesail~website]↗.

## References {#references}

- Rust Data Engineering course By Alfredo Deza et al., O'Reilly{{hi:Data engineering}}.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data_engineering: organize / edit](https://github.com/john-cd/rust_howto/issues/589)

[![jsonschema][c~jsonschema~docs~badge]][c~jsonschema~docs] [![jsonschema~crates.io][c~jsonschema~crates.io~badge]][c~jsonschema~crates.io] [![jsonschema~repo][c~jsonschema~repo~badge]][c~jsonschema~repo] [![jsonschema~lib.rs][c~jsonschema~lib.rs~badge]][c~jsonschema~lib.rs]{{hi:jsonschema}}{{hi:Validation}}{{hi:jsonschema}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

[![jsonpath][c~jsonpath~docs~badge]][c~jsonpath~docs] [![jsonpath~crates.io][c~jsonpath~crates.io~badge]][c~jsonpath~crates.io] [![jsonpath~repo][c~jsonpath~repo~badge]][c~jsonpath~repo] [![jsonpath~lib.rs][c~jsonpath~lib.rs~badge]][c~jsonpath~lib.rs]{{hi:jsonpath}}{{hi:JSON}}{{hi:Query}}{{hi:XPath}}{{hi:Javascript}}{{hi:jsonpath}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

JSONPath for Rust

---

[![jsonpath-rust][c~jsonpath-rust~docs~badge]][c~jsonpath-rust~docs] [![jsonpath-rust~crates.io][c~jsonpath-rust~crates.io~badge]][c~jsonpath-rust~crates.io] [![jsonpath-rust~repo][c~jsonpath-rust~repo~badge]][c~jsonpath-rust~repo] [![jsonpath-rust~lib.rs][c~jsonpath-rust~lib.rs~badge]][c~jsonpath-rust~lib.rs]{{hi:jsonpath-rust}}{{hi:JSON}}{{hi:XPath}}{{hi:jsonpath}}{{hi:json-path}}{{hi:jsonpath-rust}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}} [![cat~text-processing][cat~text-processing~badge]][cat~text-processing]{{hi:Text processing}}

The library provides the basic functionality to find the set of the data according to the filtering query.

---

- [`ballista`][c~ballista~docs]↗.{{hi:ballista}}
- [`spice.ai`][spice.ai~website]↗.{{hi:spice.ai}}
- [`datawithrust.com`][datawithrust~website]↗.
- [`plotters`][c~plotters~docs]↗.
- [`polars` additional docs][c~polars~more-docs~website]↗.
- [`ndarray`][c~ndarray~repo]↗.

</div>
