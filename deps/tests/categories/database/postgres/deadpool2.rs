// ANCHOR: example
use tokio_postgres::NoTls;
use dotenvy::dotenv;
use deadpool_postgres::Runtime;

#[derive(Debug, serde::Deserialize)]
struct Config {
    pub pg: deadpool_postgres::Config,
}

// Hydrate the configuration from environment variables e.g.
// add to your .env file
// PG__HOST=pg.example.com
// PG__USER=john_doe
// PG__PASSWORD=topsecret
// PG__DBNAME=example
// PG__POOL__MAX_SIZE=16
// PG__POOL__TIMEOUTS__WAIT__SECS=5
// PG__POOL__TIMEOUTS__WAIT__NANOS=0
impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cfg = Config::from_env()?;
    let pool = cfg.pg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    let client = pool.get().await?;
    let stmt = client.prepare_cached("SELECT version()").await?;
    let rows = client.query(&stmt, &[]).await?;

    for row in rows {
        let version: &str = row.get(0);
        println!("PostgreSQL version: {}", version);
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()>{
    unsafe {
      std::env::set_var("PG__PASSWORD", "mysecretpassword");
      std::env::set_var("PG__DBNAME", "library");
    }
    main()?;
    Ok(())
}
