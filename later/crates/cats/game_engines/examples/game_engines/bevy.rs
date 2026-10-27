// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Moves an entity with a headless Bevy ECS app.

use std::time::Duration;

use bevy::prelude::*;

/// Stores an entity's position in world coordinates.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Position(Vec2);

/// Stores an entity's velocity in world units per second.
#[derive(Component, Debug, Clone, Copy)]
struct Velocity(Vec2);

/// Builds a minimal app that moves one entity.
fn build_app() -> App {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, spawn_mover)
        .add_systems(Update, movement)
}

/// Spawns the entity that the movement system updates.
fn spawn_mover(mut commands: Commands<'_, '_>) {
    commands.spawn((Position(Vec2::ZERO), Velocity(Vec2::new(1.0, 0.0))));
}

/// Moves entities according to their velocity and elapsed frame time.
fn movement(
    mut query: Query<'_, '_, (&mut Position, &Velocity)>,
    time: Res<'_, Time>,
) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * time.delta_secs();
    }
}

/// Runs two frames of the headless ECS example.
fn main() {
    let mut app = build_app();
    app.update();
    app.update();
    println!("Updated a Bevy ECS app without opening a window.");
}
// ANCHOR_END: example

/// Runs the headless Bevy example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the headless Bevy example entry point.
    #[test]
    fn test_bevy_game_engine_example() {
        main();
    }

    /// Verifies that the movement system advances an entity in headless mode.
    #[test]
    fn movement_advances_position_without_a_window() {
        let mut app = build_app();
        app.insert_resource(TimeUpdateStrategy::ManualDuration(
            Duration::from_secs(1),
        ));

        app.update();
        app.update();

        let world = app.world_mut();
        let mut positions = world.query::<&Position>();
        let position = positions
            .iter(world)
            .next()
            .expect("the startup system spawns one moving entity");

        assert!(position.0.x > 0.0);
    }
}
