#[test]
#[cfg(target_family = "unix")]
fn test() -> Result<(), std::io::Error> {
    use std::fs::File;

    use flate2::write::GzEncoder;
    use flate2::Compression;

    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}
