use num::bigint::BigInt;
use num::bigint::ToBigInt;

fn factorial(x: i32) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..=x {
            factorial *= i;
        }
        factorial
    } else {
        panic!("Failed to calculate factorial!");
    }
}

#[test]
fn test() {
    println!("{}! equals {}", 100, factorial(100));
}
