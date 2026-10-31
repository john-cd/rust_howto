<!-- TODO Reviewed ??-??-2026 -->

# Game Engines

This crate contains code examples for the **Game Engines** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `bevy`, `version`, `default-features`, `features`, `optional`.

## Examples Covered

- **Game Engines** (`examples/game_engines/`): Demonstrates concepts like `bevy`, `fyrox`, `ggez`, `macroquad`.

## Usage

To check or run examples in this crate:

```console
cargo check --package game_engines --examples
cargo test --package game_engines
```

Code examples correspond to chapters in `bk/src/categories/game_engines` or related book sections.

The optional `ggez` 0.10 example supports Windows and Linux. To check its Windows
build, run `cargo check --locked --package game_engines --features ggez --example game_engines`
from `later/crates`.
