<!-- TODO Reviewed ??-??-2026 -->

# Retained-Mode Native GUI

{{#include retained_mode_gui.incl.md}}

Retained-mode toolkits keep a longer-lived widget or view tree and update it in response to application state and user events. Immediate-mode toolkits instead describe the interface again as they build each frame; many current frameworks combine ideas from both approaches, so compare their state and rendering models as well as their labels. See the [immediate-mode GUI chapter](immediate_mode_gui.md) for the other style.

## Compare Toolkits

| Toolkit                              | UI and state model                                                                                             | Renderer                                                                          | Platforms                                                                                   | Maturity                                                                           |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| [`iced`][c~iced~docs]                | Elm-style state, messages, update, and view; the runtime manages the resulting widget tree.                    | `wgpu` or the `tiny-skia` software renderer.                                      | Windows, macOS, Linux, and Web.                                                             | The current 0.14 docs still describe it as experimental.                           |
| [`slint`][c~slint~docs]              | Declarative `.slint` markup with reactive properties connected to application logic.                           | Feature- and backend-selected: software, FemtoVG, or Skia; Vello is experimental. | Desktop, embedded, mobile, and Rust-to-Wasm WebAssembly.                                    | Stable 1.x API; browser support has accessibility and web-integration limitations. |
| [`floem`][c~floem~docs]              | Fine-grained reactive signals update a persistent view tree.                                                   | `wgpu` through Vello or Vger, Skia, or a tiny-skia CPU fallback.                  | Windows, macOS, and Linux.                                                                  | Pre-1.0; upstream warns that breaking changes may occur.                           |
| [`vizia`][c~vizia~docs]              | Models hold application data; views bind to it and events update it.                                           | Skia.                                                                             | Native desktop through its window backends; current docs do not list Web or mobile targets. | The current crate is 0.4.                                                          |
| [`xilem`][c~xilem~docs] with Masonry | Xilem builds a reactive view tree; Masonry maintains the retained widget tree and handles event/update passes. | Vello and `wgpu` on the native backend; Xilem also has a Web backend.             | Native desktop and an experimental Web backend.                                             | Experimental.                                                                      |
| [GPUI][c~gpui~repo]                  | Hybrid: views describe a tree each frame, while GPUI owns entities and retained element state.                 | GPU-accelerated, with platform-specific rendering.                                | macOS, Windows, and Linux.                                                                  | Pre-1.0 and actively developed with Zed.                                           |
| [Azul][c~azul~website]               | Data-driven UI with a DOM-like tree and CSS-like styling.                                                      | CPU or GPU rendering.                                                             | Desktop and mobile; Web applications are experimental.                                      | Version 0.2.0; the project marks Rust as its mature binding.                       |
| [`druid`][c~druid~repo]              | Data-first retained widgets.                                                                                   | Platform-specific native backends through Piet.                                   | Native desktop.                                                                             | Discontinued and unmaintained; upstream recommends Xilem for new projects.         |

Check a toolkit's current platform, renderer, and release notes before choosing it; these details change faster than the basic programming model.

## Iced {#iced}

[![iced~website][c~iced~website~badge]][c~iced~website] {{#badges iced}} {{hi:Graphics}} {{hi:Gui}} {{hi:Interface}} {{hi:Ui}} {{hi:Widgets}} {{#categories gui=GUI}}

[`iced`][c~iced~docs] follows the Elm Architecture: model application state, represent user actions as messages, update the state, and describe the view from it. Its current documentation lists Windows, macOS, Linux, and Web targets; the project still calls the library experimental.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/iced.rs:example}}
```

## Slint {#slint}

[![slint~website][c~slint~website~badge]][c~slint~website] {{#badges slint}} {{hi:Design}} {{hi:Graphics}} {{hi:Gui}} {{hi:Toolkit}} {{hi:Ui}} {{#categories gui=GUI,no-std="No standard library",rendering::engine="Rendering engine"}}

[`slint`][c~slint~docs] separates declarative `.slint` UI from Rust application logic and supports desktop, embedded, and mobile apps. Rust applications can also target WebAssembly, where Slint draws to a canvas rather than using normal HTML controls; its documentation describes limitations such as unavailable screen-reader accessibility. The example's `App::run()` opens a window, so launch it from `later/crates` with `cargo run -p gui --example retained_mode_gui --features slint -- slint`; its component test uses the headless testing backend.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/slint.rs:example}}
```

## Floem {#floem}

{{#badges floem}}

[`floem`][c~floem~docs] is a declarative native UI library built around fine-grained reactive signals. Its project lists Windows, macOS, and Linux support, several GPU renderers, and a CPU fallback; it is pre-1.0, so APIs may change.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/floem.rs:example}}
```

## Vizia {#vizia}

{{#badges vizia}}

[`vizia`][c~vizia~docs] binds views to application models and routes events back to update them. The example uses Vizia 0.4's signal-based API and lives in a separate package because its Skia dependency conflicts with the GUI workspace's Slint version.

```rust,editable
{{#include ../../../crates/cats/gui/vizia/src/main.rs:example}}
```

## Xilem and Masonry {#xilem}

[![xilem~website][c~xilem~website~badge]][c~xilem~website] {{#badges xilem}} {{hi:Gui}} {{hi:Ui}} {{hi:Native}} {{hi:Gpu}} {{hi:Performance}} {{#categories gui=GUI,graphics=Graphics,accessibility=Accessibility}}

[`xilem`][c~xilem~docs] is an experimental reactive application framework. Its native backend uses Masonry, which owns a retained widget tree; the project also maintains a separate Web backend.

```rust,editable
{{#include ../../../crates/cats/gui/examples/retained_mode_gui/xilem.rs:example}}
```

## Other GUI Frameworks {#other-gui-frameworks}

### GPUI {#gpui}

{{#badges gpui}}

[GPUI][c~gpui~repo] is the hybrid immediate- and retained-mode GPU-accelerated framework used to build Zed. It targets macOS, Windows, and Linux and remains in active pre-1.0 development.

### Azul {#azul}

[![azul~website][c~azul~website~badge]][c~azul~website] {{#badges azul}} {{hi:User-interface}} {{hi:Gui}} {{hi:Svg}} {{hi:Graphics}} {{#categories gui=GUI}}

[Azul][c~azul~website] offers a data-driven tree, CSS-like styling, and CPU or GPU rendering. Its 0.2.0 release covers desktop and mobile, with Web apps marked experimental; the project identifies its Rust binding as its mature language binding.

### Druid {#druid}

{{#badges druid gui=GUI}}{{hi:Toolkit}}{{hi:Gui}}{{hi:Ui}}

[`druid`][c~druid~docs] was a data-first native toolkit. Its maintainers discontinued the project and recommend Xilem for new applications; use Druid only when its existing feature set and lack of future fixes are acceptable.

## Related Topics {#related-topics .skip}

- [Immediate-mode GUIs](immediate_mode_gui.md).
- [UI layout](ui_layout.md), [text layout](text_layout.md), and [window creation](window_creation.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
