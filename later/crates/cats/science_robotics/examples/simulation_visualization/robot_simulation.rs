// ANCHOR: example
//! Drop a robot chassis onto a floor with Rapier's headless 2D physics
//! pipeline.
//!
//! A physics engine advances bodies in small time steps, applying gravity and
//! resolving collisions. Simulating a robot first lets you test control code
//! without risking hardware. Rapier is a Rust physics engine; "headless" means
//! no window is opened.

use rapier2d::prelude::*;

const GROUND_HALF_HEIGHT: f32 = 0.1;
const CHASSIS_HALF_HEIGHT: f32 = 0.2;
const SIMULATION_STEPS: usize = 600;

/// The robot chassis state after the physics simulation settles.
#[derive(Debug)]
struct ChassisState {
    center_height: f32,
    speed: f32,
}

/// Simulates a rectangular robot chassis falling onto a static floor.
fn simulate_chassis_landing() -> ChassisState {
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

    let ground = RigidBodyBuilder::fixed()
        .translation(Vector::new(0.0, -GROUND_HALF_HEIGHT))
        .build();
    let ground_handle = bodies.insert(ground);
    let ground_collider = ColliderBuilder::cuboid(5.0, GROUND_HALF_HEIGHT)
        .friction(0.9)
        .build();
    colliders.insert_with_parent(ground_collider, ground_handle, &mut bodies);

    let chassis = RigidBodyBuilder::dynamic()
        .translation(Vector::new(0.0, 2.0))
        .linear_damping(1.0)
        .angular_damping(1.0)
        .build();
    let chassis_handle = bodies.insert(chassis);
    let chassis_collider = ColliderBuilder::cuboid(0.35, CHASSIS_HALF_HEIGHT)
        .friction(0.9)
        .restitution(0.0)
        .build();
    colliders.insert_with_parent(chassis_collider, chassis_handle, &mut bodies);

    for _ in 0..SIMULATION_STEPS {
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
    }

    let chassis = &bodies[chassis_handle];
    ChassisState {
        center_height: chassis.position().translation.y,
        speed: chassis.linvel().length(),
    }
}

/// Runs the headless chassis simulation and prints the final state.
fn main() {
    let state = simulate_chassis_landing();
    println!(
        "Robot chassis center: {:.3} m; speed: {:.3} m/s",
        state.center_height, state.speed
    );
}
// ANCHOR_END: example

/// Runs the robot simulation example through its entry point.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::CHASSIS_HALF_HEIGHT;
    use super::main;
    use super::simulate_chassis_landing;

    /// Verifies that the chassis settles on the floor with negligible speed.
    #[test]
    fn chassis_settles_on_the_floor() {
        let state = simulate_chassis_landing();

        assert!((state.center_height - CHASSIS_HALF_HEIGHT).abs() < 0.05);
        assert!(state.speed < 0.05);
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
