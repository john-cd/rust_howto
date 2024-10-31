use rand::distributions::Distribution;
use rand::distributions::Uniform;

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

#[test]
fn test() {
    main();
}
