<!-- TODO Reviewed ??-??-2026. -->

# Games

[![cat~games][cat~games~badge]][cat~games]{{hi:Games}}

These examples show several kinds of games built with Rust. Project availability and development status can change; follow each project link for current downloads and activity.

| Game | Genre / description |
| --- | --- |
| [`Veloren`][veloren.net~website]↗{{hi:Veloren}} | Multiplayer, open-world voxel role-playing game. |
| [`Zemeroth`][zemeroth~website]↗{{hi:Zemeroth}} | Turn-based tactical game. |
| [`Nox Futura`][nox-futura~website]↗{{hi:Nox Futura}} | 3D colony-building and survival simulation. |
| [`Epicinium`][epicinium~website]↗{{hi:Epicinium}} | Turn-based strategy game with simultaneous turns and resource management. |
| [`Hypercoven`][hypercoven~website]↗{{hi:Hypercoven}} | Real-time strategy game. |

## Browse More Games

- [Are We Game Yet? games][are-we-game-yet?-games~website]↗ lists games by genre and links to project pages.
- Browse the [games made with Rust on itch.io][itch.io~made-with-rust~website]↗{{hi:itch.io}} for indie releases and prototypes.

## Animate a Triangle with Macroquad

This example separates game state updates from drawing. Each frame advances the
triangle by the elapsed time, and pressing Space changes its color. Run `just cargo run --manifest-path later/crates/Cargo.toml --locked -p games --example games` from the repository root on a system with a graphical desktop.

```rust,ignore
{{#include ../../../crates/cats/games/examples/games.rs:example}}
```

## Related Topics

- [Game Engines and Frameworks](../game-engines/index.md).
- [Game Development](../game-development/index.md).
- [Graphics](https://github.com/john-cd/rust_howto/blob/86dcf0197f8d6810877a6021ffedcf84aa9230c8/bk/src/categories/graphics/index.md).
- [Multimedia](../multimedia/index.md).
- [Rust game libraries](../game-engines/game_engines.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
