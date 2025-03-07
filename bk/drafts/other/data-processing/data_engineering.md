# Data Engineering

{{#include data_engineering.incl.md}}

## Develop data analytics applications that process columnar data with Arrow {#arrow}

[![arrow][c-arrow-badge]][c-arrow]{{hi:arrow}}
[![arrow-crates.io][c-arrow-crates.io-badge]][c-arrow-crates.io]
[![arrow-github][c-arrow-github-badge]][c-arrow-github]
[![arrow-lib.rs][c-arrow-lib.rs-badge]][c-arrow-lib.rs]

[`arrow`][c-arrow]⮳{{hi:arrow}} is the official Rust implementation of `Apache Arrow`{{hi:In-memory}}

```rust,editable
{{#include ../../../crates/other/tests/data_processing/arrow.rs:example}}
```

## Query in-memory data with `datafusion` {#datafusion}

[![datafusion][c-datafusion-badge]][c-datafusion]{{hi:datafusion}}
[![datafusion-crates.io][c-datafusion-crates.io-badge]][c-datafusion-crates.io]
[![datafusion-github][c-datafusion-github-badge]][c-datafusion-github]
[![datafusion-lib.rs][c-datafusion-lib.rs-badge]][c-datafusion-lib.rs]

[![datafusion][c-datafusion-badge]][c-datafusion]{{hi:datafusion}} is the `Apache Arrow DataFusion` {{i:SQL}} Query Engine.

{{i:Apache DataFusion}} is an in-memory query engine that uses Apache Arrow as the memory model

[`datafusion`][c-datafusion]⮳{{hi:datafusion}} offers SQL and Dataframe APIs, excellent performance, built-in support for CSV, Parquet, JSON, and Avro, plus extensive customization. DataFusion is great for building projects such as domain specific query engines, new database platforms and data pipelines, query languages and more.

```rust,editable
{{#include ../../../crates/other/tests/data_processing/datafusion.rs:example}}
```

## `databend` {#databend}

`databend`

𝗗𝗮𝘁𝗮, 𝗔𝗻𝗮𝗹𝘆𝘁𝗶𝗰𝘀 & 𝗔𝗜. Modern alternative to Snowflake. Cost-effective and simple for massive-scale analytics. [datafuselabs databend][databend-github]⮳{{hi:Analytics}}

## Other frameworks

- Big [Data Processing][p-data-processing] for the AI Era: [LakeSail][lakesail-website]⮳.
- [`rerun`][rerun]⮳: visualize streams of multi-modal data. Free, fast, easy to use, and simple to integrate. Built in Rust. (see also [[science_robotics | Robotics]]).

## References

- Rust Data Engineering course By Alfredo Deza et al., O'Reilly{{hi:Data engineering}}.

[lakesail-website]: https://lakesail.com
[rerun]: https://github.com/rerun-io/rerun
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[data_engineering: organize / edit](https://github.com/john-cd/rust_howto/issues/589)
cover `ballista` `spice.ai`
</div>
