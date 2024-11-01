# Data and ETL

{{#include index.incl.md}}

## Polars

[![polars][c-polars-badge]][c-polars]{{hi:polars}}  [![polars-book][c-polars-book-badge]][c-polars-book]  [![polars-github][c-polars-github-badge]][c-polars-github]{{hi:Dataframes}}

- [docs.pola.rs][c-polars-docs]⮳

## Arrow

[![arrow][c-arrow-badge]][c-arrow]{{hi:arrow}}
[![arrow-crates.io][c-arrow-crates.io-badge]][c-arrow-crates.io]
[![arrow-github][c-arrow-github-badge]][c-arrow-github]
[![arrow-lib.rs][c-arrow-lib.rs-badge]][c-arrow-lib.rs]

`arrow` is the official Rust implementation of `Apache Arrow`{{hi:In-memory}}

## Datafusion

[![datafusion][c-datafusion-badge]][c-datafusion]{{hi:datafusion}}
[![datafusion-crates.io][c-datafusion-crates.io-badge]][c-datafusion-crates.io]
[![datafusion-github][c-datafusion-github-badge]][c-datafusion-github]
[![datafusion-lib.rs][c-datafusion-lib.rs-badge]][c-datafusion-lib.rs]

[![datafusion][c-datafusion-badge]][c-datafusion]{{hi:datafusion}} is the `Apache Arrow DataFusion` {{i:SQL}} Query Engine.

## Databend

𝗗𝗮𝘁𝗮, 𝗔𝗻𝗮𝗹𝘆𝘁𝗶𝗰𝘀 & 𝗔𝗜. Modern alternative to Snowflake. Cost-effective and simple for massive-scale analytics. [datafuselabs databend][databend-github]⮳{{hi:Analytics}}

## Plotly.rs

`Plotly.rs` is a plotting library powered by [Plotly.js][plotly.js]⮳. The aim is to bring over to Rust all the functionality that `Python` users have come to rely on; with the added benefit of type safety and speed.{{hi:Visualization}}

- [plotly.rs][c-plotly-github]{{hi:plotly.rs}}⮳

## CSV

A fast {{i:CSV}} command line toolkit written in Rust. [xsv][c-xsv-github]{{hi:xsv}}⮳

`xsv` is a command line program for indexing, slicing, analyzing, splitting and joining CSV files.

[![csv][c-csv-badge]][c-csv]{{hi:csv}}

## See also

Rust Data Engineering course By Alfredo Deza et al., O'Reilly{{hi:Data engineering}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: organize / edit

## DataFrames

### polars

Similar to the Pandas library in Python but in pure Rust. Uses the {{i:Apache Arrow}} Columnar Format as the memory model.

### datafusion

{{i:Apache DataFusion}} is an in-memory query engine that uses Apache Arrow as the memory model

</div>
