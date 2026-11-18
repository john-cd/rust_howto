<!-- TODO Reviewed ??-??-2026. -->

# Cross-platform Development with Crux

{{#include crux.incl.md}}

## Develop Across Platforms with `crux` {#develop-across-platforms}

[![crux_core][c~crux_core~docs~badge]][c~crux_core~docs]{{hi:crux}} [![crux_core~repo][c~crux_core~repo~badge]][c~crux_core~repo]{{hi:Cross-platform development}}

[`crux`][c~crux_core~docs]↗{{hi:crux_core}} shares application behavior across platform-specific shells. Keep domain rules and state transitions in a Rust core, then build the user interface and platform integrations with the tools native to each target. This lets a project reuse behavior without requiring every platform to share the same presentation layer.

### Model shared behavior in the core

A Crux application centers on four types: an `Event` describes something the app should handle, a `Model` stores its state, an `Effect` describes work that needs the outside world, and a `ViewModel` contains data for the UI. The core handles an event, updates its model, and returns commands that may request effects or a new render. This is an event-driven architecture; the model is not necessarily a durable event log.

### Run effects in the shell

The core stays independent of operating-system APIs. When it needs to make a network request, read a file, access a sensor, or use another platform service, it describes that work as an effect. The shell performs the effect and sends its result back to the core. Crux provides typed messages across the core-shell boundary, with generated bindings for supported non-Rust shells. Keep credentials, permissions, transport details, and other platform-specific decisions in the shell.

### Keep each UI native

Build the shell with the UI framework that fits each target, such as SwiftUI, Jetpack Compose, or a web framework. The shell translates user actions into core events and renders the view model. Share behavior that benefits from consistent rules; let each platform present it in the way that best fits its users.

### Test the core and the boundary

Drive the core with representative events and assert the resulting model, view model, and requested effects. These tests can run without a phone, browser, or live service. Add shell-level tests for the platform work itself, including effect execution, permission handling, serialization, and error mapping.

Crux is a good fit when several clients need the same business rules but need different interfaces. It adds a message and binding boundary, so applications with one target or heavily platform-specific behavior may be simpler without it. Crux is still pre-1.0. Keep the core and generated bindings on compatible versions.

In this example, two `Increment` events update the model and each requests a render. The shell reads the view and displays `Count is: 2`. The test checks the render effect as well as the final value. See the [upstream counter example](https://github.com/redbadger/crux/blob/master/examples/counter/README.md) for platform shells.

From `bk/crates`, run `cargo run --locked -p other --example cross_platform -- crux`.

```rust,noplayground
{{#include ../../../crates/other/examples/cross_platform/crux.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Cross-platform Development](index.md).
- [Architecture](../architecture/index.md).
- [Software Architecture Process][p~architecture-process].
- [WebAssembly](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/wasm/index.md).
- [Programming Languages](../../categories/parser-implementations/programming_languages.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
