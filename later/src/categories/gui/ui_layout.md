<!-- TODO Reviewed ??-??-2026 -->

# UI Layout

{{#include ui_layout.incl.md}}

Layout engines calculate positions and sizes independently of widgets and drawing. This page focuses on engines that can be used without adopting a particular GUI toolkit.

Many GUI frameworks include their own layout model: [egui][ex~gui~egui] positions immediate-mode widgets, while [Iced][ex~gui~iced], [Slint][ex~gui~slint], and [Xilem][ex~gui~xilem] integrate layout with their component APIs. Prefer built-in layout when using that framework; use a separate engine when building a custom widget tree, game UI, or renderer.

## `taffy` {#taffy}

[![taffy][c~taffy~docs~badge]][c~taffy~docs] [![taffy~crates.io][c~taffy~crates.io~badge]][c~taffy~crates.io] [![taffy~repo][c~taffy~repo~badge]][c~taffy~repo] [![taffy~lib.rs][c~taffy~lib.rs~badge]][c~taffy~lib.rs]{{hi:taffy}}{{hi:Cross-platform}}{{hi:Css-grid}}{{hi:Flexbox}}{{hi:Grid}}{{hi:Layout}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

Taffy implements CSS Flexbox and Grid layout calculations in a toolkit-independent tree. It is also used by UI stacks such as Bevy UI and Dioxus; choose it when an application needs to compute layout separately from rendering.

```rust,editable
{{#include ../../../crates/cats/gui/examples/ui_layout/taffy.rs:example}}
```

## `morphorm` {#morphorm}

[![morphorm][c~morphorm~docs~badge]][c~morphorm~docs] [![morphorm~crates.io][c~morphorm~crates.io~badge]][c~morphorm~crates.io] [![morphorm~repo][c~morphorm~repo~badge]][c~morphorm~repo] [![morphorm~lib.rs][c~morphorm~lib.rs~badge]][c~morphorm~lib.rs]{{hi:morphorm}}{{hi:Flex}}{{hi:Gui}}{{hi:Layout}}{{hi:Ui}}

Morphorm uses a one-pass tree traversal to calculate node sizes and positions, with row, column, overlay, and grid layout types. Vizia uses it for adaptive layout; choose it when a toolkit or application needs a separate layout tree.

Morphorm is suitable for various use cases, including:

- UI Frameworks.
- [Game Development][p~game-development].
- Developing [cross-platform][p~cross-platform] desktop applications.
- Layout in resource-constrained environments ([embedded][p~embedded] systems).

```rust,editable
{{#include ../../../crates/cats/gui/examples/ui_layout/morphorm.rs:example}}
```

The compact snippet omits the small `Node`, `Tree`, `Store`, and `Cache` implementations required by Morphorm; the complete implementations remain in the compiled example.

## Related Topics {#related-topics .skip}

- GUI frameworks and rendering.
- Responsive layout and styling.
- Cross-platform user interfaces.
- Performance-sensitive UI design.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [review](https://github.com/john-cd/rust_howto/issues/934)
</div>
