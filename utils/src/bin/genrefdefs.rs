use std::path::Path;

use anyhow::bail;
use anyhow::Result;

fn main() -> Result<()> {
    let root_path = std::fs::canonicalize("..")?;
    let deps_path = root_path.join("deps/");
    if !Path::new(&deps_path).exists() {
        let msg = format!("The folder {:?} does not exist.", deps_path);
        bail!("{}", msg);
    }
    let deps = utils::dependencies::get_dependencies(&deps_path)?;
    for d in deps {
        println!("{:?}", d);
    }

    Ok(())
}
