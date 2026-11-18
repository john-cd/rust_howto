<!-- TODO Reviewed ??-??-2026. -->

# Plot Static and Interactive Charts

## Write an SVG line chart with Plotters {#plotters}

[`plotters`][c~plotters~docs]↗{{hi:plotters}} can draw to SVG, bitmap, or browser canvas backends. This recipe plots `y = x²` into an SVG string and writes the document to standard output. The SVG backend runs without a display or GPU.

From `later/crates`, run:

```console
cargo run --quiet --locked -p visualization --example visualization -- plotters > chart.svg
```

Open `chart.svg` to see the labeled axes and the curve. Call `present()` before releasing the drawing area so the SVG document is finalized.

```rust,ignore,noplayground
{{#include ../../../crates/cats/visualization/examples/visualization/plotters.rs:example}}
```

## Export an interactive scatter plot with Plotly {#graph-data}

[`plotly`][c~plotly~docs]↗{{hi:plotly}} turns a Rust chart description into HTML using [Plotly.js][plotly.js~website]. This recipe places one marker per sample and labels both axes.

From `later/crates`, run:

```console
cargo run --locked -p visualization --example visualization -- plotly
```

Open `temp/plot.html` relative to that working directory. The command writes the HTML locally. The browser needs access to Plotly’s CDN to load the default JavaScript bundle. Once the chart loads, hover over a marker to inspect its values or drag across the plot to zoom.

```rust,ignore,noplayground
{{#include ../../../crates/cats/visualization/examples/visualization/plotly.rs:example}}
```

## Browser and desktop alternatives {#application-charts}

### Chartistry in a Leptos application {#leptos-chartistry}

[`leptos-chartistry`][c~leptos-chartistry~docs]↗{{hi:leptos-chartistry}} provides an SVG chart component for a Leptos application. This component uses fixed response-time samples. The optional `chartistry` Cargo feature checks that this component compiles. From `later/crates`, run `cargo check --locked -p visualization --features chartistry --example visualization`. To see a mounted chart and tooltips, use the upstream examples linked from the crate documentation.

```rust,ignore,noplayground
{{#include ../../../crates/cats/visualization/examples/visualization/leptos_chartistry.rs:example}}
```

### ECharts with Charming {#charming}

[`charming`][c~charming~docs]↗{{hi:charming}} builds Apache ECharts configurations in Rust. Consult its documentation for the rendering features and dependencies for your target.

### Plotters on browser canvas {#plotters-wasm}

The [Plotters WebAssembly demo][plotters-wasm-demo~repo] shows how to attach its canvas backend to an HTML canvas. Use this route when drawing within a WebAssembly application.

### Desktop plots with egui_plot {#egui-plot}

[`egui_plot`][c~egui_plot~docs]↗{{hi:egui_plot}} adds plots with pan, zoom, and hover to an egui application. See [egui add-ons][p~egui-add-ons] for library and host-version compatibility.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
