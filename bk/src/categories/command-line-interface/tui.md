# Terminal User Interfaces

{{#include tui.incl.md}}

## Build complex TUI {#ratatui}

[![ratatui][c-ratatui-badge]][c-ratatui]{{hi:ratatui}}
[![ratatui-crates.io][c-ratatui-crates.io-badge]][c-ratatui-crates.io]
[![ratatui-github][c-ratatui-github-badge]][c-ratatui-github]
[![ratatui-lib.rs][c-ratatui-lib.rs-badge]][c-ratatui-lib.rs]

[`ratatui`][c-ratatui-website]⮳ is a lightweight, high-level library that provides a set of widgets, layouts, and utilities to build complex Rust TUIs.

```rust,editable
{{#include ../../../crates/cats/command_line_interface/examples/ratatui.rs:example}}
```

[`ratatui`][c-ratatui]⮳{{hi:ratatui}} offers templates to get started. For example, to use the simple template, run the following commands:

```bash
cargo install cargo-generate
cargo generate ratatui/templates simple
```

## See also

[![tui][c-tui-badge]][c-tui] [![tui-crates.io][c-tui-crates.io-badge]][c-tui-crates.io] [![tui-github][c-tui-github-badge]][c-tui-github] [![tui-lib.rs][c-tui-lib.rs-badge]][c-tui-lib.rs]{{hi:tui}}{{hi:Terminal}}{{hi:Dashboard}}{{hi:tui}}

A library to build rich terminal user interfaces or dashboards

[![r3bl_tuify][c-r3bl_tuify-badge]][c-r3bl_tuify]{{hi:r3bl_tuify}} [![r3bl_tuify-crates.io][c-r3bl_tuify-crates.io-badge]][c-r3bl_tuify-crates.io]⮳ [![blog-tuify][blog-tuify-badge]][blog-tuify]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tui: expand (P1)](https://github.com/john-cd/rust_howto/issues/234)

[[gui | GUI]]
[[immediate_mode_gui | Immediate Mode GUI]]
[[other_gui | Other GUI]]
[[retained_mode_gui | Retained Mode GUI]]
[[web_based_gui | Web Based GUI]]

[[ansi_terminal | Ansi Terminal]]
[[user_interaction | User Interaction]]
</div>
