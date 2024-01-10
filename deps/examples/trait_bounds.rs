use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

// Trait bounds: the `print_hash` function is generic over an unknown
// type `T`, but requires that `T` implements the `Hash` trait.
fn print_hash<T: Hash>(t: &T) {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    println!("The hash is {:x}", hasher.finish());
}
struct Pair<A, B> {
    first: A,
    second: B,
}

// Generics make it possible to implement a trait conditionally.
// Here, the Pair type implements Hash if, and only if,
// its components do.
impl<A: Hash, B: Hash> Hash for Pair<A, B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first.hash(state);
        self.second.hash(state);
    }
}

fn main() {
    let p = Pair {
        first: 1,
        second: "2",
    };
    print_hash(&p);
}
