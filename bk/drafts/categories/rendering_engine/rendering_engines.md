# Rendering Engines

{{#include rendering_engines.incl.md}}

- [`rend3`][c-rend3]⮳{{hi:rend3}}: A 3D rendering engine built on top of `wgpu`. Provides higher-level abstractions for rendering.
- [`bevy_render`][c-bevy_render]⮳{{hi:bevy_render}}: Bevy's rendering system (part of the Bevy game engine).
- 3D Rendering: `three-rs`.

## Use a render engine {#render}

```rust,editable
{{#include ../../../crates/cats/rendering_engine/tests/render.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[rendering_engines: write](https://github.com/john-cd/rust_howto/issues/455)
</div>
