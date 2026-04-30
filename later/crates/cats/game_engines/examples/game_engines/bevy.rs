#![allow(dead_code)]
// ANCHOR: example
use bevy::prelude::*;

fn main() {
    // Creates a new Bevy app.
    // In many CI/headless environments, running the full App with a window will fail.
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_system)
        .update(); // Run a single update cycle for demonstration/test

    println!("Bevy app initialized and updated successfully.");
}

/// Sets up the game world.
fn setup(mut commands: Commands) {
    // Spawns a simple entity with a Transform.
    commands.spawn(Transform::default());
}

/// Moves entities with a Transform.
fn move_system(mut query: Query<&mut Transform>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 10.0 * time.delta_secs();
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
