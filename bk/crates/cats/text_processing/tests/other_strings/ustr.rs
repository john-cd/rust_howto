// ANCHOR: example
//! String interning example.
//!
//! A `Ustr` (Unique str) is a lightweight handle representing a static,
//! immutable entry in a global string cache, allowing for:
//!
//! - Fast string assignment and comparisons.
//! - Efficient storage. Only one copy of the string is held in memory, and
//!   getting access to it is just a pointer indirection.
//! - Fast hashing ‒ the precomputed hash is stored with the string.
//! - Fast FFI ‒ the string is stored with a terminating null byte, so can be
//!   passed to C directly without using `CString`.
//!
//! The downside is no strings are ever freed, so if you're creating lots of
//! strings, you might run out of memory.

use ustr::Ustr;
use ustr::ustr;

fn main() {
    // Creating `Ustr` instances.
    // `Ustr` is the main type representing an interned string.
    // Create it with the `From` trait.
    let s1: Ustr = Ustr::from("hello");
    // You may also use `ustr`, a convenience function to create a `Ustr` from a
    // string slice (`&str`).
    let s2: Ustr = ustr("world");

    // Getting the string content:
    println!("s1 content: {}", s1);

    // Comparing `Ustr` instances (cheap operation):
    println!("s1 == s2: {}", s1 == s2); // Output: s1 == s2: false.

    // Interning:
    // `s1` and `s3` are the same. `s3` will be interned and share the same
    // underlying data as `s1`.
    let s3: Ustr = ustr("hello");
    println!("s1 == s3: {}", s1 == s3); // Output: s1 == s3: true.
    assert_eq!(s1, s3);
    // `s1` and `s3` point to the same underlying data:
    println!("s1's pointer: {:p}", s1.as_ptr());
    println!("s3's pointer: {:p}", s3.as_ptr());
    assert_eq!(s1.as_ptr(), s3.as_ptr());
    // The cache has only two entries:
    assert_eq!(ustr::num_entries(), 2);

    // Common operations:

    // Checking the length.
    println!("s1 length: {}", s1.len());

    // `Ustr` itself doesn't directly support concatenation in a way that
    // creates a new interned string easily for arbitrary combinations.
    // You'd typically convert to `String` or `&str` for complex manipulations
    // and then re-ustr if needed.
    let combined_str = format!("{} {}", s1, s2);
    let s4: Ustr = ustr(&combined_str);
    println!("s4 content: {}", s4);

    // You can a `Ustr` pass straight to FFI.
    // `as_char_ptr` gets the cached string as a C nul-terminated `char*`.
    let len = unsafe { libc::strlen(s1.as_char_ptr()) };
    assert_eq!(len, 5);

    // Using `Ustr` in a collection (e.g., a `HashMap` key).
    use std::collections::HashMap;
    let mut map: HashMap<Ustr, i32> = HashMap::new();
    map.insert(ustr("apple"), 1);
    map.insert(ustr("banana"), 2);
    map.insert(ustr("apple"), 3); // Overwrites the previous value for "apple".

    println!("Value for 'apple': {:?}", map.get(&ustr("apple"))); // Output: Value for 'apple': Some(3).
    println!("Value for 'banana': {:?}", map.get(&ustr("banana"))); // Output: Value for 'banana': Some(2).

    // For best performance when using `Ustr` as key for a `HashMap` or
    // `HashSet`, you'll want to use the precomputed hash. Use the `UstrMap`
    // or `UstrSet` type aliases:
    use ustr::UstrMap;

    // The key type is always `Ustr`.
    let mut map: UstrMap<usize> = UstrMap::default();
    map.insert(s1, 17);
    assert_eq!(*map.get(&s1).unwrap(), 17);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
