use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|i| i * i).sum()
}

fn increment_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(|p| *p += 1);
}

fn main() {
    let mut v = [1, 2, 3];
    increment_all(&mut v[..]);
    println!("{}", sum_of_squares(&v[..]));
}
