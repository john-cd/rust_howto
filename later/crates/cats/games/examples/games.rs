// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A minimal game example using the `macroquad` rendering engine.
//!
//! This example creates a window, animates a rotating triangle, and responds to
//! the space bar by changing the triangle's color.

use std::f32::consts::PI;

use macroquad::prelude::*;

/// The distance in pixels from the center of the screen to each corner.
const TRIANGLE_RADIUS: f32 = 120.0;

/// Returns the corners of an equilateral triangle centered on `center`,
/// rotated by `angle` radians.
fn triangle_points(center: Vec2, radius: f32, angle: f32) -> [Vec2; 3] {
    [0.0, 2.0 * PI / 3.0, 4.0 * PI / 3.0].map(|offset| {
        let theta = angle + offset;
        center + vec2(theta.cos(), theta.sin()) * radius
    })
}

/// Draws a rotating shape whose color changes when the space key is pressed.
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
        let points = triangle_points(center, TRIANGLE_RADIUS, angle);

        draw_triangle(points[0], points[1], points[2], color);
        draw_text(
            "Press SPACE to change color",
            16.0,
            screen_height() - 20.0,
            24.0,
            WHITE,
        );

        angle += get_frame_time();
        next_frame().await;
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Every corner lies on the circle around the center.
    #[test]
    fn triangle_corners_are_on_the_circle() {
        let center = vec2(400.0, 300.0);
        for angle in [0.0, 0.7, PI] {
            for point in triangle_points(center, 120.0, angle) {
                assert!((point.distance(center) - 120.0).abs() < 1e-3);
            }
        }
    }

    /// The corners are evenly spaced and rotate with the angle.
    #[test]
    fn triangle_is_equilateral_and_rotates() {
        let center = Vec2::ZERO;
        let [a, b, c] = triangle_points(center, 100.0, 0.0);
        assert!((a - vec2(100.0, 0.0)).length() < 1e-3);
        let side = a.distance(b);
        assert!((side - b.distance(c)).abs() < 1e-3);
        assert!((side - c.distance(a)).abs() < 1e-3);

        let [rotated, ..] = triangle_points(center, 100.0, PI / 2.0);
        assert!((rotated - vec2(0.0, 100.0)).length() < 1e-3);
    }

    // output-only: the example draws into a window until it is closed.
    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_games_example_entry_point() {
        main();
    }
}
