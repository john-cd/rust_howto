// ANCHOR: example
use blake3::Hasher;

fn main() {
    // Example 1: Hashing a simple string
    let input_string = "Hello, world!";
    let hash = blake3::hash(input_string.as_bytes());
    println!("Hash of '{}': {}", input_string, hash);

    // Example 2: Incremental hashing
    let mut hasher = Hasher::new();
    hasher.update(b"The quick brown ");
    hasher.update(b"fox jumps over ");
    hasher.update(b"the lazy dog.");
    let hash2 = hasher.finalize();
    println!("Incremental hash: {}", hash2);

    // Example 3: Hashing a larger byte array
    let large_data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect(); // Example 1KB data
    let hash3 = blake3::hash(&large_data);
    println!("Hash of 1KB data: {}", hash3);

    // Example 4: Using a key for keyed hashing (KMAC)
    let key: &[u8; 32] = b"mysecretkeymysecretkey__________";
    let mut hasher_keyed = blake3::Hasher::new_keyed(key);
    hasher_keyed.update(b"Message to be keyed hashed");
    let keyed_hash = hasher_keyed.finalize();
    // OR let mac = blake3::keyed_hash(key, b"foo");
    println!("Keyed hash: {}", keyed_hash);

    // Example 5: Deriving a key using a context string
    // Given cryptographic key material of any length and a context string of
    // any length, `derive_key` outputs a 32-byte derived subkey.
    // The context string should be hardcoded, globally unique, and
    // application-specific. A good default format for such strings is
    // "[application] [commit timestamp] [purpose]", e.g., "example.com
    // 2019-12-25 16:18:03 session tokens v1".
    let context = "My application context";
    let derived_key = blake3::derive_key(context, b"Input key material");
    println!("Derived Key: {:?}", derived_key);

    // Example 6: Extended output.
    let mut output = [0u8; 1000];
    let hasher = blake3::Hasher::new();
    // Finalize the hash state and return an OutputReader, which can supply any
    // number of output bytes.
    let mut output_reader = hasher.finalize_xof();
    output_reader.fill(&mut output); // OutputReader also implements Read and Seek.
    println!("Output: {:x?}", output);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
