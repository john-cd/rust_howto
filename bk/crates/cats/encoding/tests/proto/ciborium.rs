// ANCHOR: example
use std::io::Cursor;

use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
    age: u32,
    is_active: bool,
}

fn main() -> anyhow::Result<()> {
    // Create a sample data object
    let data = MyData {
        name: "Alice".to_string(),
        age: 30,
        is_active: true,
    };

    // Serialize to CBOR
    let mut buf = Vec::new();
    into_writer(&data, &mut buf)?;

    // Deserialize from CBOR
    let mut cursor = Cursor::new(&buf[..]);
    let deserialized_data: MyData = from_reader(&mut cursor)?;

    // Print the deserialized data
    println!("{:?}", deserialized_data);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [rewiew NOW](https://github.com/john-cd/rust_howto/issues/1042)
