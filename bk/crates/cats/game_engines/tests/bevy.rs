// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use bevy::prelude::*;

// // Sets up a basic Bevy application with a 2D camera and a simple system that
// // moves a sprite horizontally across the screen.

// fn main() {
//     // Creates a new Bevy app.
//     App::build()
//         .add_plugins(DefaultPlugins) // Adds the default plugins, including
// window, input, and rendering.         .add_startup_system(setup.system()) //
// Adds the setup system to run once at the start.
//         .add_system(move_system.system()) // Adds the move_system to run
// every frame.         .run();
// }
// fn setup(mut commands: Commands) {
//     // commands.spawn_bundle(OrthographicCameraBundle::new_2d()); // Sets up
// a     // 2D camera.
//     commands.spawn_bundle(Sprite {
//         // Spawns a simple sprite.
//         transform: Transform {
//             translation: Vec3::new(0.0, 0.0, 0.0),
//             scale: Vec3::splat(0.5),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// }

// fn move_system(mut query: Query<&mut Transform>) {
//     for mut transform in query.iter_mut() {
//         transform.translation.x += 0.1;
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/767)
