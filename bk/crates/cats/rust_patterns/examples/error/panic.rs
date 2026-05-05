#![allow(dead_code)]
// ANCHOR: example
/// This example demonstrates a simple panic.
/// When executed, it will cause the program to terminate abruptly.
fn main() {
    println!(
        "panic example: calling panic! causes immediate program termination"
    );
    panic!("Crash and burn");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[should_panic]
    #[test]
    fn test_main() {
        main();
    }
}
