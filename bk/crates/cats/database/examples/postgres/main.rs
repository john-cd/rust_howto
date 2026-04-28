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
#[allow(dead_code)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(feature = "postgres")]
fn main() -> anyhow::Result<()> {
    create_tables::main()?;
    insert_query_data::main()?;
    // NOTE: `aggregate_data::main()` is a separate Postgres example that
    // operates on a different schema, so it is intentionally not executed here.
    Ok(())
}

#[cfg(feature = "postgres")]
#[test]
fn require_external_svc() -> anyhow::Result<()> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let test_url =
        std::env::var("TEST_PG_URL").expect("TEST_PG_URL must be set");
    unsafe {
        std::env::set_var("PG_URL", test_url);
    }
    main()?;
    Ok(())
}

#[cfg(not(feature = "postgres"))]
fn main() {}
// [review](https://github.com/john-cd/rust_howto/issues/713)
