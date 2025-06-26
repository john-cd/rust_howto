# Cross-platform Development with Crux

{{#include crux.incl.md}}

## Develop Across Platforms with `crux` {#develop-across-platforms}

[![crux_core][c~crux_core~docs~badge]][c~crux_core~docs]{{hi:crux}} [![crux_core~github][c~crux_core~github~badge]][c~crux_core~github]{{hi:Cross-platform development}}

[`crux`][c~crux_core~docs]{{hi:crux_core}}⮳ is a framework for cross-platform{{hi:Cross-platform}} applications that can target both mobile native (iOS/Android) and web platforms using a single codebase, building the majority of the application code once, in Rust.

It splits the application into two distinct parts, a 'Core' built in Rust, which drives as much of the business logic as possible, and a very thin 'Shell', built in the platform-native language (Swift, Kotlin, TypeScript), which provides all interfaces with the external world, including the human user, and acts as a platform on which the core runs.

The fundamental architectural concept is the strict separation of pure computational tasks from tasks that cause side effects. The [architecture][p~architecture] is event-driven{{hi:Event-driven}}, based on event sourcing{{hi:Event sourcing}}. The Core holds the majority of state, which is updated in response to events happening in the Shell. The interface between the Core and the Shell is message-based{{hi:Message-based}}.

The user interface{{hi:User interface}} layer is built natively, with modern declarative UI frameworks such as Swift UI, Jetpack Compose, and React/Vue or a WASM{{hi:WASM}} based framework on the web (e.g. Yew).

```rust,editable
{{#include ../../../crates/other/examples/cross_platform/crux.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[crux: add / edit; link to architecture pages](https://github.com/john-cd/rust_howto/issues/582)
</div>
