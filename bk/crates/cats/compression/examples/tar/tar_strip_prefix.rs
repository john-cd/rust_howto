#![allow(dead_code)]
// ANCHOR: example
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;
use flate2::read::GzDecoder;
use tar::Archive;

/// This example demonstrates how to extract a tar archive while stripping a
/// common prefix from the paths of the extracted files.
///
/// It opens a gzipped tar archive, iterates over its entries, strips the
/// "bundle/logs" prefix, and unpacks the files to the new paths.
fn main() -> Result<()> {
    let file = File::open("temp/archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "bundle/logs";
    let prefix_path = Path::new(prefix);

    println!("Extracted the following files:");
    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let path = entry.path()?;
        let stripped =
            strip_prefix_path(&path, prefix_path).ok_or_else(|| {
                anyhow!(
                    "archive entry path does not start with prefix: {}",
                    path.display()
                )
            })?;

        if stripped.as_os_str().is_empty() {
            continue;
        }

        entry.unpack(&stripped)?;
        println!("> {}", stripped.display());
    }

    Ok(())
}

fn strip_prefix_path(path: &Path, prefix: &Path) -> Option<PathBuf> {
    path.strip_prefix(prefix).ok().map(|p| p.to_path_buf())
}
// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::time::SystemTime;

    use flate2::Compression;
    use flate2::write::GzEncoder;

    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        let work_dir = std::env::temp_dir().join(format!(
            "rust_howto_tar_strip_prefix_{}_{:?}_{}_{}",
            std::process::id(),
            std::thread::current().id(),
            std::path::Path::new(file!())
                .file_name()
                .unwrap()
                .to_string_lossy(),
            SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));
        if work_dir.exists() {
            fs::remove_dir_all(&work_dir)?;
        }
        fs::create_dir_all(work_dir.join("bundle/logs"))?;
        let source_file = work_dir.join("bundle/logs/example.txt");
        fs::write(&source_file, b"hello world")?;
        fs::create_dir_all(work_dir.join("temp"))?;

        let archive_path = work_dir.join("temp/archive.tar.gz");
        let archive_file = File::create(&archive_path)?;
        let enc = GzEncoder::new(archive_file, Compression::default());
        let mut tar_builder = tar::Builder::new(enc);
        tar_builder
            .append_path_with_name(&source_file, "bundle/logs/example.txt")?;
        let enc = tar_builder.into_inner()?;
        enc.finish()?;

        let _cwd_lock = crate::CWD_LOCK.lock().unwrap();
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(&work_dir)?;
        let result = main();
        std::env::set_current_dir(original_dir)?;

        result?;
        assert!(work_dir.join("example.txt").exists());
        fs::remove_dir_all(&work_dir)?;
        Ok(())
    }
}
// [review; tar_strip_prefix.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/256)
