#![allow(dead_code)]
// ANCHOR: example
// ggez example (commented out due to dependency conflicts in the workspace)
/*
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Canvas};
use ggez::event::{self, EventHandler};

struct MainState;

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let text = graphics::Text::new("Hello, ggez!");
        canvas.draw(&text, graphics::DrawParam::from((ggez::glam::Vec2::new(200.0, 300.0),)));

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("hello_ggez", "author");
    println!("ggez context builder initialized.");
    Ok(())
}
*/
fn main() {
    println!("ggez example is currently disabled due to workspace dependency conflicts.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
