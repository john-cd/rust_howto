// ANCHOR: example
#![doc(html_playground_url = "https://playground.example.com/")]

/// This is a simple example to demonstrate the use of the
/// `html_playground_url` attribute.
///
/// You can click on the "Run" button in the documentation to execute this code
/// in the playground.
fn main() {
    println!(
        "Note the above doc attribute is an _inner_ attribute that starts with #!"
    );
    println!("It should be place at the top of your crate.")
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
