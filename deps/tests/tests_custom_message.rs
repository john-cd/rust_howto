#[test]
#[should_panic]
fn custom_message() {
    let result = "Carl";

    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

#[test]
fn test() {}
