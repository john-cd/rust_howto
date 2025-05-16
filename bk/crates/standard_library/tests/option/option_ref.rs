// ANCHOR: example
fn as_ref() {
    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("We still can print `text`: {text:?}. Length: {:?}", text_length);
}

fn as_mut() {
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => {
            *v = 42;
            println!("x is Some({v})");
        }
        None => {
            println!("x is None");
        }
    }
    assert_eq!(x, Some(42));
}

fn main() {
    as_ref();
    as_mut();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
