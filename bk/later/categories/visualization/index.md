# Visualization

[![cat-visualization][cat-visualization-badge]][cat-visualization]{{hi:Visualization}}

View, plot or graph data.

For simple 2D Plots, use [`plotters`][c-plotters]⮳{{hi:plotters}}. For advanced 2D/3D Plots, use [`plotters`][c-plotters]⮳{{hi:plotters}} with custom drawing or extensions, [`glium`][c-glium]⮳{{hi:glium}}, or [`wgpu`][c-wgpu]⮳{{hi:wgpu}}. For graph visualization, combine [`petgraph`][c-petgraph]⮳{{hi:petgraph}} with graph rendering tools.

The Rust visualization ecosystem offers a variety of options, from general-purpose plotting libraries to more specialized tools.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| 2D Plotting | [`plotters`][c-plotters]⮳{{hi:plotters}} | [`plotters`][c-plotters]⮳{{hi:plotters}} is a versatile plotting library. |
| 3D Plotting | [`plotters`][c-plotters]⮳{{hi:plotters}} (with extensions), [`glium`][c-glium]⮳{{hi:glium}} (graphics library - used to build upon), [`wgpu`][c-wgpu]⮳{{hi:wgpu}} (WebGPU bindings - useful for cross-platform 3D) | [`plotters`][c-plotters]⮳{{hi:plotters}} can be used for basic 3D plots, but for more advanced 3D visualization, you'll likely need to work with a graphics library like [`glium`][c-glium]⮳{{hi:glium}} or [`wgpu`][c-wgpu]⮳{{hi:wgpu}}. |
| Graph Visualization | [`petgraph`][c-petgraph]⮳{{hi:petgraph}}, [`dot`][c-dot]⮳{{hi:dot}} (for graph description language) | [`petgraph`][c-petgraph]⮳{{hi:petgraph}} is a graph library that can be used with visualization tools. [`dot`][c-dot]⮳{{hi:dot}} can be used to generate graph descriptions in the DOT language, which can then be rendered by `Graphviz`. |

{{#include visualization.incl.md}}

## Web Visualization

- Rust (compiled to WASM) can use the Canvas API via `web-sys` to draw 2D graphics directly in the browser. This is suitable for simpler visualizations and interactive graphics.
- For 3D visualizations and high-performance graphics, Rust can leverage WebGL through `web-sys`.
- Libraries like [`wgpu`][c-wgpu]⮳{{hi:wgpu}} and `three-d` simplify WebGL development in Rust.
  - [`wgpu`][c-wgpu]⮳{{hi:wgpu}} is a WebGPU implementation that allows you to write portable graphics code that can run on WebGL, Vulkan, Metal, and DirectX 12. It provides a efficient way to perform GPU accelerated computations and rendering. This is important for complex 3D rendering and large datasets.
  - `three-d` is a Rust 3D graphics library built on top of [`wgpu`][c-wgpu]⮳{{hi:wgpu}}. It simplifies 3D rendering and provides a high-level API for creating interactive 3D visualizations.

[`yew`][c-yew]⮳{{hi:yew}} (a front-end framework) and [`egui`][c-egui]⮳{{hi:egui}} (an immediate mode GUI) may also be useful.

See [[gui | GUI]] and [[wasm | WASM]].

## In Summary

- Choose the appropriate library based on whether you need 2D or 3D visualization.
- Consider whether you need web-based or desktop visualization. For web visualization, using JavaScript charting libraries (like D3.js, Chart.js, etc.) and interacting via WebAssembly is common.
- If you need interactive plots, GUI frameworks like [`iced`][c-iced]⮳{{hi:iced}} or [`egui`][c-egui]⮳{{hi:egui}} might be a good choice.
- For performance-critical visualizations, consider using lower-level graphics libraries like [`glium`][c-glium]⮳{{hi:glium}} or [`wgpu`][c-wgpu]⮳{{hi:wgpu}} or perhaps a game engine like [`bevy`][c-bevy]⮳{{hi:bevy}}.

## Related Topics

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| Data Exploration & Analysis | [`polars`][c-polars]⮳{{hi:polars}} , [`dataframe`][c-dataframe]⮳{{hi:dataframe}} | These crates are useful for data manipulation and analysis, often a precursor to visualization. |
| Image Processing | [`image`][c-image]⮳{{hi:image}} , [`imageproc`][c-imageproc]⮳{{hi:imageproc}} | These crates are used for image processing and manipulation, which can be part of a data visualization pipeline. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/969)
</div>
