// ANCHOR: example
#![doc(html_playground_url = "https://playground.example.com/")]

fn main() {
    println!("Note the above is an _inner_ attribute that starts with #!");
    println!("It should be place at the top of your crate.")
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
