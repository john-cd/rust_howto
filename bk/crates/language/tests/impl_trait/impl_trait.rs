// ANCHOR_END: example
// Define an example trait and two structs that implement it:
trait Speak {
    fn speak(&self) -> String;
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}

// Define a function that accepts `impl Trait` in argument position:
fn print_speak(animal: impl Speak) {
    println!("{}", animal.speak());
}

// Define a function with `impl Trait` in return position.
// This function always returns a `Dog`,
// even though the caller only sees `impl Speak`, an opaque type that implements
// `Speak`.
fn get_animal() -> impl Speak {
    Dog
}

// This function would NOT compile if we tried to return different types.
// Use a trait object in that situation:
// fn get_animal(is_dog: bool) -> impl Speak {
//     if is_dog {
//         Dog //  This is one concrete type
//     } else {
//         Cat // Error: `if` and `else` have incompatible types
//     }
// }

fn main() {
    let my_dog = Dog;
    let my_cat = Cat;

    print_speak(my_dog); // This call is monomorphized for `Dog`.
    print_speak(my_cat); // This call is monomorphized for `Cat`.

    let some_speaker = get_animal();
    // The return value implements the `Speak` trait:
    println!("The hidden dog says: {}", some_speaker.speak());

    // // ERROR: mismatched types. Expected struct `Dog`, found opaque type
    // `impl Speak`.
    // let dog: Dog = get_animal();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
