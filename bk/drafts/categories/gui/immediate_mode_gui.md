# Immediate Mode Native GUI

{{#include immediate_mode_gui.incl.md}}

## `egui` {#egui}

[![egui][c-egui-badge]][c-egui]{{hi:egui}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}
[![egui-crates.io][c-egui-crates.io-badge]][c-egui-crates.io]
[![egui-github][c-egui-github-badge]][c-egui-github]
[![egui-lib.rs][c-egui-lib.rs-badge]][c-egui-lib.rs]

[`egui`][c-egui]⮳ is an easy-to-use immediate mode GUI that runs on both web and native. [`egui`][c-egui]⮳{{hi:egui}} aims to be the best choice when you want a simple way to create a GUI, or you want to add a GUI to a game engine.

Immediate-mode UI. Lots of widgets. The most usable out of the box if your needs are simple and you don't need to customize of the look and feel.

- [egui.rs][c-egui-website]⮳
- [`egui` demo lib][c-egui_demo_lib-github]⮳
- [`egui` widgets][c-egui-widgets]⮳
- [`egui` containers][c-egui-containers]⮳
- [`egui` main struct Ui][c-egui::Ui]⮳

```rust,editable
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[immediate_mode_gui: write](https://github.com/john-cd/rust_howto/issues/385)

## Key Concepts of Immediate Mode GUI

- Redrawing Every Frame: Immediate mode GUIs redraw the entire UI on every frame. This simplifies the API but can be less efficient for very complex UIs if not optimized.
- State Management: You manage the UI state directly in your application code. The [GUI][p-gui] library doesn't maintain an internal representation of the UI.
- Declarative Style (Often): While not strictly required, immediate mode GUIs are often used in a declarative style, where you describe the [UI layout][p-ui-layout] and elements in your code, and the GUI library handles the rendering.

For almost all cases where you want an immediate mode [GUI][p-gui] in Rust, [`egui`][c-egui]⮳{{hi:egui}} is the recommended choice. It's well-featured, performant, and has a great community.

## Immediate Mode GUI Libraries

- [`egui`][c-egui]⮳{{hi:egui}}: This is the most popular and widely used immediate mode [GUI][p-gui] library in Rust. It's well-documented, actively maintained, and a great choice for most projects. It's particularly well-suited for tools, [games][p-games], and applications where performance and ease of use are important.
- [`conrod`][c-conrod]⮳{{hi:conrod}}: An older immediate mode [GUI][p-gui] library. While still usable, [`egui`][c-egui]⮳{{hi:egui}} is generally preferred for new projects due to its better [documentation][p-documentation] and active maintenance.

## Integration with Rendering

Immediate mode [GUI][p-gui] libraries often need to be integrated with a [rendering][p-rendering] backend. [`egui`][c-egui]⮳{{hi:egui}} has its own renderers for various platforms and backends, including [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`glow`][c-glow]⮳{{hi:glow}}, and others.

## Use Cases

- Tools: Immediate mode GUIs are often used for creating tools and editors because they are easy to prototype and iterate on.
- Games: The performance of immediate mode GUIs makes them suitable for game UIs.
- Prototyping: The ease of use and rapid iteration make immediate mode GUIs good for prototyping.

## Advantages of Immediate Mode

- Simple API: Easy to learn and use.
- Fast prototyping: Quickly create and iterate on UI designs.
- Performance (when optimized): Can be very performant for many use cases.

## Disadvantages of Immediate Mode

- State management: Requires you to manage UI state directly.
- Redrawing: Redrawing the entire UI every frame can be inefficient for very complex UIs if not optimized.

</div>
