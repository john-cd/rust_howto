use rayon::prelude::*;

#[test]
fn test() {
    let mut v = [-5, 4, 1, -3, 2];
    v.par_sort();
    println!("{:#?}", v);
}
