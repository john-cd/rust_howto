#![allow(dead_code)]
// ANCHOR: example
use rapier2d::prelude::*;

fn main() {
    // A small simulation: one dynamic ball falling onto a static ground.
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();

    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let mut multibody_joints = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();

    let ground_body = RigidBodyBuilder::new_static().build();
    let ground_handle = bodies.insert(ground_body);
    let ground_collider = ColliderBuilder::cuboid(5.0, 0.1)
        .translation(vector![0.0, -0.5])
        .build();
    colliders.insert(ground_collider, ground_handle, &mut bodies);

    let ball_body = RigidBodyBuilder::new_dynamic()
        .translation(vector![0.0, 3.0])
        .build();
    let ball_handle = bodies.insert(ball_body);
    let ball_collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    colliders.insert(ball_collider, ball_handle, &mut bodies);

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
            &(),
            &(),
        );

        let ball = &bodies[ball_handle];
        let position = ball.position();
        println!(
            "step {:>3}: ball position = ({:.2}, {:.2}), velocity = ({:.2}, {:.2})",
            step,
            position.translation.x,
            position.translation.y,
            ball.linvel().x,
            ball.linvel().y,
        );
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
// TODO review / add to a chapter on physics engines with rapier2d
