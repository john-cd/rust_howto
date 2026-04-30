mod tar_compress;
mod tar_decompress;
mod tar_strip_prefix;

#[cfg(target_family = "unix")]
fn main() -> anyhow::Result<()> {
    use std::fs;
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    tar_compress::run()?;
    // the following requires the archive created above.
    tar_strip_prefix::run()?;
    tar_decompress::run()?;

    Ok(())
}

#[cfg(not(target_family = "unix"))]
fn main() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
