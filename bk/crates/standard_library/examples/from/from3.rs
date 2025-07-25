// ANCHOR: example
//! Parsing from raw bytes.

#[derive(Debug)]
struct Message(String);

// Implement `From` to convert from a `u8` slice to our custom struct.
impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Self {
        Message(String::from_utf8_lossy(bytes).into())
    }
}

fn main() {
    let raw = b"Hello world";
    let msg: Message = raw.into(); // Use `From<&[u8]>`.
    println!("{msg:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
