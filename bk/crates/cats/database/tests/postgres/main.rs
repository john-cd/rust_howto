#[cfg(feature = "postgres")]
mod tokio_postgres;

#[cfg(feature = "postgres")]
#[allow(dead_code)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(feature = "postgres")]
fn main() -> anyhow::Result<()> {
    tokio_postgres::main()?;
    Ok(())
}

#[cfg(feature = "postgres")]
#[test]
fn require_external_svc() -> anyhow::Result<()> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let test_url = match std::env::var("TEST_PG_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!(
                "Skipping PostgreSQL integration test; set TEST_PG_URL to run it."
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

#[cfg(not(feature = "postgres"))]
fn main() {}
