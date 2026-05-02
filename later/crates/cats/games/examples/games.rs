#![allow(dead_code)]
// ANCHOR: example
//! A minimal game example using the `macroquad` rendering engine.
//!
//! This example creates a window, animates a rotating triangle, and responds to
//! the space bar by changing the triangle's color.

use std::f32::consts::PI;

use macroquad::prelude::*;

#[macroquad::main("Rendering Engine Example")]
async fn main() {
    let mut angle: f32 = 0.0;
    let mut color = BLUE;

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            color = Color::new(
                rand::gen_range(0.0, 1.0),
                rand::gen_range(0.0, 1.0),
                rand::gen_range(0.0, 1.0),
                1.0,
            );
        }

        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let radius = 120.0;
        let points = [
            center + vec2(angle.cos(), angle.sin()) * radius,
            center
                + vec2(
                    (angle + 2.0 * PI / 3.0).cos(),
                    (angle + 2.0 * PI / 3.0).sin(),
                ) * radius,
            center
                + vec2(
                    (angle + 4.0 * PI / 3.0).cos(),
                    (angle + 4.0 * PI / 3.0).sin(),
                ) * radius,
        ];

        draw_triangle(points[0], points[1], points[2], color);
        draw_text(
            "Press SPACE to change color",
            16.0,
            screen_height() - 20.0,
            24.0,
            WHITE,
        );

        angle += get_frame_time();
        next_frame().await
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_main() {
        main();
    }
}
// TODO review
