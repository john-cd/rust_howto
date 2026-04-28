#![allow(dead_code)]
// ANCHOR: example
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand_core_06::OsRng;

fn main() -> anyhow::Result<()> {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
    let pub_key = RsaPublicKey::from(&priv_key);

    let message = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &message[..])?;

    println!("Encrypted: {:?}", enc_data);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
