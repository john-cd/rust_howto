// ANCHOR: example
use rand::Rng;
use rand::distributions::Distribution;
use rand::distributions::Standard;

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.r#gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.r#gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.r#gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
