// // ANCHOR: example
// use ed25519_dalek::Signature;
// use ed25519_dalek::SigningKey;
// use ed25519_dalek::Verifier;
// use ed25519_dalek::VerifyingKey;
// use rand::rngs::OsRng;
// use ed25519_dalek::Signer;

// // `ed25519` key generation, signing, and verification.

// fn main() -> anyhow::Result<()> {
//     // First, we need to generate a Keypair, which includes both public and
//     // secret halves of an asymmetric key. To do so, we need a
// cryptographically     // secure pseudorandom number generator (CSPRNG). For
// this example, we'll     // use the operating system's builtin PRNG:

//     let mut csprng = OsRng;
//     let signing_key: SigningKey = SigningKey::generate(&mut csprng);

//     // Message to be signed
//     let message = b"Hello, world!";

//     // Sign the message
//     let signature: Signature = signing_key.sign(message);

//     // Verify the signature
//     assert!(signing_key.verify(message, &signature).is_ok());

//     // Anyone else, given the public half of the signing_key can also easily
//     // verify this signature:

//     let verifying_key: VerifyingKey = signing_key.verifying_key();
//     assert!(verifying_key.verify(message, &signature).is_ok());

//     // VerifyingKeys, SecretKeys, SigningKeys, and Signatures can be
// serialised     // into byte-arrays by calling .to_bytes().

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/694)
// // conflicts with rand = "0.9.0-beta.1"
