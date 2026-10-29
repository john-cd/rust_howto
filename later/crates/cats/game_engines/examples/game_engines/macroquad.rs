// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A bouncing ball in a Macroquad game loop.

use macroquad::prelude::*;

const BALL_RADIUS: f32 = 15.0;

/// A ball with a position and velocity in pixels.
#[derive(Debug, Clone, Copy)]
struct Ball {
    /// The center of the ball.
    position: Vec2,
    /// The ball's velocity on each axis in pixels per second.
    velocity: Vec2,
}

/// Advances the ball and reflects it from each screen edge.
fn step(ball: &mut Ball, delta_seconds: f32, bounds: (f32, f32)) {
    ball.position += ball.velocity * delta_seconds;

    if ball.position.x < BALL_RADIUS {
        ball.position.x = BALL_RADIUS;
        ball.velocity.x = ball.velocity.x.abs();
    } else if ball.position.x > bounds.0 - BALL_RADIUS {
        ball.position.x = bounds.0 - BALL_RADIUS;
        ball.velocity.x = -ball.velocity.x.abs();
    }

    if ball.position.y < BALL_RADIUS {
        ball.position.y = BALL_RADIUS;
        ball.velocity.y = ball.velocity.y.abs();
    } else if ball.position.y > bounds.1 - BALL_RADIUS {
        ball.position.y = bounds.1 - BALL_RADIUS;
        ball.velocity.y = -ball.velocity.y.abs();
    }
}

/// Draws a bouncing ball and advances it once per frame.
async fn async_main() {
    let mut ball = Ball {
        position: vec2(100.0, 100.0),
        velocity: vec2(160.0, 120.0),
    };

    loop {
        clear_background(WHITE);
        step(
            &mut ball,
            get_frame_time(),
            (screen_width(), screen_height()),
        );
        draw_circle(ball.position.x, ball.position.y, BALL_RADIUS, RED);
        next_frame().await;
    }
}
// ANCHOR_END: example

/// Starts the Macroquad window and game loop.
pub fn run() {
    macroquad::Window::new("Bouncing Ball", async_main());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a ball hitting an edge reverses its velocity.
    #[test]
    fn step_reflects_a_ball_from_the_left_edge() {
        let mut ball = Ball {
            position: vec2(20.0, 50.0),
            velocity: vec2(-10.0, 0.0),
        };

        step(&mut ball, 1.0, (100.0, 100.0));

        assert!((ball.position.x - BALL_RADIUS).abs() < f32::EPSILON);
        assert!((ball.velocity.x - 10.0).abs() < f32::EPSILON);
    }

    /// Runs the Macroquad example entry point in a native graphics window.
    #[test]
    #[ignore = "opens a graphics window"]
    fn test_macroquad_example_entry_point() {
        run();
    }
}
