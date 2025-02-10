// ANCHOR: example
use rand::Rng;
use rand::distr::Distribution;
use rand::distr::StandardUniform;

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

// Required by:
// pub fn random<T>() -> T
// where
//     StandardUniform: Distribution<T>,
impl Distribution<Point> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.random::<(i32, i32)>();
        Point { x, y }
    }
}

fn main() {
    let mut rng = rand::rng();
    let rand_tuple = rng.random::<(i32, bool, f64)>();
    println!("Random tuple: {:?}", rand_tuple);

    let rand_point: Point = rng.random();
    println!("Random Point: {:?}", rand_point);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
