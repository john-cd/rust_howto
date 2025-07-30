# Terminal User Interfaces

{{#include tui.incl.md}}

## Build Complex TUI {#ratatui}

[![ratatui][c~ratatui~docs~badge]][c~ratatui~docs]{{hi:ratatui}}
[![ratatui~crates.io][c~ratatui~crates.io~badge]][c~ratatui~crates.io]
[![ratatui~github][c~ratatui~github~badge]][c~ratatui~github]
[![ratatui~lib.rs][c~ratatui~lib.rs~badge]][c~ratatui~lib.rs]

[`ratatui`][c~ratatui~website]⮳ is a lightweight, high-level library that provides a set of widgets, layouts, and utilities to build complex Rust TUIs.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/ratatui.rs:example}}
```

[`ratatui`][c~ratatui~docs]⮳{{hi:ratatui}} offers templates to get started. For example, to use the simple template, run the following commands:

```bash
cargo install cargo-generate
cargo generate ratatui/templates simple
```

## See Also

[![tui][c~tui~docs~badge]][c~tui~docs] [![tui~crates.io][c~tui~crates.io~badge]][c~tui~crates.io] [![tui~github][c~tui~github~badge]][c~tui~github] [![tui~lib.rs][c~tui~lib.rs~badge]][c~tui~lib.rs]{{hi:tui}}{{hi:Terminal}}{{hi:Dashboard}}{{hi:tui}}

A library to build rich terminal user interfaces or dashboards

[![r3bl_tuify][c~r3bl_tuify~docs~badge]][c~r3bl_tuify~docs]{{hi:r3bl_tuify}} [![r3bl_tuify~crates.io][c~r3bl_tuify~crates.io~badge]][c~r3bl_tuify~crates.io]⮳ [![blog~tuify][blog~tuify~badge]][blog~tuify]⮳

## Related Topics {#related-topics}

- [[ansi_terminal | ANSI Terminal]].
- [[gui | GUI]].
- [[user_interaction | User Interaction]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tui: expand](https://github.com/john-cd/rust_howto/issues/234)
</div>
