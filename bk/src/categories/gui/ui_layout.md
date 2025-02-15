# UI layout

{{#include ui_layout.incl.md}}

## `taffy` {#taffy}

[![taffy][c-taffy-badge]][c-taffy] [![taffy-crates.io][c-taffy-crates.io-badge]][c-taffy-crates.io] [![taffy-github][c-taffy-github-badge]][c-taffy-github] [![taffy-lib.rs][c-taffy-lib.rs-badge]][c-taffy-lib.rs]{{hi:taffy}}{{hi:Cross-platform}}{{hi:Css-grid}}{{hi:Flexbox}}{{hi:Grid}}{{hi:Layout}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}}

[`taffy`][c-taffy]⮳{{hi:taffy}} is a flexible UI layout library. "Supports Flexbox and CSS Grid algorithms."

```rust,editable
{{#include ../../../crates/cats/gui/examples/ui_layout/taffy.rs:example}}
```

## `morphorm` {#morphorm}

[![morphorm][c-morphorm-badge]][c-morphorm] [![morphorm-crates.io][c-morphorm-crates.io-badge]][c-morphorm-crates.io] [![morphorm-github][c-morphorm-github-badge]][c-morphorm-github] [![morphorm-lib.rs][c-morphorm-lib.rs-badge]][c-morphorm-lib.rs]{{hi:morphorm}}{{hi:Flex}}{{hi:Gui}}{{hi:Layout}}{{hi:Ui}}

[`morphorm`][c-morphorm]⮳{{hi:morphorm}}  "implements its own layout algorithm based on Subform layout."

[`morphorm`][c-morphorm]⮳{{hi:morphorm}}  is a UI layout engine written in Rust. It is designed to be used in UI frameworks that need to arrange elements on a screen or within a container. It takes a tree of elements (nodes) and calculates their positions and sizes based on a set of layout rules. It is heavily inspired by CSS Flexbox and Grid layout models. Morphorm is a standalone crate with no external dependencies.

Morphorm is suitable for various use cases, including:

- UI Frameworks,
- Game Development,
- Developing cross-platform desktop applications,
- Layout in resource-constrained environments (embedded systems).

```rust,editable
{{#include ../../../crates/cats/gui/examples/ui_layout/morphorm.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 review](https://github.com/john-cd/rust_howto/issues/934)
</div>
