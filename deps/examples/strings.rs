fn main() {
    let mut s1 = String::from("hello"); // Unicode, not ASCII
    s1.push_str(", world!"); // This kind of string can be mutated
    s1.clear(); // this empties the String, making it equal to ""

    let s2 = "initial contents".to_string(); // alternative init from string literals - available on any type that implements the Display trait

    let s3 = s1 + &s2; // concat: note s1 has been moved here and can no longer be used
                       // ERROR let s = format!("{s1}-{s2}-{s3}");

    let _s: &str = &s3[0..4]; // string slice -  contains the first 4 bytes of the string.
                              // Caution: If we were to try to slice only part of a unicode character’s bytes, Rust would panic at runtime

    // Iterate
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
