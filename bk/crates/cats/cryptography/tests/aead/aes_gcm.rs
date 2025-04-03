// ANCHOR: example
// Algorithm
use aes_gcm::Aes256Gcm; // Use Aes128Gcm or Aes256Gcm, respectively.
// Cryptokey:
use aes_gcm::Key;
// A 'nonce' is an arbitrary (often random) number
// that must be used just once
// in a cryptographic communication.
use aes_gcm::Nonce;
// Trait that provides `encrypt` and `decrypt`:
use aes_gcm::aead::Aead;
// Trait that provides `generate_key`:
use aes_gcm::aead::KeyInit;
// The operating-system's random data source:
use aes_gcm::aead::OsRng;

fn main() {
    // Generate a random 256-bit key
    let key: Key<Aes256Gcm> = Aes256Gcm::generate_key(&mut OsRng);

    // Generate a random 96-bit nonce.
    let nonce = Nonce::from_slice(b"unique nonce");

    // Create the cipher...
    let cipher = Aes256Gcm::new(&key);

    let plaintext = b"Secret message";

    // ...that will encrypt the secret message.
    // To authenticate additional Associated Data,
    // apply the same method than in the AES-GCM-SIV example
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .expect("encryption failure!");
    println!("Ciphertext: {:?}", ciphertext);

    let decrypted_ciphertext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");

    assert_eq!(plaintext, decrypted_ciphertext.as_slice());

    println!(
        "Decrypted Ciphertext: {}",
        String::from_utf8(decrypted_ciphertext).unwrap()
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
