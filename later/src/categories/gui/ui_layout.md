# UI Layout

{{#include ui_layout.incl.md}}

## `taffy` {#taffy}

[![taffy][c~taffy~docs~badge]][c~taffy~docs] [![taffy~crates.io][c~taffy~crates.io~badge]][c~taffy~crates.io] [![taffy~repo][c~taffy~repo~badge]][c~taffy~repo] [![taffy~lib.rs][c~taffy~lib.rs~badge]][c~taffy~lib.rs]{{hi:taffy}}{{hi:Cross-platform}}{{hi:Css-grid}}{{hi:Flexbox}}{{hi:Grid}}{{hi:Layout}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`taffy`][c~taffy~docs]↗{{hi:taffy}} is a flexible UI layout library. "Supports Flexbox and CSS Grid algorithms."

```rust,editable
{{#include ../../../crates/cats/gui/examples/ui_layout/taffy.rs:example}}
```

## `morphorm` {#morphorm}

[![morphorm][c~morphorm~docs~badge]][c~morphorm~docs] [![morphorm~crates.io][c~morphorm~crates.io~badge]][c~morphorm~crates.io] [![morphorm~repo][c~morphorm~repo~badge]][c~morphorm~repo] [![morphorm~lib.rs][c~morphorm~lib.rs~badge]][c~morphorm~lib.rs]{{hi:morphorm}}{{hi:Flex}}{{hi:Gui}}{{hi:Layout}}{{hi:Ui}}

[`morphorm`][c~morphorm~docs]↗{{hi:morphorm}} "implements its own layout algorithm based on Subform layout."

[`morphorm`][c~morphorm~docs]↗{{hi:morphorm}} is a UI layout engine written in Rust. It is designed to be used in UI frameworks that need to arrange elements on a screen or within a container. It takes a tree of elements (nodes) and calculates their positions and sizes based on a set of layout rules. It is heavily inspired by CSS Flexbox and Grid layout models. Morphorm is a standalone crate with no external dependencies.

Morphorm is suitable for various use cases, including:

- UI Frameworks.
- [Game Development][p~game-development].
- Developing [cross-platform][p~cross-platform] desktop applications.
- Layout in resource-constrained environments ([embedded][p~embedded] systems).

```rust,editable
{{#include ../../../crates/cats/gui/examples/ui_layout/morphorm.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/934)
</div>
