# Vector Graphics Formats

[![cat~rendering::data-formats][cat~rendering::data-formats~badge]][cat~rendering::data-formats]{{hi:Vector formats}}{{hi:Rendering data formats}}

Vector files describe paths, shapes, and styles independently of a fixed pixel grid. This category focuses on SVG; for APIs that draw vector geometry directly, see [[2d_vector_graphics | 2D Vector Graphics]].

## SVG documents {#svg}

[`usvg`][c~usvg~docs]↗ parses and normalizes SVG documents into a tree. [`resvg`][c~resvg~docs]↗ renders that tree into pixels, using [`tiny-skia`][c~tiny-skia~docs]↗ for its pixmap.

The [SVG rendering chapter][p~svg-rendering] includes tested inline examples for parsing with `usvg` and rasterizing with `resvg`. It also explains relative image/font resources and safe output sizing.

## Rasterize at a Bounded Size {#rasterize-svg}

A vector document has no pixel size of its own. This example scales the document so that its longer side is 100 pixels before rendering. Bound the output size in the same way when the SVG comes from an untrusted source.

```rust,editable
{{#include ../../../crates/cats/rendering_data_formats/examples/vector_formats/main.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[svg_rendering | SVG Rendering]].
- [[2d_vector_graphics | 2D Vector Graphics]].
- [[multimedia_images | Multimedia: Images]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
