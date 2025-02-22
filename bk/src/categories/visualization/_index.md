# Visualization

[![cat-visualization][cat-visualization-badge]][cat-visualization]{{hi:Visualization}}

View, plot or graph data.

{{#include visualization.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 review](https://github.com/john-cd/rust_howto/issues/969)

`bevy`: A data-driven game engine that might be suitable for 3D visualization of aerospace simulations.
`plotters`: For creating plots and charts of simulation data.

---

The Rust visualization ecosystem offers a variety of options, from general-purpose plotting libraries to more specialized tools.

| Topic | Rust Crates (Examples) | Notes |
|---|---|---|
| 2D Plotting | `plotters`, `chartist`, `iced` (with custom drawing), `egui` (immediate mode GUI) | `plotters` is a versatile plotting library. `chartist` is a simpler option. `iced` and `egui` are GUI frameworks that can be used for custom 2D drawing. |
| 3D Plotting | `plotters` (can be extended), `glium` (graphics library - used to build upon), `wgpu` (WebGPU bindings - useful for cross-platform 3D) | `plotters` can be used for basic 3D plots, but for more advanced 3D visualization, you'll likely need to work with a graphics library like `glium` or `wgpu`. |
| Data Visualization Libraries (General) | `iced`, `egui`, `tauri` (for desktop apps), `yew` (for web apps) | These crates provide frameworks for building applications that often include data visualization components. `iced` and `egui` are immediate mode GUIs. [`tauri`][c-tauri]⮳{{hi:tauri}}  and `yew` are for building desktop and web applications, respectively. |
| Charting Libraries | (Often built on top of 2D plotting libraries) | Specialized charting libraries are less common in pure Rust, but many can be constructed using the 2D plotting libraries. |
| Graph Visualization | `petgraph`, `dot` (for graph description language) | [`petgraph`][c-petgraph]⮳{{hi:petgraph}}  is a graph library that can be used with visualization tools. `dot` can be used to generate graph descriptions in the DOT language, which can then be rendered by Graphviz. |
| Image Processing & Visualization | [`image`][cached-docker-images]⮳{{hi:image}} [`image`][cat-multimedia::images]⮳{{hi:image}} [`image`][c-image::DynamicImage::resize]⮳{{hi:image}} [`image`][c-image::ImageBuffer::new]⮳{{hi:image}} [`image`][c-image::ImageBuffer::put_pixel]⮳{{hi:image}} [`image`][c-image::ImageBuffer::save]⮳{{hi:image}} [`image`][c-image::Rgb::from_channels]⮳{{hi:image}} [`image`][c-image]⮳{{hi:image}} , `imageproc` | These crates are used for image processing and manipulation, which can be part of a visualization pipeline. |
| Web Visualization | `yew`, `perseus` (static site generation) | For web-based visualizations, `yew` (a front-end framework) and `perseus` (for static site generation) are useful. Often, you'll use JavaScript charting libraries (like D3.js, Chart.js, etc.) and interact with them from Rust via WebAssembly. |
| Data Exploration & Analysis (Related) | [`polars`][c-polars]⮳{{hi:polars}} , `dataframe` | These crates are useful for data manipulation and analysis, often a precursor to visualization. |
| Scientific Visualization | (Often uses graphics libraries and numerical computation crates) | Scientific visualization often involves custom solutions built using graphics libraries and numerical computation crates. |

## Key Considerations

- 2D vs. 3D: Choose the appropriate library based on whether you need 2D or 3D visualization.
- Web vs. Desktop: Consider whether you need web-based or desktop visualization. For web visualization, using JavaScript charting libraries and interacting via WebAssembly is common.
- Interactivity: If you need interactive plots, GUI frameworks like `iced` or `egui` might be a good choice.
- Performance: For performance-critical visualizations, consider using lower-level graphics libraries like `glium` or `wgpu`.
- Complexity: Choose the library that best balances features and complexity for your project.

## Choosing the Right Crate

- Simple 2D Plots: `plotters`, `chartist`
- Advanced 2D/3D Plots: `plotters` (with custom drawing or extensions), `glium`, `wgpu`
- Interactive Plots/GUIs: `iced`, `egui`
- Web Visualization: `yew` with JavaScript charting libraries
- Graph Visualization: [`petgraph`][c-petgraph]⮳{{hi:petgraph}}  with graph rendering tools

</div>
