use anyhow::bail;
use anyhow::Result;

fn main() -> Result<()> {
    let root_path = std::fs::canonicalize("..")?;
    let deps_path = root_path.join("deps/");
    if !deps_path.exists() {
        bail!("The folder {:?} does not exist.", deps_path);
    }
    let deps = utils::dependencies::get_dependencies(&deps_path)?;
    for d in deps {
        println!("{:?}", d);
    }

    Ok(())
}
