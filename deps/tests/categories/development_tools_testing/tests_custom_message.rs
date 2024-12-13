// ANCHOR: example
fn main() {
    let result = "Carl";

    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
// ANCHOR_END: example

#[should_panic]
#[test]
fn test() {
    main();
}
// TODO P0 println!("{}", );
