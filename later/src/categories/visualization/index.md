<!-- TODO Reviewed ??-??-2026 -->

# Visualization

[![cat~visualization][cat~visualization~badge]][cat~visualization]{{hi:Visualization}}

View, plot or graph data.

| Topic                      | Rust Crates                                                                                                                           | Notes                                                                                                                                                        |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Charts and plots           | [`plotters`][c~plotters~docs]↗{{hi:plotters}}, [`charming`][c~charming~docs]↗{{hi:charming}}, [`plotly`][c~plotly~docs]↗{{hi:plotly}} | `plotters` draws static 2D charts to bitmaps, SVG or a canvas; `charming` and `plotly` wrap the ECharts and Plotly.js libraries for interactive HTML output. |
| Interactive plots in a GUI | [`egui_plot`][c~egui_plot~docs]↗{{hi:egui_plot}}                                                                                      | Live plots inside an [`egui`][c~egui~docs]↗{{hi:egui}} application. See [GUI](../gui/index.md).                                                              |
| 3D and custom rendering    | [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`three-d`][c~three-d~crates.io]↗{{hi:three-d}}, [`glium`][c~glium~docs]↗{{hi:glium}}              | Use a graphics library when `plotters` is not enough.                                                                                                        |
| Graph (network) diagrams   | [`petgraph`][c~petgraph~docs]↗{{hi:petgraph}}, [`layout-rs`][c~layout-rs~docs]↗{{hi:layout-rs}}                                       | `petgraph::dot` writes DOT for [Graphviz][graphviz~website]↗{{hi:Graphviz}}; `layout-rs` renders DOT-style graphs to SVG in pure Rust.                       |
| Data exploration           | [`rerun`][c~rerun~docs]↗{{hi:rerun}}, [`polars`][c~polars~docs]↗{{hi:polars}}                                                         | `rerun` logs and visualizes time-series, images and 3D data; `polars` prepares the data.                                                                     |

{{#include visualization.incl.md}}

## Web Visualization

- Rust (compiled to WASM) can use the Canvas API via [`web-sys`][c~web-sys~crates.io]↗{{hi:web-sys}} to draw 2D graphics directly in the browser. This is suitable for simpler visualizations and interactive graphics.
- For 3D visualizations and high-performance graphics, Rust can leverage WebGL through [`web-sys`][c~web-sys~crates.io]↗{{hi:web-sys}}.
- Libraries like [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} and [`three-d`][c~three-d~crates.io]↗{{hi:three-d}} simplify WebGL development in Rust.
  - [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} is a WebGPU implementation that allows you to write portable graphics code that can run on WebGL, Vulkan, Metal, and DirectX 12. It provides a efficient way to perform GPU accelerated computations and rendering. This is important for complex 3D rendering and large datasets.
  - [`three-d`][c~three-d~crates.io]↗{{hi:three-d}} is a 2D/3D renderer using OpenGL, OpenGL ES, and WebGL. It offers a higher-level API for interactive visualizations.

[`yew`][c~yew~docs]↗{{hi:yew}} (a front-end framework) and [`egui`][c~egui~docs]↗{{hi:egui}} (an immediate mode GUI) may also be useful.

See [[gui | GUI]] and [[wasm | WASM]].

Choose by output: static image or SVG (`plotters`), interactive HTML (`charming`, `plotly`), desktop GUI (`egui_plot`, [`iced`][c~iced~docs]↗{{hi:iced}}), or the browser (the Web Visualization section above).

## Related Topics

| Topic            | Rust Crates (Examples)                                                                  | Notes                                                                                                            |
| ---------------- | --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Image Processing | [`image`][c~image~docs]↗{{hi:image}} , [`imageproc`][c~imageproc~docs]↗{{hi:imageproc}} | These crates are used for image processing and manipulation, which can be part of a data visualization pipeline. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
