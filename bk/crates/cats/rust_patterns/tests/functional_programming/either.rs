// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
use either::Either;

// Function that can return two different types
fn process_input(value: i32) -> Either<String, i32> {
    if value > 0 {
        Either::Right(value * 2)
    } else {
        Either::Left(format!("Invalid input: {}", value))
    }
}

fn main() {
    let result1 = process_input(5);
    let result2 = process_input(-3);

    match result1 {
        Either::Left(ref err) => println!("Error: {}", err),
        Either::Right(val) => println!("Processed value: {}", val),
    }

    match result2 {
        Either::Left(err) => println!("Error: {}", err),
        Either::Right(val) => println!("Processed value: {}", val),
    }

    // Transforming Either values
    let _mapped_result =
        result1.map_left(|s| s.to_uppercase()).map_right(|n| n + 10);
}

#[test]
fn test() {
    main();
}
// [finish example](https://github.com/john-cd/rust_howto/issues/1317)
