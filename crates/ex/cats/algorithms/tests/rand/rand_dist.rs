// // ANCHOR: example
// use rand::distr::Distribution;
// use rand_distr::Normal;
// use rand_distr::NormalError;

// fn main() -> Result<(), NormalError> {
//     let mut rng = rand::rng();
//     let normal = Normal::new(2.0, 3.0)?;
//     let v = normal.sample(&mut rng);
//     println!("{} is from a N(2, 9) distribution", v);
//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// [ P0 fix version conflict for rand_distr](https://github.com/john-cd/rust_howto/issues/1008)
