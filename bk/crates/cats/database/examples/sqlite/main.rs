#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod initialization;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod insert_select;
#[cfg(all(target_os = "linux", feature = "sqlite"))]
mod transactions;

#[cfg(all(target_os = "linux", feature = "sqlite"))]
fn main() -> anyhow::Result<()> {
    let _ = std::fs::remove_file("temp/cats.db");
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
