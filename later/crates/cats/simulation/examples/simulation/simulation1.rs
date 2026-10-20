#![allow(dead_code)]
// ANCHOR: example
//! Simulate a simple 2D bouncing ball using the `rapier2d` physics engine.
//!
//! This example creates a static ground and a dynamic ball, then steps the
//! physics simulation for several frames.

use rapier2d::prelude::*;

fn main() {
    let gravity = vector![0.0f32, -9.81f32];
    let integration_parameters = IntegrationParameters::default();

    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhaseMultiSap::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let mut multibody_joints = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();

    let ground_body = RigidBodyBuilder::fixed()
        .translation(vector![0.0f32, -1.0f32])
        .build();
    let ground_handle = bodies.insert(ground_body);
    let ground_collider = ColliderBuilder::cuboid(10.0f32, 1.0f32).build();
    colliders.insert_with_parent(ground_collider, ground_handle, &mut bodies);

    let ball_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0f32, 5.0f32])
        .linvel(vector![1.0f32, 0.0f32])
        .build();
    let ball_handle = bodies.insert(ball_body);
    let ball_collider = ColliderBuilder::ball(0.5f32)
        .restitution(0.7f32)
        .friction(0.5f32)
        .build();
    colliders.insert_with_parent(ball_collider, ball_handle, &mut bodies);

    for step in 0..120 {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut impulse_joints,
            &mut multibody_joints,
            &mut ccd_solver,
            None,
            &(),
            &(),
        );

        let ball = &bodies[ball_handle];
        let position = ball.position();
        let velocity = ball.linvel();

        println!(
            "step {:>3}: position=({:.2}, {:.2}), velocity=({:.2}, {:.2})",
            step,
            position.translation.x,
            position.translation.y,
            velocity.x,
            velocity.y,
        );
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
// [review](https://github.com/john-cd/rust_howto/issues/846)
