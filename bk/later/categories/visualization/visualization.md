# Visualization

{{#include visualization.incl.md}}

## `plotters` {#plotters}

[![plotters-website][c-plotters-website-badge]][c-plotters-website] [![plotters][c-plotters-badge]][c-plotters] [![plotters-crates.io][c-plotters-crates.io-badge]][c-plotters-crates.io] [![plotters-github][c-plotters-github-badge]][c-plotters-github] [![plotters-lib.rs][c-plotters-lib.rs-badge]][c-plotters-lib.rs]{{hi:plotters}}{{hi:Drawing}}{{hi:Plotting}}{{hi:Visualization}}{{hi:Webassembly}} [![cat-visualization][cat-visualization-badge]][cat-visualization]{{hi:Visualization}} [![cat-wasm][cat-wasm-badge]][cat-wasm]{{hi:WebAssembly}}

A Rust drawing library focus on data plotting for both [WASM][p-wasm] and native applications.

{{#example plotters}}

## Plot and Graph Data with `plotly` {#graph-data}

[![plotly][c-plotly-badge]][c-plotly]{{hi:plotly}}
[![plotly-crates.io][c-plotly-crates.io-badge]][c-plotly-crates.io]
[![plotly-github][c-plotly-github-badge]][c-plotly-github]
[![plotly-lib.rs][c-plotly-lib.rs-badge]][c-plotly-lib.rs]

[`plotly.rs`][c-plotly]⮳{{hi:Plotly.rs}} is a plotting library powered by [Plotly.js][plotly.js]⮳. The aim is to bring over to Rust all the functionality that [`Python`][python]⮳{{hi:Python}} users have come to rely on; with the added benefit of type safety and speed.{{hi:Visualization}}

```rust,editable
{{#include ../../../crates/cats/visualization/examples/visualization/plotly.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[visualization: write](https://github.com/john-cd/rust_howto/issues/494)
cover `charming` `egui_plot` https://github.com/plotters-rs/plotters-wasm-demo
</div>
