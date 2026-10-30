// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A minimal `fyrox` plugin: game logic that the engine calls every frame.
//!
//! Projects are normally generated with `fyrox-template`, which creates a
//! game crate that wires a plugin like this one to the editor and to a
//! standalone executor. The plugin below contains only the game logic.

use fyrox::core::reflect::prelude::*;
use fyrox::core::visitor::prelude::*;
use fyrox::plugin::Plugin;
use fyrox::plugin::PluginContext;
use fyrox::plugin::error::GameResult;

/// The game state. `Visit` saves and loads it; `Reflect` exposes its fields to
/// the editor.
#[derive(Default, Visit, Reflect, Debug)]
#[reflect(non_cloneable)]
pub struct Game {
    /// Number of updates so far.
    ticks: u32,
}

impl Game {
    /// Advances the game by one fixed time step.
    fn tick(&mut self) {
        self.ticks = self.ticks.saturating_add(1);
    }

    /// Returns the number of updates so far.
    fn ticks(&self) -> u32 {
        self.ticks
    }
}

impl Plugin for Game {
    /// The engine calls `update` at a fixed rate (`context.dt` seconds).
    fn update(&mut self, _context: &mut PluginContext<'_, '_>) -> GameResult {
        self.tick();
        Ok(())
    }
}

/// Creates the plugin the way a game executor would.
fn main() {
    let mut game = Game::default();
    game.tick();
    println!("The plugin ran {} update(s).", game.ticks());
}
// ANCHOR_END: example

/// Runs the Fyrox game engine example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The game logic is plain Rust: no window or GPU is needed to test it.
    #[test]
    fn tick_counts_updates() {
        let mut game = Game::default();
        game.tick();
        game.tick();
        assert_eq!(game.ticks(), 2);
    }

    /// Runs the Fyrox game engine example.
    #[test]
    fn test_fyrox_game_engine_example() {
        main();
    }
}
