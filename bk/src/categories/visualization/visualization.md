<!-- TODO Reviewed ??-??-2026. -->

# Plot Static and Interactive Charts

## Write an SVG line chart with Plotters {#plotters}

[`plotters`][c~plotters~docs]↗{{hi:plotters}} can draw to SVG, bitmap, or browser canvas backends. This recipe plots `y = x²` into an SVG string and writes the document to standard output. The SVG backend runs without a display or GPU.

From `bk/crates`, run:

```console
cargo run --quiet --locked -p visualization --example visualization -- plotters > chart.svg
```

Open `chart.svg` to see the labeled axes and the curve. Call `present()` before releasing the drawing area so the SVG document is finalized.

```rust,ignore,noplayground
{{#include ../../../crates/cats/visualization/examples/visualization/plotters.rs:example}}
```

## Export an interactive scatter plot with Plotly {#graph-data}

[`plotly`][c~plotly~docs]↗{{hi:plotly}} turns a Rust chart description into HTML using [Plotly.js][plotly.js~website]. This recipe places one marker per sample and labels both axes.

From `bk/crates`, run:

```console
cargo run --locked -p visualization --example visualization -- plotly
```

Open `temp/plot.html` relative to that working directory. The command writes the HTML locally. The browser needs access to Plotly’s CDN to load the default JavaScript bundle. Once the chart loads, hover over a marker to inspect its values or drag across the plot to zoom.

```rust,ignore,noplayground
{{#include ../../../crates/cats/visualization/examples/visualization/plotly.rs:example}}
```

## Update a Chartistry line chart in the browser {#leptos-chartistry}

[`leptos-chartistry`][c~leptos-chartistry~docs] draws a line from a Leptos signal. The app mounts into the document body and starts with four response-time samples. **Add sample** appends a fifth point, and **Remove sample** restores the original data. Hover over the chart to inspect a sample.

Install [Trunk][trunk~releases] and the WebAssembly target. From `bk/crates/cats/visualization`, run:

```console
rustup target add wasm32-unknown-unknown
trunk serve examples/chartistry/index.html --locked
```

Open the local URL printed by Trunk. The HTML entry point selects the `chartistry` binary and enables its optional Cargo feature. Trunk builds the WebAssembly, generates the JavaScript loader, and serves both files.

```rust,ignore,noplayground
{{#include ../../../crates/cats/visualization/examples/chartistry/main.rs:example}}
```

## Browser and desktop alternatives {#application-charts}

### ECharts with Charming {#charming}

[`charming`][c~charming~docs]↗{{hi:charming}} builds Apache ECharts configurations in Rust. Consult its documentation for the rendering features and dependencies for your target.

### Plotters on browser canvas {#plotters-wasm}

The [Plotters WebAssembly demo][plotters-wasm-demo~repo] shows how to attach its canvas backend to an HTML canvas. Use this route when drawing within a WebAssembly application.

### Desktop plots with egui_plot {#egui-plot}

Use [`eframe`][c~eframe~docs]↗{{hi:eframe}} to run a standalone egui application. [`egui_plot`][c~egui_plot~docs]↗{{hi:egui_plot}} adds plots with pan, zoom, and hover to an egui application. Check the [egui_plot project][c~egui_plot~repo] for examples and compatible egui versions.

## Add Samples to a Live Desktop Plot {#live-plot}

This `eframe` app keeps samples in application state. Click **Add sample** to generate the next point and redraw the `egui_plot` line. The sample model is testable without opening a window. See [GUI](../gui/egui_ecosystem.md#egui-plot) for additional plot widgets.

From `bk/`, run `just cargo run --manifest-path platform/Cargo.toml -p gui --example live_plot` in a desktop session.

```rust,ignore,noplayground
{{#include ../../../platform/cats/gui/examples/live_plot.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
