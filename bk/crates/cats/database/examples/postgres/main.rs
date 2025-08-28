#[cfg(feature = "postgres")]
mod aggregate_data;
#[cfg(feature = "postgres")]
mod cornucopia;
#[cfg(feature = "postgres")]
mod create_tables;
#[cfg(feature = "postgres")]
mod insert_query_data;
#[cfg(feature = "postgres")]
mod tokio_postgres;

#[cfg(feature = "postgres")]
fn main() -> anyhow::Result<()> {
    create_tables::main()?;
    insert_query_data::main()?;
    // FIXME aggregate_data::main()?;
    Ok(())
}

#[cfg(feature = "postgres")]
#[test]
fn require_external_svc() -> anyhow::Result<()> {
    main()?;
    Ok(())
}

#[cfg(not(feature = "postgres"))]
fn main() {}
// [review](https://github.com/john-cd/rust_howto/issues/713)
