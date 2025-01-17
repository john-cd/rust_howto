// // ANCHOR: example

// // encoding and decoding private keys in PKCS#8 format.

// use std::fs::File;
// use std::io::Read;
// use std::io::Write;

// use pkcs8::DecodePrivateKey;
// use pkcs8::EncodePrivateKey;
// use rsa::BigUint;
// use rsa::RsaPrivateKey;
// use rsa::traits::PrivateKeyParts;
// use rsa::traits::PublicKeyParts;
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Generate an RSA keypair
//     let mut rng = rand::thread_rng();
//     let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
//     let public_key = PublicKey::from(&private_key);

//     // Encode the private key in PKCS#8 format
//     let pkcs8_der = private_key.to_pkcs8_der()?;

//     // Save the PKCS#8 key to a file
//     let mut file = File::create("private_key.pem")?;
//     file.write_all(&pkcs8_der.as_bytes())?;

//     // Read the PKCS#8 key from the file
//     let mut file = File::open("private_key.pem")?;
//     let mut buf = Vec::new();
//     file.read_to_end(&mut buf)?;

//     // Decode the private key from PKCS#8
//     let decoded_private_key = RsaPrivateKey::from_pkcs8_der(&buf)?;

//     // Verify that the decoded key is the same as the original
//     assert_eq!(private_key.n(), decoded_private_key.n());
//     assert_eq!(private_key.e(), decoded_private_key.e());
//     assert_eq!(private_key.d(), decoded_private_key.d());

//     println!("Successfully read and decoded PKCS#8 private key.");

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/698)
