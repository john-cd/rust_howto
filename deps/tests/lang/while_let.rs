fn main() {
    // ANCHOR: example
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // <-- while let
        println!("{}", top);
    }
    // ANCHOR_END: example
}

#[test]
fn test() {
    main();
}
