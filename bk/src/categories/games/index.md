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

## Collect Three Markers with Macroquad {#collect-markers}

Move the blue player with the arrow keys. Touch the gold marker to score one
point and reveal the next. Collect all three to win, press R to restart, or
press Escape to quit. From a graphical desktop, run this at the repository root:

```console
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p games --example games
```

Each frame reads input, updates the world in seconds, then draws the result.
The update method normalizes diagonal movement, caps long frame durations,
and checks the distance between the player and the marker. Drawing scales
the same square world to the current window. Resizing changes the view while
the positions and score remain in world coordinates.

To experiment, move one marker in the source and run again. The layout
changes while scoring follows the same overlap rule. To add a fourth
marker, extend the marker list. The displayed total and the win condition
both come from its length. Use the [Game Development recipes](../game-development/game_development.md)
to explore collision queries and transforms independently of the window.

```rust,ignore
{{#include ../../../crates/cats/games/examples/games.rs:example}}
```

The headless checks cover scoring, restart, diagonal speed, and world bounds.
For a bounded desktop smoke run, set `RUST_HOWTO_SMOKE_TEST=1` in the shell.
The example supplies scripted movement, presents three frames, and exits after
collecting the first marker. An ordinary launch uses the keyboard.

## Related Topics

- [Game Engines and Frameworks](../game-engines/index.md).
- [Game Development](../game-development/index.md).
- [Graphics](../graphics/index.md).
- [Multimedia](../multimedia/index.md).
- [Rust game libraries](../game-engines/game_engines.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
