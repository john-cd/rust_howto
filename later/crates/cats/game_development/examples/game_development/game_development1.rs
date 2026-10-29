// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Detecting the collision of two shapes with `parry2d`.

use parry2d::math::Pose;
use parry2d::math::Vector;
use parry2d::query;
use parry2d::shape::Ball;
use parry2d::shape::Cuboid;

/// Creates two shapes and tests whether they intersect.
fn main() {
    // Define shapes.
    let ball = Ball::new(1.0);
    let cuboid = Cuboid::new(Vector::new(1.0, 1.0));

    // Define positions.
    let ball_pos = Pose::from_translation(Vector::new(0.0, 0.0));
    let cuboid_pos = Pose::from_translation(Vector::new(1.5, 0.0));

    // 1. Intersection test.
    let intersects =
        query::intersection_test(&ball_pos, &ball, &cuboid_pos, &cuboid)
            .expect("Intersection test failed");
    println!("Shapes intersect: {}", intersects.intersecting);

    // 2. Distance computation.
    let distance = query::distance(&ball_pos, &ball, &cuboid_pos, &cuboid)
        .expect("Distance computation failed");
    println!("Distance between shapes: {}", distance.distance);

    // 3. Contact points.
    if let Some(contact) = query::contact(
        &ball_pos,
        &ball,
        &cuboid_pos,
        &cuboid,
        0.1, // Prediction distance
    )
    .expect("Contact query failed")
    {
        println!("Contact detected!");
        println!("Contact normal: {:?}", contact.normal1);
        println!("Contact depth: {}", contact.dist);
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the game development1 example through its entry point.
    #[test]
    fn test_game_development1_example_entry_point() {
        main();
    }

    #[test]
    fn overlapping_ball_and_cuboid_are_in_contact() {
        let ball = Ball::new(1.0);
        let cuboid = Cuboid::new(Vector::new(1.0, 1.0));
        let ball_pos = Pose::from_translation(Vector::new(0.0, 0.0));
        let near = Pose::from_translation(Vector::new(1.5, 0.0));

        assert!(
            query::intersection_test(&ball_pos, &ball, &near, &cuboid)
                .expect("supported shape pair")
                .intersecting
        );
        let contact = query::contact(&ball_pos, &ball, &near, &cuboid, 0.1)
            .expect("supported shape pair")
            .expect("overlapping shapes touch");
        assert!(contact.dist < 0.0, "overlap means a negative distance");
    }

    #[test]
    fn distant_shapes_do_not_intersect() {
        let ball = Ball::new(1.0);
        let cuboid = Cuboid::new(Vector::new(1.0, 1.0));
        let ball_pos = Pose::from_translation(Vector::new(0.0, 0.0));
        let far = Pose::from_translation(Vector::new(10.0, 0.0));

        assert!(
            !query::intersection_test(&ball_pos, &ball, &far, &cuboid)
                .expect("supported shape pair")
                .intersecting
        );
        let distance = query::distance(&ball_pos, &ball, &far, &cuboid)
            .expect("supported shape pair");
        assert!((distance.distance - 8.0).abs() < 1e-4);
        assert!(
            query::contact(&ball_pos, &ball, &far, &cuboid, 0.1)
                .expect("supported shape pair")
                .is_none()
        );
    }
}
