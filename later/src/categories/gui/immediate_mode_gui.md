<!-- TODO Reviewed ??-??-2026 -->

# Immediate Mode Native GUIs

{{#include immediate_mode_gui.incl.md}}

Immediate-mode GUI code describes the controls for the current frame from the application's current state. The application calls its UI code again on the next frame, and the GUI library computes widget layout and draw commands from that description. With [`egui`][c~egui~docs]↗, your application owns its domain data while the context retains interaction details such as widget memory between frames. This style keeps UI flow close to ordinary Rust control flow; it also means expensive work should be cached or moved out of the frame-building path.

An immediate-mode interface differs from a [retained-mode GUI][ex~gui~retained-mode], where widgets are represented by a longer-lived tree that is updated as state changes. Choose based on your application's architecture and integrations, not on the name alone.

## When to Use Immediate Mode {#use-cases .skip}

Immediate-mode GUIs work especially well for development tools, editors, debug panels, and game interfaces. They are quick to prototype and easy to connect to application data. They also suit applications that already own a render loop or engine integration.

If a frame builds a very large interface or performs slow work while drawing widgets, the repeated layout and UI code can become costly. Keep expensive calculations and I/O outside the frame callback, cache reusable results, and update the model in response to user actions.

## Build a Desktop App with `eframe` {#eframe}

For a standalone native or web app, [`eframe`][c~eframe~docs]↗ supplies the application runner around `egui`. Implement `eframe::App` and build the widgets for each frame, as in the example below. The existing [egui example][ex~gui~egui] includes editable text, a slider, a button, and a headless interaction test.

```rust,editable
{{#include ../../../crates/cats/gui/examples/immediate_mode_gui/egui.rs:example}}
```

## Embed `egui` in a Game or Renderer {#integration-with-rendering}

When an application already has an event loop or renderer, integrate `egui` into it instead of starting a second application runner:

- For a [Bevy][c~bevy~docs]↗ game, [`bevy_egui`][c~bevy_egui~docs]↗ provides a Bevy plugin. Choose a release compatible with the Bevy version used by your project.
- For a custom `winit` application, [`egui-winit`][c~egui-winit~docs]↗ adapts window events and input for `egui`.
- For a custom `wgpu` renderer, [`egui-wgpu`][c~egui-wgpu~docs]↗ paints egui output with `wgpu`. These integrations can be combined when the application owns both the event loop and renderer.

The `egui` project also provides other platform and renderer integrations. Check the integration documentation for supported versions and features before selecting a combination.

## Plot Data and Test Interfaces {#plot-and-test}

Use [`egui_plot`][c~egui_plot~docs]↗ to display interactive 2D plots such as line series and bar charts. For interaction tests, [`egui_kittest`][c~egui_kittest~docs]↗ can exercise widgets through a test harness; the [add-on chapter][ex~gui~egui-kittest] has examples, alongside examples of plots and other extensions.

## Other Immediate-Mode Libraries {#other-immediate-mode-gui-libraries}

[`imgui`][c~imgui~docs]↗ is the Rust binding for Dear ImGui maintained by the [`imgui-rs` project][c~imgui~repo]↗. An application typically chooses a platform backend to deliver input and a renderer to draw its output. This can be a good fit when an engine or tool already uses Dear ImGui or when its ecosystem fits the application.

## Learn More {#learn-more .skip}

- Browse the [`egui` examples][egui.info~examples~website]↗.
- See the [egui add-on crates][ex~gui~egui-kittest] for tables, plots, docking, testing, and more.
- Compare the [retained-mode GUI chapter][ex~gui~retained-mode].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
