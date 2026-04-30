#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod initialization;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod insert_select;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod transactions;

#[cfg(all(target_os = "linux", feature = "sqlite"))]
fn main() -> anyhow::Result<()> {
    use std::fs;
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    let _ = fs::remove_file("temp/cats.db");
    initialization::run()?;
    insert_select::run()?;
    transactions::run()?;
    Ok(())
}

#[cfg(not(all(target_os = "linux", feature = "sqlite")))]
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
