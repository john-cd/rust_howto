mod aggregate_data;
mod create_tables;
mod insert_query_data;

#[test]
fn test() -> anyhow::Result<()> {
    create_tables::main()?;
    insert_query_data::main()?;
    // TODO aggregate_data::main()?;
    Ok(())
}
