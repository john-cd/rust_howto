// ANCHOR: example
use std::fs::File;
use std::io::Read;
use std::io::Write;

// Trait to parse a private key from a PKCS#8 encoded document.
use pkcs8::DecodePrivateKey;
// Trait to serialize a private key to a PKCS#8 encoded document.
use pkcs8::EncodePrivateKey;
// Operating System's Random Number Generator
use rand::rngs::OsRng;
use rsa::RsaPrivateKey;
// Components of an RSA private key
use rsa::traits::PrivateKeyParts;

use base64ct::LineEnding;

// Encoding and decoding private keys in PKCS#8 format.
// PKCS #8 is a standard syntax for storing private key information.
// PKCS #8 private keys are typically exchanged in the PEM base64-encoded file
// format PEM is a method of encoding binary data as a string (also known as
// ASCII armor). It contains a header and a footer line (specifying the type of
// data that is encoded and showing begin/end if the data is chained together)
// and the data in the middle is the base 64 data.
fn main() -> anyhow::Result<()> {
    // Generate an RSA keypair
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;

    // Encode the private key in PKCS#8 format
    let pkcs8_pem = private_key.to_pkcs8_pem(LineEnding::LF)?;

    // Save the PKCS#8 key to a file
    let mut file = File::create("private_key.pem")?;
    file.write_all(&pkcs8_pem.as_bytes())?;
    file.flush()?;

    // Read the PKCS#8 key from the file
    let pem: String = std::fs::read_to_string("private_key.pem")?;

    // Decode the private key from PKCS#8
    let decoded_private_key = RsaPrivateKey::from_pkcs8_pem(&pem)?;

    // Verify that the decoded key is the same as the original
    assert_eq!(private_key.primes(), decoded_private_key.primes());

    println!("Successfully read and decoded PKCS#8 private key.");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
