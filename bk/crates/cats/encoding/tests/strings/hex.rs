// ANCHOR: example
//! Example of encoding and decoding a string using hexadecimal encoding.

use data_encoding::DecodeError;
use data_encoding::HEXUPPER;

fn main() -> Result<(), DecodeError> {
    // The original string to be encoded.
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    println!("{}", encoded);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    println!("{:?}", decoded);
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
