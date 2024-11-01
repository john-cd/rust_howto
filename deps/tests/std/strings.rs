// ANCHOR: example
fn main() {
    // `String` is Unicode, not ASCII
    let mut s1 = String::from("hello");
    s1.push_str(", world!"); // `String` can be mutated
    s1.clear(); // Empties the String, making it equal to ""

    // Alternative initialization from string literals
    // `to_string` is available on any type that implements
    // the Display trait
    let s2 = "initial contents".to_string();

    // Concatenation: note s1 has been moved here and can no longer
    // be used afterwards
    let s3 = s1 + &s2;
    // ERROR let s = format!("{s1}-{s2}-{s3}");

    // String slice -  contains the first 4 bytes of the string.
    let _s: &str = &s3[0..4];
    // Caution: If we were to try to slice only part of a unicode
    // character’s bytes, Rust would panic at runtime.

    // Iteration
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
