// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates a basic Bevy application that:
// //!
// //! 1. Sets up a 2D camera.
// //! 2. Spawns a sprite.
// //! 3. Moves the sprite horizontally across the screen.

// use bevy::prelude::*;

// fn main() {
//     // Creates a new Bevy app.
//     App::build()
//         // Adds the default plugins, including window, input, and rendering.
//         .add_plugins(DefaultPlugins)
//         // Adds the setup system. Runs once at the start.
//         .add_startup_system(setup.system())
//         // Adds the move_system. Runs every frame.
//         .add_system(move_system.system())
//         .run();
// }

// /// Sets up the game world by spawning a sprite.
// fn setup(mut commands: Commands) {
//     // Spawns a 2D camera.
//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//     // Spawns a simple sprite.
//     commands.spawn_bundle(Sprite {
//         transform: Transform {
//             translation: Vec3::new(0.0, 0.0, 0.0),
//             scale: Vec3::splat(0.5),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// }

// /// Moves the sprite horizontally across the screen.
// fn move_system(mut query: Query<&mut Transform>) {
//     for mut transform in query.iter_mut() {
//         transform.translation.x += 0.1;
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/767)
