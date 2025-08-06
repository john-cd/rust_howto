mod tar_compress;
mod tar_decompress;
mod tar_strip_prefix;

#[cfg(target_family = "unix")]
fn main() -> anyhow::Result<()> {
    tar_compress::main()?;
    // the following requires the archive created above.
    tar_strip_prefix::main()?;
    tar_decompress::main()?;

    Ok(())
}

#[cfg(not(target_family = "unix"))]
fn main() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
