#![allow(dead_code)]

// Put unit tests in the same file than the main code

#[cfg(test)] // only for unit tests
mod tests {
    // Access to all objects in the parent module,
    // which contains the main code
    use super::*;

    // Test functions must be free, monomorphic functions that take no
    // arguments, and commonly return () or Result<T, E> where T:
    // Termination, E: Debug
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        // or assert_eq!(result, some_const);
        // or assert_ne!
    }

    // This test passes if the code inside the function panics;
    // It fails if the code inside the function doesn’t panic.
    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }

    // With Result
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) // Pass if OK
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // Code that takes an hour to run
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, _another: &Rectangle) -> bool {
        true
    }
}

fn main() {}
