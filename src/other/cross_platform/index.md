# Cross-platform Development

{{#include index.incl.md}}

## Crux

[![crux][c-crux-badge]][c-crux]  [![crux-github][c-crux-github-badge]][c-crux-github]

[`crux`][c-crux]{{hi:crux}}⮳ is an experimental approach to building cross-platform{{hi:Cross-platform}} applications.

 It splits the application into two distinct parts, a Core built in Rust, which drives as much of the business logic as possible, and a Shell, built in the platform native language (Swift, Kotlin, TypeScript), which provides all interfaces with the external world, including the human user, and acts as a platform on which the core runs.

 The architecture is event-driven{{hi:Event-driven}}, based on event sourcing{{hi:Event sourcing}}. The Core holds the majority of state, which is updated in response to events happening in the Shell. The interface between the Core and the Shell is message-based{{hi:Message-based}}.

The user interface{{hi:User interface}} layer is built natively, with modern declarative UI frameworks such as Swift UI, Jetpack Compose and React/Vue or a WASM{{hi:WASM}} based framework on the web.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
