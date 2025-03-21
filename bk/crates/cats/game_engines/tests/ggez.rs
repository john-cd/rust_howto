// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use std::path;

// use ggez::Context;
// use ggez::ContextBuilder;
// use ggez::GameResult;
// use ggez::event::EventHandler;
// use ggez::event::EventsLoop;
// use ggez::graphics::Color;
// use ggez::graphics;

// // `ggez` is a lightweight game framework for Rust.
// // To run this example, you'll need to have a directory named resources in
// // the same folder as your Cargo.toml. This directory can be empty for now.

// // `MainState` is the main game state.
// struct MainState;

// impl MainState {
//     pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
//         let s = MainState;
//         Ok(s)
//     }
// }

// impl EventHandler for MainState {
//     // The `update` function updates the game state. In this simple example,
// it     // does nothing.
//     fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
//         Ok(())
//     }

//     // The `draw` function clears the screen and draws a piece of text
// ("Hello,     // ggez!") at the coordinates (200, 300).
//     fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
//         graphics::clear(ctx, Color::from_rgb(0, 0, 0));
//         let text = graphics::Text::new("Hello, ggez!");
//         graphics::draw(
//             ctx,
//             &text,
//             (ggez::mint::Point2 { x: 200.0, y: 300.0 },),
//         )?;
//         graphics::present(ctx)?;
//         Ok(())
//     }
// }

// // The `main` function sets up the `ggez` context and event loop, and then
// runs // the game.
// fn main() -> GameResult {
//     let (mut ctx, mut event_loop) =
//         ContextBuilder::new("hello_ggez", "author_name")
//             .add_resource_path(path::PathBuf::from("./resources"))
//             .build()
//             .expect("Could not create ggez context!");

//     let mut state = MainState::new(&mut ctx)?;
//     ggez::event::run(&mut ctx, &mut event_loop, &mut state)
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/769)
