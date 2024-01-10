fn main() {
    let s2 = String::from("hello");
    let _s3 = s2.clone();
    // `clone` deeply copies the heap data of the `String`, not just
    // the stack data
}
