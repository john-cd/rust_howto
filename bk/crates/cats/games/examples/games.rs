// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Collects three markers in a small Macroquad game.

use macroquad::prelude::*;

const PLAYER_RADIUS: f32 = 0.035;
const MARKER_RADIUS: f32 = 0.025;
const SPEED: f32 = 0.55;
const START: Vec2 = vec2(0.15, 0.5);
const MARKERS: [Vec2; 3] = [vec2(0.28, 0.5), vec2(0.5, 0.25), vec2(0.78, 0.72)];

/// The position and score live in a fixed one-unit square world.
struct Game {
    player: Vec2,
    score: usize,
}

impl Game {
    /// Starts a round at the same position and score.
    fn new() -> Self {
        Self {
            player: START,
            score: 0,
        }
    }

    /// Applies movement and collects the current marker if the circles overlap.
    ///
    /// Direction components come from input, and elapsed time is in seconds.
    fn update(&mut self, direction: Vec2, elapsed: f32) {
        let Some(&marker) = MARKERS.get(self.score) else {
            return;
        };
        self.player += direction.normalize_or_zero() * SPEED * elapsed.clamp(0.0, 0.05);
        self.player = self
            .player
            .clamp(Vec2::splat(PLAYER_RADIUS), Vec2::splat(1.0 - PLAYER_RADIUS));

        if self.player.distance(marker) <= PLAYER_RADIUS + MARKER_RADIUS {
            self.score += 1;
        }
    }
}

/// Reads the keyboard, keeping the window API outside the update method.
fn direction() -> Vec2 {
    let axis = |positive, negative| {
        f32::from(u8::from(is_key_down(positive))) - f32::from(u8::from(is_key_down(negative)))
    };
    vec2(
        axis(KeyCode::Right, KeyCode::Left),
        axis(KeyCode::Down, KeyCode::Up),
    )
}

/// Draws the square world centered in the current window.
fn draw(game: &Game) {
    clear_background(BLACK);
    let size = screen_width().min(screen_height());
    let origin = (vec2(screen_width(), screen_height()) - Vec2::splat(size)) / 2.0;
    let to_screen = |point: Vec2| origin + point * size;
    draw_rectangle_lines(origin.x, origin.y, size, size, 2.0, GRAY);

    if let Some(&marker) = MARKERS.get(game.score) {
        let position = to_screen(marker);
        draw_circle(position.x, position.y, MARKER_RADIUS * size, GOLD);
    }
    let player = to_screen(game.player);
    draw_circle(player.x, player.y, PLAYER_RADIUS * size, SKYBLUE);

    let status = if game.score == MARKERS.len() {
        "You collected all three markers! R restarts.".to_owned()
    } else {
        format!(
            "{}/{}  Arrow keys move · R restarts · Esc quits",
            game.score,
            MARKERS.len()
        )
    };
    draw_text(&status, 12.0, 26.0, 21.0, WHITE);
}

/// Presents each updated frame and exits on Escape or window close.
#[macroquad::main("Collect the Markers")]
async fn main() {
    let mut game = Game::new();
    let smoke = std::env::var_os("RUST_HOWTO_SMOKE_TEST").is_some();
    let mut frames = 0;
    while !is_key_pressed(KeyCode::Escape) {
        if is_key_pressed(KeyCode::R) {
            game = Game::new();
        }
        if smoke {
            game.update(Vec2::X, 0.05);
        } else {
            game.update(direction(), get_frame_time());
        }
        draw(&game);
        next_frame().await;

        if smoke {
            frames += 1;
            if frames == 3 {
                assert_eq!(game.score, 1);
                println!("Presented three frames and collected the first marker.");
                break;
            }
        }
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Counts each marker once and stops updating after the last one.
    #[test]
    fn scoring_finishes_the_round() {
        let mut game = Game::new();
        for (index, marker) in MARKERS.into_iter().enumerate() {
            game.player = marker;
            game.update(Vec2::ZERO, 0.01);
            assert_eq!(game.score, index + 1);
            game.update(Vec2::ZERO, 0.01);
            assert_eq!(game.score, index + 1);
        }
        game.update(Vec2::X, 0.05);
        assert_eq!(game.player, MARKERS[2]);
        game = Game::new();
        assert_eq!(game.score, 0);
        assert_eq!(game.player, START);
    }

    /// Keeps diagonal speed consistent and clamps movement at the boundary.
    #[test]
    fn movement_respects_speed_and_world_bounds() {
        let mut game = Game::new();
        game.update(vec2(1.0, 1.0), 0.05);
        assert!((game.player.distance(START) - SPEED * 0.05).abs() < 1e-6);
        game.player.x = PLAYER_RADIUS;
        game.update(-Vec2::X, 0.05);
        assert!((game.player.x - PLAYER_RADIUS).abs() < f32::EPSILON);
        assert_eq!(game.score, 0);
    }
}
