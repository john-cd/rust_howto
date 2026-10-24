<!-- TODO Reviewed ??-??-2026 -->

# Terminal User Interfaces

{{#include tui.incl.md}}

Use a text user interface (TUI) when users need to inspect or edit state through a persistent terminal screen. For a command that asks one question or reports progress, a regular command-line interface is simpler and works better in scripts and redirected output.

Most TUIs separate application state, event handling, and rendering:

- Store the current screen, selection, and user-entered values in application state.
- Turn keyboard, mouse, resize, and application events into state updates.
- Render the complete current view from that state on each draw.

This separation keeps input behavior testable without a real terminal. Handle escape and quit keys, terminal resize, and errors deliberately; make keyboard navigation and visible focus cues clear.

## Build Complex TUI {#ratatui}

[![ratatui][c~ratatui~docs~badge]][c~ratatui~docs]{{hi:ratatui}}
[![ratatui~crates.io][c~ratatui~crates.io~badge]][c~ratatui~crates.io]
[![ratatui~repo][c~ratatui~repo~badge]][c~ratatui~repo]
[![ratatui~lib.rs][c~ratatui~lib.rs~badge]][c~ratatui~lib.rs]

[`ratatui`][c~ratatui~website]↗ is a high-level library of widgets, layouts, and rendering utilities. It draws a frame from application state; pair it with a backend such as `crossterm` to read terminal events and manage terminal modes.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/ratatui.rs:example}}
```

Ratatui applications normally have a loop that draws the current state, reads events, updates that state, and repeats until the application exits. The example initializes raw mode and the alternate screen through Ratatui, then restores the terminal after the loop. `ratatui::init` also installs a panic hook; if you manage terminal setup yourself, restore the previous terminal state on both errors and panics.

Keep blocking terminal reads off an async runtime worker. For an async application, use an async event stream or read events on a dedicated thread and pass them to async tasks over a channel. Do not write unrelated output while the full-screen UI is active; route diagnostics to a file or coordinate them with the event loop. Keep machine-readable command output separate from the TUI mode.

[`ratatui`][c~ratatui~docs]↗{{hi:ratatui}} offers templates to get started. For example, to use the simple template, run the following commands:

```bash
cargo install cargo-generate
cargo generate ratatui/templates simple
```

## See Also {#see-also .skip}

[![tui][c~tui~docs~badge]][c~tui~docs] [![tui~crates.io][c~tui~crates.io~badge]][c~tui~crates.io] [![tui~repo][c~tui~repo~badge]][c~tui~repo] [![tui~lib.rs][c~tui~lib.rs~badge]][c~tui~lib.rs]{{hi:tui}}{{hi:Terminal}}{{hi:Dashboard}}{{hi:tui}}

`tui` is another library for terminal interfaces and dashboards. Choose a library based on the backend, widget set, maintenance activity, and whether its event model fits the application.

[![r3bl_tuify][c~r3bl_tuify~docs~badge]][c~r3bl_tuify~docs]{{hi:r3bl_tuify}} [![r3bl_tuify~crates.io][c~r3bl_tuify~crates.io~badge]][c~r3bl_tuify~crates.io]↗ [![blog~tuify][blog~tuify~badge]][blog~tuify]↗

`r3bl_tuify` provides terminal selection and menu components. It can be useful when an application needs a focused interactive picker rather than a full-screen TUI.

## Related Topics {#related-topics .skip}

- [[ansi_terminal | ANSI Terminal]].
- [[gui | GUI]].
- [[user_interaction | User Interaction]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tui: expand](https://github.com/john-cd/rust_howto/issues/234)
</div>
