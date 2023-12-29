fn main() {
    let s2 = String::from("hello");
    let _s3 = s2.clone(); // Deeply copy the heap data of the String, not just the stack data
}
