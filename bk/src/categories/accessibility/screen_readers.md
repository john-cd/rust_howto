<!-- TODO Reviewed ??-??-2026. -->

# Screen Readers

{{#include screen_readers.incl.md}}

## Make a User Interface Accessible to Screen Readers {#accesskit}

{{#badges accesskit accessibility=Accessibility gui=GUI}}

[`accesskit`][c~accesskit~docs]↗{{hi:accesskit}} represents an interface as a tree of nodes with roles, accessible names, values, and relationships. A toolkit sends full trees and incremental updates to a platform adapter; the adapter maps them to the operating system's accessibility API. AccessKit's released adapters include Windows UI Automation, macOS `NSAccessibility`, and Unix AT-SPI. A tree can be built and checked without opening a window, as in this example, but it does not prove that a running application works with a screen reader.

Framework integration is available in several toolkits: `eframe` enables AccessKit on native platforms with an available adapter, direct `egui` integrations can opt in, Slint exposes its accessibility integration through its `accessibility` feature, and Bevy includes an accessibility plugin. Set meaningful semantics for custom widgets and verify the actual application with a screen reader on each supported platform. See [Immediate Mode GUI](../gui/immediate_mode_gui.md) and [issue #385][rust-howto~issue-385], and [Retained Mode GUI](../gui/retained_mode_gui.md) and [issue #389][rust-howto~issue-389] for the related GUI chapter work.

```rust,noplayground
{{#include ../../../platform/cats/accessibility/examples/screen_readers/accesskit.rs:example}}
```

For a native window, create the adapter while the window is hidden, forward window events, and send the full tree when the platform requests it. Run this example on a desktop with a display and verify it with a screen reader:

```rust,noplayground
{{#include ../../../platform/cats/accessibility/examples/winit_accessibility.rs:example}}
```

## Related Topics {#related-topics .skip}

- [GUI](../gui/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
