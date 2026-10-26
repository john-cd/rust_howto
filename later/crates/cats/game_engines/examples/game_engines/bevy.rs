#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A minimal `bevy` app with a system that moves entities.

use bevy::prelude::*;

/// Builds and updates a headless Bevy app.
fn main() {
    // Creates a new Bevy app.
    // In many CI/headless environments, running the full App with a window will
    // fail.
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_system)
        .update(); // Run a single update cycle for demonstration/test

    println!("Bevy app initialized and updated successfully.");
}

/// Sets up the game world.
fn setup(mut commands: Commands<'_, '_>) {
    // Spawns a simple entity with a Transform.
    commands.spawn(Transform::default());
}

/// Moves entities with a Transform.
fn move_system(mut query: Query<'_, '_, &mut Transform>, time: Res<'_, Time>) {
    for mut transform in &mut query {
        transform.translation.x += 10.0 * time.delta_secs();
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[test]
fn test() {
    main();
}
