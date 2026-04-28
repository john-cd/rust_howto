#![allow(dead_code)]
// ANCHOR: example
use rsa::RsaPrivateKey;
use rand_core_06::OsRng;

fn main() -> anyhow::Result<()> {
    let mut rng = OsRng;
    let priv_key = RsaPrivateKey::new(&mut rng, 2048)?;
    println!("RSA private key generated.");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
