// ANCHOR: example
fn main() {
    // Labelled block expressions must begin with a label, here `'block`.
    let result = 'block: {
        println!("Entering the labelled block.");
        if condition() {
            println!("About to break with value 1");
            // Break expressions within a labelled block expression must have a
            // label.
            break 'block 1;
        }
        if !condition() {
            println!("About to break with value 2");
            break 'block 2;
        }
        println!("About to return with value 3");
        3
    };
    println!("Result: {}", result);
}

fn condition() -> bool {
    false
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
