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
    create_tables::run()?;
    insert_query_data::run()?;
    // NOTE: `aggregate_data::run()` is a separate Postgres example that
    // operates on a different schema, so it is intentionally not executed here.
    Ok(())
}

#[cfg(not(feature = "postgres"))]
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "postgres")]
    #[test]
    fn require_external_svc() -> anyhow::Result<()> {
        let _lock = ENV_MUTEX.lock().unwrap();
        let test_url = match std::env::var("TEST_PG_URL") {
            Ok(val) => val,
            Err(_) => {
                eprintln!(
                    "Skipping Postgres integration test; set TEST_PG_URL to run."
                );
                return Ok(());
            }
        };

        unsafe {
            std::env::set_var("PG_URL", test_url);
        }
        main()?;
        Ok(())
    }
}
// [review](https://github.com/john-cd/rust_howto/issues/713)
