// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Forward and inverse kinematics of a planar arm with `k`, the kinematics
//! library underneath OpenRR. No robot, simulator or GUI is needed.
//!
//! The `openrr` umbrella crate adds robot clients, planners and tools, but it
//! is experimental and pulls in native libraries and affected dependencies.
//! `k` covers the hardware-free core: load a URDF description, compute where
//! the end of the arm is, and solve for joint angles that reach a target.

use k::prelude::*;

/// A planar arm with two revolute joints and a fixed tip: 1.0 m to the elbow
/// and 0.5 m beyond it.
const URDF: &str = r#"
<robot name="planar_arm">
  <link name="base"/>
  <link name="upper"/>
  <link name="tip_base"/>
  <link name="tip"/>
  <joint name="shoulder" type="revolute">
    <parent link="base"/>
    <child link="upper"/>
    <origin xyz="0 0 0" rpy="0 0 0"/>
    <axis xyz="0 0 1"/>
    <limit lower="-3.14" upper="3.14" effort="10" velocity="1"/>
  </joint>
  <joint name="elbow" type="revolute">
    <parent link="upper"/>
    <child link="tip_base"/>
    <origin xyz="1.0 0 0" rpy="0 0 0"/>
    <axis xyz="0 0 1"/>
    <limit lower="-3.14" upper="3.14" effort="10" velocity="1"/>
  </joint>
  <joint name="tip_mount" type="fixed">
    <parent link="tip_base"/>
    <child link="tip"/>
    <origin xyz="0.5 0 0" rpy="0 0 0"/>
  </joint>
</robot>
"#;

/// Returns the world position `[x, y, z]` of the `tip` link after moving the
/// joints to `angles` (forward kinematics). The tip is 0.5 m past the elbow.
///
/// # Errors
///
/// Returns an error if the URDF is invalid or the number of angles is wrong.
fn tip_position(angles: &[f64]) -> anyhow::Result<[f64; 3]> {
    let robot = urdf_rs::read_from_string(URDF)?;
    let chain = k::Chain::<f64>::from(&robot);
    chain.set_joint_positions(angles)?;
    chain.update_transforms();
    let tip = chain
        .find_link("tip")
        .ok_or_else(|| anyhow::anyhow!("no link named `tip`"))?;
    let position = tip.world_transform().ok_or_else(|| {
        anyhow::anyhow!("the transform of `tip` is not computed")
    })?;
    let t = position.translation.vector;
    Ok([t.x, t.y, t.z])
}

/// Solves for joint angles that move the end of the arm to a target (inverse
/// kinematics) and returns the position it reaches.
///
/// # Errors
///
/// Returns an error if the URDF is invalid or the solver does not converge.
fn reach(target_xy: [f64; 2]) -> anyhow::Result<[f64; 3]> {
    let robot = urdf_rs::read_from_string(URDF)?;
    let chain = k::Chain::<f64>::from(&robot);
    // Start from a bent elbow: a fully stretched arm is a singular pose, where
    // the Jacobian cannot be inverted.
    chain.set_joint_positions(&[0.3, 0.6])?;
    chain.update_transforms();
    let tip = chain
        .find_link("tip")
        .ok_or_else(|| anyhow::anyhow!("no link named `tip`"))?;
    // Choose the joints from the base to the tip, then move the target.
    let arm = k::SerialChain::from_end(tip);
    let mut target = tip.world_transform().ok_or_else(|| {
        anyhow::anyhow!("the transform of `tip` is not computed")
    })?;
    target.translation.vector.x = target_xy[0];
    target.translation.vector.y = target_xy[1];
    // The arm has two joints, so only the position can be constrained.
    let mut solver = k::JacobianIkSolver::default();
    solver.allowable_target_distance = 1e-3;
    let constraints = k::Constraints {
        rotation_x: false,
        rotation_y: false,
        rotation_z: false,
        position_z: false,
        ..Default::default()
    };
    solver.solve_with_constraints(&arm, &target, &constraints)?;
    chain.update_transforms();
    let reached = tip.world_transform().ok_or_else(|| {
        anyhow::anyhow!("the transform of `tip` is not computed")
    })?;
    let t = reached.translation.vector;
    Ok([t.x, t.y, t.z])
}

/// Prints the tip position for a pose, then solves for a new target.
fn main() -> anyhow::Result<()> {
    let [x, y, _] = tip_position(&[0.0, std::f64::consts::FRAC_PI_2])?;
    println!("Forward kinematics: tip at ({x:.3}, {y:.3})");
    let [x, y, _] = reach([0.8, 0.9])?;
    println!("Inverse kinematics: tip reached ({x:.3}, {y:.3})");
    Ok(())
}
// ANCHOR_END: example

/// Runs the kinematics example.
///
/// # Errors
///
/// Returns an error if the kinematics computation fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// With both joints at zero the arm is stretched along the x axis.
    #[test]
    fn straight_arm_reaches_one_and_a_half_meters() -> anyhow::Result<()> {
        let [x, y, _] = tip_position(&[0.0, 0.0])?;
        assert!((x - 1.5).abs() < 1e-9 && y.abs() < 1e-9);
        Ok(())
    }

    /// Bending the elbow by 90 degrees puts the tip 0.5 m to the side.
    #[test]
    fn elbow_bend_moves_the_tip_sideways() -> anyhow::Result<()> {
        let [x, y, _] = tip_position(&[0.0, std::f64::consts::FRAC_PI_2])?;
        assert!((x - 1.0).abs() < 1e-9 && (y - 0.5).abs() < 1e-9);
        Ok(())
    }

    /// The solver finds joint angles that bring the tip close to the target.
    #[test]
    fn inverse_kinematics_reaches_a_reachable_target() -> anyhow::Result<()> {
        let [x, y, _] = reach([0.8, 0.9])?;
        assert!((x - 0.8).abs() < 5e-3 && (y - 0.9).abs() < 5e-3);
        Ok(())
    }

    /// A wrong number of angles is an error.
    #[test]
    fn wrong_joint_count_is_rejected() {
        assert!(tip_position(&[0.0]).is_err());
    }

    /// Runs the example through its entry point.
    #[test]
    fn test_openrr_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
