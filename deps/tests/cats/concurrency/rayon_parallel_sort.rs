// ANCHOR: example
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;

fn main() {
    let mut vec = vec![String::new(); 100];

    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
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
