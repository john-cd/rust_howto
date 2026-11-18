<!-- TODO Reviewed ??-??-2026. -->

# Visualization

[![cat~visualization][cat~visualization~badge]][cat~visualization]{{hi:Visualization}}

Turn structured data into a visual output for inspection, reporting, or interaction. Choose the output format and host first; many visualization libraries do not provide an application shell or data-processing pipeline.

| Topic | Rust Crates | Notes |
| --- | --- | --- |
| Charts and plots | [`plotters`][c~plotters~docs]↗{{hi:plotters}}, [`charming`][c~charming~docs]↗{{hi:charming}}, [`plotly`][c~plotly~docs]↗{{hi:plotly}} | `plotters` draws static 2D charts to bitmaps, SVG or a canvas; `charming` and `plotly` wrap the ECharts and Plotly.js libraries for interactive HTML output. |
| Exact text output | [tabled](https://docs.rs/tabled/) | Show individual values and units in a [terminal table](terminal_tables.md). |
| Interactive plots in a GUI | [`egui_plot`][c~egui_plot~docs]↗{{hi:egui_plot}} | Live plots inside an [`egui`][c~egui~docs]↗{{hi:egui}} application. See [GUI](../gui/index.md). |
| 3D and custom rendering | [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`three-d`][c~three-d~crates.io]↗{{hi:three-d}}, [`glium`][c~glium~docs]↗{{hi:glium}} | Use a graphics library when `plotters` is not enough. |
| Graph (network) diagrams | [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}}, [`layout-rs`][c~layout-rs~docs]↗{{hi:layout-rs}} | `petgraph::dot` writes DOT for [Graphviz][graphviz~website]↗{{hi:Graphviz}}; `layout-rs` renders DOT-style graphs to SVG in pure Rust. |
| Data exploration | [`rerun`][c~rerun~docs]↗{{hi:rerun}}, [`polars`][c~polars~docs]↗{{hi:polars}} | `rerun` logs and visualizes time-series, images and 3D data; `polars` prepares the data. |

{{#include visualization.incl.md}}

## Web Visualization

- Rust (compiled to WASM) can use the Canvas API via [`web-sys`][c~web-sys~crates.io]↗{{hi:web-sys}} to draw 2D graphics directly in the browser. This is suitable for simpler visualizations and interactive graphics.
- For GPU-backed browser visualizations, [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} can target WebGPU or WebGL 2, depending on browser support and enabled features. WebGL 2 exposes a smaller feature set, so test the required operations on the actual browser backend.
- [`three-d`][c~three-d~crates.io]↗{{hi:three-d}} provides a higher-level renderer for interactive visualizations; check its current browser and backend requirements before choosing it.

[`yew`][c~yew~docs]↗{{hi:yew}} (a front-end framework) and [`egui`][c~egui~docs]↗{{hi:egui}} (an immediate mode GUI) may also be useful.

See [[gui | GUI]] and [[wasm | WASM]].

Choose by output: static image or SVG (`plotters`), interactive HTML (`charming`, `plotly`), desktop GUI (`egui_plot`, [`iced`][c~iced~docs]↗{{hi:iced}}), or the browser (the Web Visualization section above).

## Related Topics

| Topic | Rust Crates (Examples) | Notes |
| --- | --- | --- |
| Image and video data | [`image`][c~image~docs]↗{{hi:image}}, [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}} | Decode and prepare raster data before drawing; see [Images](https://john-cd.com/rust_howto/categories/multimedia_images/index.html) and [[multimedia_video | Multimedia: Video]]. |
| Data preparation | [`polars`][c~polars~docs]↗{{hi:polars}}, [`arrow`][c~arrow~docs]↗{{hi:arrow}} | Filter and aggregate data before selecting a chart or rendering backend; see [[data-processing | Data and ETL]]. |
| Application host | [`egui_plot`][c~egui_plot~docs]↗{{hi:egui_plot}}, [`plotters`][c~plotters~docs]↗{{hi:plotters}}, [`plotly`][c~plotly~docs]↗{{hi:plotly}} | Match the renderer to a desktop GUI, static file, or browser application. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
