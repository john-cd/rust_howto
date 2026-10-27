<!-- TODO Reviewed ??-??-2026 -->

# Text Layout

{{#include text_layout.incl.md}}

A text layout engine turns Unicode text and style into positioned glyphs that an application can draw and interact with. A complete layout pipeline chooses fonts with the needed character coverage, shapes character runs into glyphs, resolves mixed left-to-right and right-to-left text, and breaks lines to fit the available width. It then positions glyph runs and supports operations such as hit testing, caret movement, selection, and editing.

These steps matter for more than multilingual paragraphs: even short strings can include combining marks, ligatures, emoji, or characters missing from the primary font. A layout library keeps those rules consistent while the GUI handles application state and the renderer draws the resulting glyphs.

| Crate                                | Rich text                        | Bidirectional text | Editing support                 | Used by                                                                   |
| ------------------------------------ | -------------------------------- | ------------------ | ------------------------------- | ------------------------------------------------------------------------- |
| [`parley`][c~parley~docs]↗           | Styled spans and inline elements | Yes                | Selection and editing utilities | Xilem and Masonry                                                         |
| [`cosmic-text`][c~cosmic-text~docs]↗ | Attributed text spans            | Yes                | Editor APIs                     | COSMIC desktop and iced; glyphon uses it in a `wgpu` text-rendering stack |

Both examples use the bundled Roboto font, so their wrapping does not depend on installed system fonts. The font's SIL Open Font License 1.1 is included in `later/crates/cats/gui/assets/OFL.txt`.

## `parley` {#parley}

[![parley][c~parley~docs~badge]][c~parley~docs] [![parley~crates.io][c~parley~crates.io~badge]][c~parley~crates.io] [![parley~repo][c~parley~repo~badge]][c~parley~repo] [![parley~lib.rs][c~parley~lib.rs~badge]][c~parley~lib.rs]{{hi:parley}}{{hi:Layout}}{{hi:Text}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`parley`][c~parley~docs]↗ provides rich-text layout for applications built on the Linebender ecosystem. It handles styled ranges, line breaking, bidirectional text, and text selection or editing utilities. It is used by Xilem and Masonry.

```rust,editable
{{#include ../../../crates/cats/gui/examples/text_layout/parley.rs:example}}
```

## `cosmic-text` {#cosmic-text}

[![cosmic-text][c~cosmic-text~docs~badge]][c~cosmic-text~docs] [![cosmic-text~crates.io][c~cosmic-text~crates.io~badge]][c~cosmic-text~crates.io] [![cosmic-text~repo][c~cosmic-text~repo~badge]][c~cosmic-text~repo] [![cosmic-text~lib.rs][c~cosmic-text~lib.rs~badge]][c~cosmic-text~lib.rs]{{hi:cosmic-text}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`cosmic-text`][c~cosmic-text~docs]↗ is a pure Rust text engine with font fallback, shaping, bidirectional layout, wrapping, and attributed text. Its buffer and editor APIs support interactive text as well as measuring and laying out paragraphs. It is used in the COSMIC desktop and by `iced`; [`glyphon`][c~glyphon~repo]↗ uses it to prepare text for `wgpu` rendering.

```rust,editable
{{#include ../../../crates/cats/gui/examples/text_layout/cosmic_text.rs:example}}
```

## Lower-Level Building Blocks {#lower-level-building-blocks .skip}

Use lower-level crates when you need to assemble or customize a text pipeline. [`harfrust`][c~harfrust~repo]↗ and [`rustybuzz`][c~rustybuzz~repo]↗ implement HarfBuzz-style shaping: they map text runs and font data to glyph sequences and positions. [`swash`][c~swash~repo]↗ provides font and glyph utilities, including rasterization. These components do not by themselves provide a complete GUI layout and editing layer, such as font fallback, paragraph wrapping, selection, and cursor movement.

This chapter focuses on shaping and layout. For font parsing and glyph rasterization, see [Font Loading & Text Rendering][ex~gui~text-rendering]; GPU text drawing is a separate renderer concern.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
