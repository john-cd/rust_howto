use anyhow::bail;
use anyhow::Result;

fn main() -> Result<()> {
    let root_path = std::fs::canonicalize("..")?;
    let deps_path = root_path.join("deps/");
    if !deps_path.exists() {
        bail!("The folder {:?} does not exist.", deps_path);
    }
    utils::fs::create_dir("/code/book/temp/")?;
    let refdef_dest_path = "/code/book/temp/merged_ref_defs.md";

    utils::generate_refdefs_to(&deps_path, "/code/src", refdef_dest_path)?;

    Ok(())
}
