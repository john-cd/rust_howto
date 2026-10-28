<!-- TODO Reviewed ??-??-2026 -->

# Visualization

{{#include visualization.incl.md}}

## Plot a Chart to an Image or Vector File with `plotters` {#plotters}

[![plotters~website][c~plotters~website~badge]][c~plotters~website] [![plotters][c~plotters~docs~badge]][c~plotters~docs] [![plotters~crates.io][c~plotters~crates.io~badge]][c~plotters~crates.io] [![plotters~repo][c~plotters~repo~badge]][c~plotters~repo] [![plotters~lib.rs][c~plotters~lib.rs~badge]][c~plotters~lib.rs]{{hi:plotters}}{{hi:Drawing}}{{hi:Plotting}}{{hi:Visualization}}{{hi:Webassembly}} [![cat~visualization][cat~visualization~badge]][cat~visualization]{{hi:Visualization}}

[`plotters`][c~plotters~docs]↗{{hi:plotters}} is a hardware-accelerated drawing library focused on data plotting for native applications and WebAssembly. It supports multiple rendering backends, including bitmap image formats (PNG, BMP), vector graphics (SVG), and canvas/HTML5 backends.

The following example constructs a Cartesian 2D chart and renders a line series into an in-memory SVG string buffer using `plotters::backend::SVGBackend`:

```rust,editable
{{#include ../../../crates/cats/visualization/examples/visualization/plotters.rs:example}}
```

## Interactive Charts in the Browser {#browser-charts}

Interactive web charts render data inside WebAssembly applications or export interactive HTML pages powered by JavaScript visualization engines.

### Interactive HTML Plots with `plotly` {#graph-data}

[![plotly][c~plotly~docs~badge]][c~plotly~docs]{{hi:plotly}} [![plotly~crates.io][c~plotly~crates.io~badge]][c~plotly~crates.io] [![plotly~repo][c~plotly~repo~badge]][c~plotly~repo] [![plotly~lib.rs][c~plotly~lib.rs~badge]][c~plotly~lib.rs]{{hi:plotly}}{{hi:Visualization}}

[`plotly`][c~plotly~docs]↗{{hi:plotly}} brings the capabilities of [Plotly.js][plotly.js~website]↗ to Rust, providing declarative data structures for building interactive scatter plots, bar charts, and 3D surfaces that export directly to HTML documents.

```rust,editable
{{#include ../../../crates/cats/visualization/examples/visualization/plotly.rs:example}}
```

### Declarative ECharts with `charming` {#charming}

[![charming][c~charming~docs~badge]][c~charming~docs]{{hi:charming}} [![charming~crates.io][c~charming~crates.io~badge]][c~charming~crates.io] [![charming~repo][c~charming~repo~badge]][c~charming~repo] [![charming~lib.rs][c~charming~lib.rs~badge]][c~charming~lib.rs]{{hi:charming}}{{hi:Visualization}}

[`charming`][c~charming~docs]↗{{hi:charming}} is a Rust visualization library built on Apache ECharts. It provides a declarative Rust API to build interactive charts and can render to multiple backends, including HTML documents, SVG, image buffers, and WebAssembly canvas targets.

### WebAssembly Canvas Plotting with `plotters` {#plotters-wasm}

For WebAssembly applications, `plotters` can render directly to an HTML `<canvas>` element using the `CanvasBackend`. See the [`plotters-wasm-demo`][plotters-wasm-demo~repo]↗ repository for complete examples of embedding live, reactive charts into WebAssembly frontend applications.

## Plots in a Desktop GUI with `egui_plot` {#egui-plot}

[![egui_plot][c~egui_plot~docs~badge]][c~egui_plot~docs]{{hi:egui_plot}} [![egui_plot~crates.io][c~egui_plot~crates.io~badge]][c~egui_plot~crates.io] [![egui_plot~repo][c~egui_plot~repo~badge]][c~egui_plot~repo] [![egui_plot~lib.rs][c~egui_plot~lib.rs~badge]][c~egui_plot~lib.rs]{{hi:egui_plot}}{{hi:Plot}}{{hi:Chart}}

[`egui_plot`][c~egui_plot~docs]↗{{hi:egui_plot}} provides immediate-mode 2D plotting widgets for [`egui`][c~egui~docs]↗ desktop applications. It supports panning, zooming, hovering, line plots, scatter plots, bar charts, and custom annotations directly within native GUI windows. See the [Immediate-mode GUI `egui` add-ons][p~egui-add-ons] chapter for examples of embedding interactive plots in desktop applications.

## Related Topics {#related-topics .skip}

- [[graphics | Graphics]].
- [[gui | GUI]].
- [[rendering | Rendering]].
- [[wasm | WASM]].
- [[development_tools | Development Tools]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
