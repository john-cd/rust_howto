fn main() {
    let s1 = String::from("hello");

    let _len = calculate_length(&s1); // pass an immutable reference to s1

    fn calculate_length(s: &str) -> usize {
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, s1 is not dropped.
}
