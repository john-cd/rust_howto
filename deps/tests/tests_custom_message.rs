#[test]
#[should_panic]
fn test() {
    let result = "Carl";

    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
