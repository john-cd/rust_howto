// ANCHOR: example
use rand::distr::Distribution;
use rand::distr::Uniform;

fn main() {
    let mut rng = rand::rng();
    let die = Uniform::try_from(1..7).unwrap();

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
