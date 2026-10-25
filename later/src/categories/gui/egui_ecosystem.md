<!-- TODO Reviewed ??-??-2026 -->

# `egui` Add-on Crates

{{#include egui_ecosystem.incl.md}}

[`egui`][c~egui~docs]↗{{hi:egui}} itself provides the basic widgets. Most applications soon need tables, plots, docking, notifications and icons, which come from add-on crates. Pick versions that depend on the same `egui` version as your application: a mismatch fails to compile with confusing "expected `egui::Ui`, found `egui::Ui`" errors. See the [`egui`][p~egui] page for a first application.

## Headless Testing {#egui-headless}

[![egui][c~egui~docs~badge]][c~egui~docs]{{hi:egui}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}{{hi:Testing}}

An `egui::Context` does not need a window. `Context::run_ui` runs one frame from a `RawInput` that you build, and returns the shapes to paint. Use it to run the examples of this page in continuous integration, where there is no display. Add events to the `RawInput` to simulate input. For tests that find widgets by label and click them, look at the `egui_kittest` crate.

```rust,editable
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui_headless.rs:example}}
```

## Data Tables {#egui-table}

[![egui_extras][c~egui_extras~docs~badge]][c~egui_extras~docs]{{hi:egui_extras}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:Table}}

[`egui_extras::TableBuilder`][c~egui_extras::TableBuilder~docs]↗{{hi:egui_extras::TableBuilder}} lays out a table with a header, resizable columns and striped rows. `body.rows` is virtualized: it calls your closure only for the visible rows, so a table of 10 000 rows is as cheap as one of 20. Every row must have the same height; use `body.heterogeneous_rows` when they differ.

```rust,editable,no_run
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui_table.rs:example}}
```

## Plots and Bar Charts {#egui-plot}

[![egui_plot][c~egui_plot~docs~badge]][c~egui_plot~docs]{{hi:egui_plot}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:Plot}}{{hi:Chart}}

[`egui_plot::Plot`][c~egui_plot::Plot~docs]↗{{hi:egui_plot::Plot}} shows interactive 2D plots: the user can pan, zoom and hover. Add items to the `PlotUi`: [`Line`][c~egui_plot::Line~docs]↗{{hi:egui_plot::Line}} from a `Vec<[f64; 2]>`, [`BarChart`][c~egui_plot::BarChart~docs]↗{{hi:egui_plot::BarChart}}, `VLine` markers, and more. The plot fits its bounds to the data unless you set them.

```rust,editable
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui_plot.rs:example}}
```

## Docking and Tabbed Panes {#egui-tiles}

[![egui_tiles][c~egui_tiles~docs~badge]][c~egui_tiles~docs]{{hi:egui_tiles}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:Docking}}{{hi:Tabs}}

[`egui_tiles::Tree`][c~egui_tiles::Tree~docs]↗{{hi:egui_tiles::Tree}} stores a layout of panes in tabs, rows, columns and grids that the user rearranges by dragging. Implement `Behavior` to draw your own pane type and its tab title. The tree is plain data: create it, add or remove tiles, and activate a tab without a window, which makes the layout logic testable.

```rust,editable
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui_tiles.rs:example}}
```

## Toasts and Icons {#egui-toasts-icons}

[![egui-notify][c~egui-notify~docs~badge]][c~egui-notify~docs]{{hi:egui-notify}} [![egui-phosphor][c~egui-phosphor~docs~badge]][c~egui-phosphor~docs]{{hi:egui-phosphor}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:Notification}}{{hi:Icon}}

[`egui-notify`][c~egui-notify~docs]↗{{hi:egui-notify}} shows short messages that fade out. Keep one `Toasts` in the application state, push messages from anywhere that has access to it, and call `show` once per frame. [`egui-phosphor`][c~egui-phosphor~docs]↗{{hi:egui-phosphor}} adds an icon font: register it once, then put constants such as `egui_phosphor::regular::TRASH` into the text of labels and buttons.

```rust,editable
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui_toasts_icons.rs:example}}
```

## In-app Log Viewer {#egui-log-viewer}

[![egui_tracing][c~egui_tracing~docs~badge]][c~egui_tracing~docs]{{hi:egui_tracing}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:Logging}}{{hi:tracing}}

[`egui_tracing::EventCollector`][c~egui_tracing::EventCollector~docs]↗{{hi:egui_tracing::EventCollector}} is a [`tracing`][c~tracing~docs]↗{{hi:tracing}} subscriber layer that keeps the events in memory. The `Logs` widget displays them, with filters by level and target. A user of a desktop application can then see the logs without a console. Install the layer once with `.init()` at startup, next to any other layer such as one that writes to a file.

```rust,editable
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui_log_viewer.rs:example}}
```

## More Add-ons {#more-add-ons}

| Need                       | Crate                                                                                | Notes                                                        |
| -------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------ |
| Graphs with layouts        | [`egui_graphs`][c~egui_graphs~docs]↗{{hi:egui_graphs}}                               | Node and edge graph widget built on `petgraph`.              |
| Drag-and-drop reordering   | [`egui_dnd`][c~egui_dnd~docs]↗{{hi:egui_dnd}}                                        | Reorder a list of items by dragging.                         |
| JSON tree viewer           | [`egui_json_tree`][c~egui_json_tree~docs]↗{{hi:egui_json_tree}}                      | Collapsible view of a `serde_json::Value`, with search.      |
| Markdown rendering         | [`egui_commonmark`][c~egui_commonmark~docs]↗{{hi:egui_commonmark}}                   | Render CommonMark text, including code blocks.               |
| File dialog inside egui    | [`egui-file-dialog`][c~egui-file-dialog~docs]↗{{hi:egui-file-dialog}}                | An `egui`-drawn dialog, unlike the native one of [`rfd`][c~rfd~docs]↗{{hi:rfd}}. |
| Pull to refresh            | [`egui_pull_to_refresh`][c~egui_pull_to_refresh~docs]↗{{hi:egui_pull_to_refresh}}    | Touch gesture for lists on mobile.                           |
| Scale the whole UI         | [`egui-scale`][c~egui-scale~docs]↗{{hi:egui-scale}}                                  | Let the user choose the UI scale.                            |

## Related Topics {#related-topics .skip}

- [Immediate-mode GUI][p~egui]
- [File Dialogs](file_dialogs.md)
- [Web-based GUI](web_based_gui.md)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[egui add-on crates](https://github.com/john-cd/rust_howto/issues/1933)
</div>
