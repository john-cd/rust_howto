<!-- TODO Reviewed ??-??-2026 -->

# Visualization

{{#include visualization.incl.md}}

## Plot a Chart to an Image or Vector File with `plotters` {#plotters}

[![plotters~website][c~plotters~website~badge]][c~plotters~website] {{#badges plotters}} {{hi:Drawing}} {{hi:Plotting}} {{hi:Visualization}} {{hi:Webassembly}} {{#categories visualization=Visualization}}

[`plotters`][c~plotters~docs]↗{{hi:plotters}} is a drawing library focused on data plotting for native applications and WebAssembly. It supports bitmap images, SVG, and browser canvas backends. The SVG example below builds its output in memory and needs no GPU, display, or output file.

The following example constructs a Cartesian 2D chart and renders a line series into an in-memory SVG string buffer using `plotters::backend::SVGBackend`:

```rust,editable
{{#include ../../../crates/cats/visualization/examples/visualization/plotters.rs:example}}
```

## Interactive Charts in the Browser {#browser-charts}

Interactive web charts render data inside WebAssembly applications or export interactive HTML pages powered by JavaScript visualization engines.

### Interactive HTML Plots with `plotly` {#graph-data}

{{#badges plotly}}{{hi:plotly}}{{hi:Visualization}}

[`plotly`][c~plotly~docs]↗{{hi:plotly}} brings the capabilities of [Plotly.js][plotly.js~website]↗ to Rust, providing declarative data structures for building interactive scatter plots, bar charts, and 3D surfaces that export directly to HTML documents.

```rust,editable
{{#include ../../../crates/cats/visualization/examples/visualization/plotly.rs:example}}
```

### Reactive Leptos Charts with `leptos-chartistry` {#leptos-chartistry}

[`leptos-chartistry`][c~leptos-chartistry~docs]↗{{hi:leptos-chartistry}} provides an SVG chart component for Leptos applications. This example defines a reactive line chart component; mount it from a Leptos application to render it in a browser.

```rust,editable
{{#include ../../../crates/cats/visualization/examples/visualization/leptos_chartistry.rs:example}}
```

### Declarative ECharts with `charming` {#charming}

{{#badges charming}}{{hi:charming}}{{hi:Visualization}}

[`charming`][c~charming~docs]↗{{hi:charming}} is a Rust visualization library built on Apache ECharts. It provides a declarative Rust API to build interactive charts and can render to multiple backends, including HTML documents, SVG, image buffers, and WebAssembly canvas targets.

### WebAssembly Canvas Plotting with `plotters` {#plotters-wasm}

For WebAssembly applications, `plotters-canvas` supplies `CanvasBackend` to draw onto an HTML `<canvas>` element. See the [`plotters-wasm-demo`][plotters-wasm-demo~repo]↗ repository for examples of embedding charts into WebAssembly frontend applications.

## Plots in a Desktop GUI with `egui_plot` {#egui-plot}

{{#badges egui_plot}}{{hi:egui_plot}}{{hi:Plot}}{{hi:Chart}}

[`egui_plot`][c~egui_plot~docs]↗{{hi:egui_plot}} provides immediate-mode 2D plotting widgets for [`egui`][c~egui~docs]↗ desktop applications. It supports panning, zooming, hovering, line plots, scatter plots, bar charts, and custom annotations directly within native GUI windows. See the [Immediate-mode GUI `egui` add-ons][p~egui-add-ons] chapter for examples of embedding interactive plots in desktop applications.

## Related Topics {#related-topics .skip}

- [[graphics | Graphics]].
- [[gui | GUI]].
- [[rendering | Rendering]].
- [[wasm | WASM]].
- [[development_tools | Development Tools]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
