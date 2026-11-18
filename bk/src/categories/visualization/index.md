<!-- TODO Reviewed ??-??-2026. -->

# Visualization

[![cat~visualization][cat~visualization~badge]][cat~visualization]{{hi:Visualization}}

Choose an output based on what the reader needs to see. A table preserves exact reported values and units. A chart brings out patterns such as trends or relationships. Interactive charts let readers inspect individual points.

{{#include visualization.incl.md}}

## Choose an output

| Goal | Start with | Requirements |
| --- | --- | --- |
| Share a static chart | [Plotters SVG](visualization.md#plotters) | Generate on the command line and open the SVG in a viewer or browser. |
| Inspect points interactively | [Plotly HTML](visualization.md#graph-data) | Generate on the command line and open in a browser that can load Plotly.js. |
| Report exact readings and units | [Terminal table](terminal_tables.md) | Print plain text with `tabled`. |
| Update a chart in a browser app | [Chartistry](visualization.md#leptos-chartistry) | Build to WebAssembly with Trunk. |
| Embed charts in another application | [Browser and desktop alternatives](visualization.md#application-charts) | Choose a library that fits your GUI or web application. |

Label axes and units, make the series legible in grayscale, and offer a table or source data when readers need exact values. See [Data Processing][p~data-processing] for aggregating and preparing data and [Graphics][p~graphics] for drawing and rendering primitives.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
