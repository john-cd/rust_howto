# Cross-platform Applications

## Crux

[![crux-badge]][crux]  [![crux-github][crux-github-badge]][crux-github]

[`Crux`][Crux]⮳ is an experimental approach to building cross-platform applications.

 It splits the application into two distinct parts, a Core built in Rust, which drives as much of the business logic as possible, and a Shell, built in the platform native language (Swift, Kotlin, TypeScript), which provides all interfaces with the external world, including the human user, and acts as a platform on which the core runs.

 The architecture is event-driven, based on event sourcing. The Core holds the majority of state, which is updated in response to events happening in the Shell. The interface between the Core and the Shell is messaged based.

The user interface layer is built natively, with modern declarative UI frameworks such as Swift UI, Jetpack Compose and React/Vue or a WASM based framework on the web.

{{#include ../refs/link-refs.md}}
