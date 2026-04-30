#![allow(dead_code)]
// ANCHOR: example
use parry2d::math::Isometry;
use parry2d::math::Vector;
use parry2d::query;
use parry2d::shape::Ball;
use parry2d::shape::Cuboid;

pub fn main() {
    // Define shapes
    let ball = Ball::new(1.0);
    let cuboid = Cuboid::new(Vector::new(1.0, 1.0));

    // Define positions
    let ball_pos = Isometry::translation(0.0, 0.0);
    let cuboid_pos = Isometry::translation(1.5, 0.0);

    // 1. Intersection test
    let intersects =
        query::intersection_test(&ball_pos, &ball, &cuboid_pos, &cuboid)
            .expect("Intersection test failed");
    println!("Shapes intersect: {}", intersects);

    // 2. Distance computation
    let distance = query::distance(&ball_pos, &ball, &cuboid_pos, &cuboid)
        .expect("Distance computation failed");
    println!("Distance between shapes: {}", distance);

    // 3. Contact points
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
