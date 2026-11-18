<!-- TODO Reviewed ??-??-2026. -->

# Game Development

{{#include game_development.incl.md}}

Start with [Collect Three Markers](../games/index.md#collect-markers) for a
complete input, update, and drawing loop. Use the smaller examples here when
you need to isolate game state or vector math from the frame loop.

## Game Development {#game-development}

```rust,editable
{{#include ../../../crates/cats/game_development/examples/game_development/game_development1.rs:example}}
```

### `glam` {#glam}

{{#badges glam game-engines="Game engines" no-std="No standard library"}}{{hi:Gamedev}}{{hi:Math}}{{hi:Matrix}}{{hi:Quaternion}}{{hi:Vector}}

[`glam`][c~glam~docs]↗{{hi:glam}} is a 3D math library for [games][p~games] and [graphics][p~graphics].

"Fast math library optimized for [game development][p~game-development] use cases."

```rust,editable
{{#include ../../../crates/cats/game_development/examples/game_development/glam.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[game-engines | Game Engines]].
- [[games | Games]] written in Rust.
- [[graphics | Graphics]] and [[rendering_graphics-api | Rendering Graphics API]].
- [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
