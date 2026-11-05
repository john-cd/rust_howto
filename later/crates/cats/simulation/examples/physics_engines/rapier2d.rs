// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A ball falling onto the ground in a 2D physics simulation with `rapier2d`.
//!
//! A physics engine advances bodies in small time steps, applying gravity and
//! resolving collisions. A rigid body is a shape that does not deform. Games
//! use physics engines for movement and collisions, and robotics uses them for
//! simulation.

use rapier2d::prelude::*;

/// The position and velocity of the ball after one simulation step.
struct Snapshot {
    /// The ball's position.
    position: (f32, f32),
    /// The ball's linear velocity.
    velocity: (f32, f32),
}

/// Simulates the falling ball and returns its state after every step.
fn simulate(steps: u32) -> Vec<Snapshot> {
    let mut trace = Vec::new();
    // A small simulation: one dynamic ball falling onto a static ground.
    let gravity = Vector::new(0.0, -9.81);
    let integration_parameters = IntegrationParameters::default();

    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhaseBvh::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let mut multibody_joints = MultibodyJointSet::new();
    let mut soft_bodies = SoftBodySet::new();
    let mut ccd_solver = CCDSolver::new();

    let ground_body = RigidBodyBuilder::fixed().build();
    let ground_handle = bodies.insert(ground_body);
    let ground_collider = ColliderBuilder::cuboid(5.0, 0.1)
        .translation(Vector::new(0.0, -0.5))
        .build();
    colliders.insert_with_parent(ground_collider, ground_handle, &mut bodies);

    let ball_body = RigidBodyBuilder::dynamic()
        .translation(Vector::new(0.0, 3.0))
        .build();
    let ball_handle = bodies.insert(ball_body);
    let ball_collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    colliders.insert_with_parent(ball_collider, ball_handle, &mut bodies);

    for _ in 0..steps {
        physics_pipeline.step(
            gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut impulse_joints,
            &mut multibody_joints,
            &mut soft_bodies,
            &mut ccd_solver,
            &(),
            &(),
        );

        let ball = &bodies[ball_handle];
        let position = ball.position();
        trace.push(Snapshot {
            position: (position.translation.x, position.translation.y),
            velocity: (ball.linvel().x, ball.linvel().y),
        });
    }
    trace
}

/// Simulates the ball and prints its position at each step.
fn main() {
    for (step, snapshot) in simulate(120).iter().enumerate() {
        println!(
            "step {:>3}: ball position = ({:.2}, {:.2}), velocity = ({:.2}, {:.2})",
            step,
            snapshot.position.0,
            snapshot.position.1,
            snapshot.velocity.0,
            snapshot.velocity.1,
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

    /// Runs the rapier2d example through its entry point.
    #[test]
    fn test_rapier2d_example_entry_point() {
        main();
    }
    #[test]
    fn ball_falls_under_gravity() {
        let trace = simulate(10);
        assert_eq!(trace.len(), 10);
        assert!(trace[9].position.1 < 3.0, "the ball has dropped");
        assert!(trace[9].velocity.1 < trace[0].velocity.1);
        assert!(trace.iter().all(|s| s.position.0.abs() < 1e-3));
    }

    #[test]
    fn ball_comes_to_rest_above_the_ground() {
        let trace = simulate(600);
        let last = trace.last().expect("the simulation ran");
        // The ground top is at y = -0.4 and the ball has radius 0.5.
        assert!(last.position.1 > 0.0 && last.position.1 < 0.5);
        assert!(last.velocity.1.abs() < 0.5);
    }

    #[test]
    fn zero_steps_produce_an_empty_trace() {
        assert!(simulate(0).is_empty());
    }
}
