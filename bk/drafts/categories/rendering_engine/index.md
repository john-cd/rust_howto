# Rendering Engines (Built on Low-Level APIs)

[![cat-rendering::engine][cat-rendering::engine-badge]][cat-rendering::engine]{{hi:Rendering engine}}

High-level solutions for rendering on the screen.

## 2D/3D General Purpose Rendering Engines

Consider:

- [`rend3`][c-rend3]⮳{{hi:rend3}}: A 3D rendering engine built on top of `wgpu`. Provides higher-level abstractions for rendering.
- [`bevy_render`][c-bevy_render]⮳{{hi:bevy_render}}: Bevy's rendering system (part of the `bevy` game engine).
- `three-rs`.

{{#include rendering_engines.incl.md}}

## Related Topics

- [[graphics | Graphics]].
- [[rendering | Rendering]].
- [[rendering_data-formats | Rendering: Data Formats]].
- [[rendering_graphics-api | Rendering Graphics API]] - 2D/3D Lower Level Graphics.

### Applications

- [[game-development | Game Development]].
- [[game_engines | Game Engines]].
- [[gui | GUI]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/955)
should be a pointer to rendering
</div>
