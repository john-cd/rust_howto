#![cfg(target_family = "unix")]

mod tar_compress;
mod tar_decompress;
mod tar_strip_prefix;

fn main() -> anyhow::Result<()> {
    tar_compress::main()?;
    // the following requires the archive created above.
    tar_strip_prefix::main()?;
    tar_decompress::main()?;

    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    main();
}
