// ANCHOR: example
use rand::Rng;
use rand::distr::Alphanumeric;
use rand::rng;
use rayon::prelude::*;

fn main() {
    let mut vec = vec![String::new(); 100];

    vec.par_iter_mut().for_each(|p| {
        let mut rng = rng();
        *p = (0..5).map(|_| rng.sample(Alphanumeric) as char).collect();
    });
    vec.par_sort_unstable();
    println!("{:?}", vec);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
