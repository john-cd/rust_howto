#![allow(dead_code)]
// ANCHOR: example
//! This example uses the macroquad game engine to create a simple window
//! where a red square can be moved around using the arrow keys.
//! The square's movement speed is constant, and it moves smoothly based on
//! the frame time.

use macroquad::prelude::*;

/// Create a window with a moving red square controlled by the arrow keys.
#[macroquad::main("Moving Square")]
async fn main() {
    let mut position = vec2(100.0, 100.0);
    let speed = 200.0;

    loop {
        clear_background(WHITE);

        if is_key_down(KeyCode::Right) {
            position.x += speed * get_frame_time();
        }
        if is_key_down(KeyCode::Left) {
            position.x -= speed * get_frame_time();
        }
        if is_key_down(KeyCode::Up) {
            position.y -= speed * get_frame_time();
        }
        if is_key_down(KeyCode::Down) {
            position.y += speed * get_frame_time();
        }

        draw_rectangle(position.x, position.y, 50.0, 50.0, RED);

        next_frame().await
    }
}
// ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [finish](https://github.com/john-cd/rust_howto/issues/771) need proper testing
