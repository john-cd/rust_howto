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
    initialization::main()?;
    insert_select::main()?;
    transactions::main()?;
    Ok(())
}

#[cfg(not(all(target_os = "linux", feature = "sqlite")))]
fn main() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
