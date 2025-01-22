// ANCHOR: example
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;

fn compress() -> anyhow::Result<()> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"foo")?;
    e.write_all(b"bar")?;
    let _compressed_bytes = e.finish();
    Ok(())
}

fn decompress() -> anyhow::Result<()> {
    let mut d = GzDecoder::new("...".as_bytes());
    let mut s = String::new();
    d.read_to_string(&mut s)?;
    println!("{}", s);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    compress()?;
    decompress()?;
    Ok(())
}
// ANCHOR_END: example

#[test]
#[ignore = "not yet implemented"]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// TODO P1 write; read / write a file
