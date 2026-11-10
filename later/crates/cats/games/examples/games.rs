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

/// Represents the dynamic visual state of the game.
#[derive(Debug, Clone, Copy, PartialEq)]
struct GameState {
    /// The current rotation angle of the triangle in radians.
    angle: f32,
    /// The fill color of the rotating triangle.
    color: Color,
}

impl GameState {
    /// Creates a new default game state with zero rotation and blue color.
    fn new() -> Self {
        Self {
            angle: 0.0,
            color: BLUE,
        }
    }

    /// Advances the rotation angle by elapsed time and updates color if requested.
    fn update(
        &mut self,
        space_pressed: bool,
        delta_time: f32,
        color_provider: impl FnOnce() -> Color,
    ) {
        if space_pressed {
            self.color = color_provider();
        }
        self.angle += delta_time;
    }
}

/// Generates a random RGBA color with full opacity.
fn random_color() -> Color {
    Color::new(
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        1.0,
    )
}

/// Renders a single frame displaying the triangle and instruction overlay.
fn draw_frame(state: &GameState) {
    clear_background(BLACK);

    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let points = triangle_points(center, TRIANGLE_RADIUS, state.angle);

    draw_triangle(points[0], points[1], points[2], state.color);
    draw_text(
        "Press SPACE to change color",
        16.0,
        screen_height() - 20.0,
        24.0,
        WHITE,
    );
}

/// Draws a rotating shape whose color changes when the space key is pressed.
#[macroquad::main("Rendering Engine Example")]
async fn main() {
    let mut state = GameState::new();

    #[expect(
        clippy::infinite_loop,
        reason = "The frame loop runs until the window is closed and the process exits"
    )]
    loop {
        let space_pressed = is_key_pressed(KeyCode::Space);
        state.update(space_pressed, get_frame_time(), random_color);

        draw_frame(&state);

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

    /// Verifies that `GameState::new` initializes angle to 0.0 and color to BLUE.
    #[test]
    fn game_state_initializes_with_defaults() {
        let state = GameState::new();
        assert_eq!(state.angle, 0.0);
        assert_eq!(state.color, BLUE);
    }

    /// Verifies that `GameState::update` increments the angle by delta_time and preserves color
    /// when space is not pressed.
    #[test]
    fn update_advances_angle_without_changing_color_when_space_not_pressed() {
        let mut state = GameState::new();
        let mut provider_called = false;
        state.update(false, 0.5, || {
            provider_called = true;
            RED
        });

        assert!(!provider_called);
        assert!((state.angle - 0.5).abs() < f32::EPSILON);
        assert_eq!(state.color, BLUE);
    }

    /// Verifies that `GameState::update` updates the color when space key is pressed.
    #[test]
    fn update_changes_color_when_space_is_pressed() {
        let mut state = GameState::new();
        let mut provider_called = false;
        state.update(true, 0.25, || {
            provider_called = true;
            RED
        });

        assert!(provider_called);
        assert!((state.angle - 0.25).abs() < f32::EPSILON);
        assert_eq!(state.color, RED);
    }

    /// Verifies that `random_color` produces valid RGB components between 0 and 1 with an alpha of
    /// 1.
    #[test]
    fn random_color_generates_valid_rgba_bounds() {
        for _ in 0..100 {
            let color = random_color();
            assert!((0.0..=1.0).contains(&color.r));
            assert!((0.0..=1.0).contains(&color.g));
            assert!((0.0..=1.0).contains(&color.b));
            assert_eq!(color.a, 1.0);
        }
    }

    // output-only: the example draws into a window until it is closed.
    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_games_example_entry_point() {
        main();
    }
}
