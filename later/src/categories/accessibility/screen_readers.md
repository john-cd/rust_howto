<!-- TODO Reviewed ??-??-2026 -->

# Screen Readers

{{#include screen_readers.incl.md}}

## Make a User Interface Accessible to Screen Readers {#accesskit}

[![accesskit][c~accesskit~docs~badge]][c~accesskit~docs]{{hi:accesskit}}
[![accesskit~crates.io][c~accesskit~crates.io~badge]][c~accesskit~crates.io]
[![accesskit~repo][c~accesskit~repo~badge]][c~accesskit~repo]
[![accesskit~lib.rs][c~accesskit~lib.rs~badge]][c~accesskit~lib.rs]
[![cat~accessibility][cat~accessibility~badge]][cat~accessibility]{{hi:Accessibility}}
[![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`accesskit`][c~accesskit~docs]↗{{hi:accesskit}} represents an interface as a tree of nodes with roles, accessible names, values, and relationships. A toolkit sends full trees and incremental updates to a platform adapter; the adapter maps them to the operating system's accessibility API. AccessKit's released adapters include Windows UI Automation, macOS `NSAccessibility`, and Unix AT-SPI. A tree can be built and checked without opening a window, as in this example, but it does not prove that a running application works with a screen reader.

Framework integration is available in several toolkits: `eframe` enables AccessKit on native platforms with an available adapter, direct `egui` integrations can opt in, Slint exposes its accessibility integration through its `accessibility` feature, and Bevy includes an accessibility plugin. Set meaningful semantics for custom widgets and verify the actual application with a screen reader on each supported platform. See [Immediate Mode GUI](../gui/immediate_mode_gui.md) and [issue #385](https://github.com/john-cd/rust_howto/issues/385), and [Retained Mode GUI](../gui/retained_mode_gui.md) and [issue #389](https://github.com/john-cd/rust_howto/issues/389) for the related GUI chapter work.

```rust,editable
{{#include ../../../crates/cats/accessibility/examples/screen_readers/accesskit.rs:example}}
```

In a winit resumed handler, create the adapter for an initially hidden window and the application event proxy, then forward tree changes while accessibility is active:

```rust,ignore
let adapter = accesskit_winit::Adapter::with_event_loop_proxy(
    active_event_loop,
    window,
    proxy,
);
adapter.update_if_active(|| accessibility_tree());
```

## Related Topics {#related-topics .skip}

- [GUI](../gui/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
