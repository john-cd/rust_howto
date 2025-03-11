# Immediate Mode Native GUIs

{{#include immediate_mode_gui.incl.md}}

For almost all cases where you want an immediate mode [GUI][p-gui] in Rust, [`egui`][c-egui]⮳{{hi:egui}} is the recommended choice. It's well-featured, performant, and has a great community.

## Key Concepts {#skip}

- Immediate mode GUIs redraw the entire UI on every frame. This simplifies the API but can be less efficient for very complex UIs.
- You'll manage the UI state directly in your application code. The [GUI][p-gui] library doesn't maintain an internal representation of the UI.
- Immediate-mode GUIs are often used in a declarative style, where you describe the [UI layout][p-ui-layout] and elements in your code, and the GUI library handles the rendering.

## Use Cases {#skip}

- Tools: Immediate mode GUIs are often used for creating tools and editors because they are easy to prototype and iterate on.
- [Games][p-games]: The [performance][p-performance] of immediate mode GUIs makes them suitable for game UIs.
- Prototyping: The ease of use and rapid iteration make immediate mode GUIs good for prototyping.

## `egui` {#egui}

[![egui][c-egui-badge]][c-egui]{{hi:egui}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}
[![egui-crates.io][c-egui-crates.io-badge]][c-egui-crates.io]
[![egui-github][c-egui-github-badge]][c-egui-github]
[![egui-lib.rs][c-egui-lib.rs-badge]][c-egui-lib.rs]

[`egui`][c-egui]⮳ is an easy-to-use immediate mode GUI that runs on both web and native. [`egui`][c-egui]⮳{{hi:egui}} aims to be the best choice when you want a simple way to create a GUI and you don't need to customize of the look and feel, or you want to add a GUI to a game engine. Great for tools and quick prototyping.

- [egui.rs][c-egui-website]⮳.
- [`egui` demo lib][c-egui_demo_lib-github]⮳.
- [`egui` widgets][c-egui-widgets]⮳.
- [`egui` containers][c-egui-containers]⮳.
- [`egui` main struct Ui][c-egui::Ui]⮳.

```rust,editable
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui.rs:example}}
```

## Integration with Rendering {#skip1}

Immediate mode [GUI][p-gui] libraries often need to be integrated with a [rendering][p-rendering] backend. [`egui`][c-egui]⮳{{hi:egui}} has its own renderers for various platforms and backends, including [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`glow`][c-glow]⮳{{hi:glow}}, and others.

## Other Immediate Mode GUI Libraries {#skip2}

- `imgui` provides Rust bindings for `Dear ImGui`.
- [`conrod`][c-conrod]⮳{{hi:conrod}} is an older immediate mode [GUI][p-gui] library.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[immediate_mode_gui: write](https://github.com/john-cd/rust_howto/issues/385)
need in depth review.
</div>
