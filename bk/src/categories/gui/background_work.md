<!-- TODO Reviewed ??-??-2026. -->

# Responsive Background Work

A UI frame should process input and draw current state without waiting for parsing,
model inference, media decoding, or network I/O. Give a worker an immutable input
snapshot and tag its reply with the input revision. Apply the reply only if that
revision is still current; cancellation alone cannot prevent an already queued stale
reply from arriving.

## A Bounded egui Worker

The example owns one worker and a one-message reply channel. It copies the input,
computes a bounded sum, requests repaint, and polls completion without joining an
active thread inside a frame. Editing the input invalidates an in-flight result.
Only the UI thread changes visible state. Errors are displayed separately from values.

```rust,ignore,noplayground
{{#include ../../../platform/cats/gui/examples/background_work.rs:example}}
```

```sh
just cargo run --manifest-path bk/platform/Cargo.toml --locked -p gui --example background_work
just cargo test --manifest-path bk/platform/Cargo.toml --locked -p gui --example background_work
just cargo run --manifest-path bk/platform/Cargo.toml --locked -p gui --example background_work -- --smoke
```

The optional `--smoke` mode launches a calculation, applies the expected reply, and
closes the native window automatically. It fails if the expected result is not applied;
it does not certify keyboard, screen-reader, or visual behavior.

The worker uses [`Context::request_repaint`](https://docs.rs/egui/0.37.0/egui/struct.Context.html#method.request_repaint)
to wake eframe; the short repaint deadline also covers the send/thread-completion race.
No UI widget is accessed from the worker. The tests cover stale/current replies,
visible failure, actual worker completion, and owned-thread cleanup without a display.

Closing the app sets a cooperative cancellation flag and joins this bounded CPU loop.
For long I/O, cancellation must also interrupt the operation or use a deadline; merely
setting a flag does not stop a blocking read. Do not copy the drop-time join into an
unbounded network worker. A production queue also needs a policy for replacing pending
requests and preserving errors; spawning one thread per keystroke is not that policy.

## Toolkit Coverage

| Approach | Common crates | Staged coverage and limits |
| --- | --- | --- |
| Immediate-mode desktop | `egui`, `eframe`, `egui_plot` | [Immediate-mode GUI](immediate_mode_gui.md), layout tests and this worker. |
| Declarative desktop | `iced`, `slint`, `floem`, `xilem` | [Retained-mode GUI](retained_mode_gui.md); framework features and native backend prerequisites differ. Slint has headless logic tests; one framework's test does not validate another. |
| Native Linux toolkit | `gtk4`, `relm4` | [GTK](gtk.md); requires the actual GTK development/runtime libraries and native Linux execution. |
| Web or webview UI | `dioxus`, `tauri`, `wry` | [Web-based GUI](web_based_gui.md); distinguish browser, SSR and desktop webview builds. Some webview adapters remain disabled pending dependency compatibility. |
| Windows and pixels | `winit`, `softbuffer`, `pixels`, `minifb` | [Window Creation](window_creation.md) and [2D Renderers](../graphics/2d_renderers.md). Window events, DPI and surface lifetime remain application responsibilities. |
| Platform integration | `rfd`, `arboard`, `accesskit` | [Dialogs](file_dialogs.md), [clipboard](clipboard.md), and [accessibility](../accessibility/index.md). Device/UI tests are separate from pure state tests. |

Use each toolkit's own UI-thread scheduling mechanism when porting the pattern.
Generation checks, immutable snapshots and bounded queues remain useful across toolkits.
Also test keyboard navigation, focus, resizing, DPI changes and assistive technology;
passing state tests does not certify those interactions.

## Related Topics

- [Immediate-mode GUI](immediate_mode_gui.md).
- [Retained-mode GUI](retained_mode_gui.md).
- [Window Creation](window_creation.md).
- [Accessibility](../accessibility/index.md).
