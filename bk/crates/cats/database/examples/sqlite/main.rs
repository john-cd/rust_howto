#[cfg(target_os = "linux")]
mod initialization;
#[cfg(target_os = "linux")]
mod insert_select;
#[cfg(target_os = "linux")]
mod transactions;

#[cfg(target_os = "linux")]
fn main() -> anyhow::Result<()> {
    let _ = std::fs::remove_file("temp/cats.db");
    initialization::main()?;
    insert_select::main()?;
    transactions::main()?;
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() -> anyhow::Result<()> {
    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
