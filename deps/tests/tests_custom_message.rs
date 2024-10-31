fn main() {
    let result = "Carl";

    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

#[should_panic]
#[test]
fn test() {
    main();
}
