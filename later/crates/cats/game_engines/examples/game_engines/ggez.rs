// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Moves and draws a bouncing ball with ggez.

use ggez::Context;
use ggez::ContextBuilder;
use ggez::GameResult;
use ggez::event;
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::graphics::Canvas;

const BALL_RADIUS: f32 = 16.0;
const SCREEN_SIZE: Vec2 = Vec2::new(800.0, 600.0);

/// Stores a ball's position and velocity in screen coordinates.
#[derive(Debug, Clone, Copy)]
struct Ball {
    /// The center of the ball.
    position: Vec2,
    /// The velocity of the ball in pixels per second.
    velocity: Vec2,
}

/// Advances the ball and reflects it from the edges of the screen.
fn step(ball: &mut Ball, delta_seconds: f32, bounds: Vec2) {
    ball.position += ball.velocity * delta_seconds;

    if ball.position.x < BALL_RADIUS {
        ball.position.x = BALL_RADIUS;
        ball.velocity.x = ball.velocity.x.abs();
    } else if ball.position.x > bounds.x - BALL_RADIUS {
        ball.position.x = bounds.x - BALL_RADIUS;
        ball.velocity.x = -ball.velocity.x.abs();
    }

    if ball.position.y < BALL_RADIUS {
        ball.position.y = BALL_RADIUS;
        ball.velocity.y = ball.velocity.y.abs();
    } else if ball.position.y > bounds.y - BALL_RADIUS {
        ball.position.y = bounds.y - BALL_RADIUS;
        ball.velocity.y = -ball.velocity.y.abs();
    }
}

/// Holds the game state and reusable circle mesh.
struct MainState {
    /// The moving ball.
    ball: Ball,
    /// The circle mesh rendered each frame.
    circle: graphics::Mesh,
}

impl MainState {
    /// Creates the ball and its circle mesh.
    ///
    /// # Errors
    ///
    /// Returns a graphics error if the circle mesh cannot be created.
    fn new(context: &Context) -> GameResult<Self> {
        let circle = graphics::Mesh::new_circle(
            context,
            graphics::DrawMode::fill(),
            Vec2::ZERO,
            BALL_RADIUS,
            2.0,
            graphics::Color::WHITE,
        )?;

        Ok(Self {
            ball: Ball {
                position: SCREEN_SIZE / 2.0,
                velocity: Vec2::new(160.0, 120.0),
            },
            circle,
        })
    }
}

impl EventHandler for MainState {
    /// Advances the ball for one frame.
    fn update(&mut self, context: &mut Context) -> GameResult {
        step(
            &mut self.ball,
            context.time.delta().as_secs_f32(),
            SCREEN_SIZE,
        );
        Ok(())
    }

    /// Draws the ball on a black background.
    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(context, graphics::Color::BLACK);
        canvas.draw(
            &self.circle,
            graphics::DrawParam::default().dest(self.ball.position),
        );
        canvas.finish(context)
    }
}

/// Builds the game context and starts the ggez event loop.
///
/// # Errors
///
/// Returns a ggez error if context or graphics initialization fails.
pub fn run() -> GameResult {
    let (context, event_loop) =
        ContextBuilder::new("rust_howto_ggez", "rust_howto").build()?;
    let state = MainState::new(&context)?;

    event::run(context, event_loop, state)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the ball reverses direction when it reaches an edge.
    #[test]
    fn step_reflects_ball_from_left_edge() {
        let mut ball = Ball {
            position: Vec2::new(20.0, 50.0),
            velocity: Vec2::new(-10.0, 0.0),
        };

        step(&mut ball, 1.0, Vec2::new(100.0, 100.0));

        assert!((ball.position.x - BALL_RADIUS).abs() < f32::EPSILON);
        assert!((ball.velocity.x - 10.0).abs() < f32::EPSILON);
    }

    /// Runs the ggez entry point in a native graphics window.
    #[test]
    #[ignore = "opens a graphics window"]
    fn test_ggez_game_entry_point() -> GameResult {
        run()
    }
}
