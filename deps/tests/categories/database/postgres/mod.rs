mod aggregate_data;
mod create_tables;
mod insert_query_data;
// TODO mod deadpool2;

#[test]
fn test() -> anyhow::Result<()> {
    create_tables::main()?;
    insert_query_data::main()?;
    // TODO
    // aggregate_data::main()?;
    Ok(())
}
