#[cfg(feature = "cassandra")]
mod cassandra_protocol;
#[cfg(feature = "cassandra")]
mod cdrs_tokio;

use std::sync::Mutex;

use once_cell::sync::Lazy;

pub(crate) static ENV_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "cassandra")]
    cdrs_tokio::run().await?;

    Ok(())
}
