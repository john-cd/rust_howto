// ANCHOR_END: example
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

// `impl Trait` in argument position:
fn print_speak(animal: impl Speak) {
    println!("{}", animal.speak());
}

// `impl Trait` in return position.
// This function always returns a Dog, even though the caller only sees `impl
// Speak`
fn get_a_dog() -> impl Speak {
    Dog
}

// This function would NOT compile if we tried to return different types:
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

    print_speak(my_dog); // Monomorphized for Dog.
    print_speak(my_cat); // Monomorphized for Cat.

    let some_speaker = get_a_dog();
    println!("The hidden dog says: {}", some_speaker.speak());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
