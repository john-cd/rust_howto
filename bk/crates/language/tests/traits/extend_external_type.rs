// ANCHOR: example
/// A trait for types that can be hashed to a `u64`.
trait MyHash {
    /// Returns a hash of the object.
    fn myhash(&self) -> u64;
}

/// Implement the `MyHash` trait for preexisting type `i64`,
/// which is defined in the standard library.
impl MyHash for i64 {
    fn myhash(&self) -> u64 {
        *self as u64
    }
}

fn main() {
    let x = 1i64;
    // Since `i64` implements the `MyHash` trait, we can call its method.
    println!("{}", x.myhash());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
