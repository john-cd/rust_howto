#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! A minimal `ggez` application that draws a greeting in a window.

use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use ggez::event;
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;

/// Holds the text drawn by the game.
struct MainState {
    greeting: graphics::Text,
}

impl EventHandler for MainState {
    /// Advances the game state for one frame.
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draws the greeting on a black background.
    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(context, Color::BLACK);
        canvas.draw(
            &self.greeting,
            DrawParam::default().dest(Vec2::new(32.0, 48.0)),
        );
        canvas.finish(context)
    }
}

/// Builds the game context and runs the `ggez` event loop.
///
/// # Errors
///
/// Returns a `ggez` error if context creation or the event loop fails.
pub fn run() -> GameResult {
    let (context, event_loop) =
        ContextBuilder::new("rust_howto_ggez", "rust_howto").build()?;
    let state = MainState {
        greeting: graphics::Text::new("Hello, ggez!"),
    };

    event::run(context, event_loop, state)
}
// ANCHOR_END: example
