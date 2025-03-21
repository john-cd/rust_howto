// ANCHOR: example
use futures::stream::TryStreamExt;
use tiberius::AuthMethod;
use tiberius::Client;
use tiberius::Config;
use tiberius::QueryItem;
use tiberius::QueryStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

// In your `Cargo.toml`, add the following dependencies:
// [dependencies]
// tiberius = "0.12.3"
// tokio-util = { version = "0.7.13", features = [ "compat" ] }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host =
        std::env::var("MSSQL_HOST").unwrap_or_else(|_| "localhost".into());
    // Configure connection to SQL Server
    let mut config = Config::new();
    config.host(host);
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", "password123!"));
    config.database("master"); // You can change this to your database name

    // Open a connection to SQL Server
    let tcp_stream = tokio::net::TcpStream::connect(config.get_addr()).await?;
    // Sets the value of the TCP_NODELAY option on this socket.
    // If set, segments are always sent as soon as possible,
    // even if there is only a small amount of data.
    // When not set, data is buffered until there is a sufficient amount to send
    // out, thereby avoiding the frequent sending of small packets.
    tcp_stream.set_nodelay(true)?;
    let mut client = Client::connect(config, tcp_stream.compat_write()).await?;

    // Run a simple query to check the connection
    let q = "SELECT TOP 1 name FROM sys.databases";
    let mut query_stream: QueryStream = client.query(q, &[]).await?;

    // Process the result
    while let Some(item) = query_stream.try_next().await? {
        match item {
            QueryItem::Metadata(_meta) => {
                // ...
            }
            QueryItem::Row(row) => {
                // Retrieve a column value for a given column index, which can
                // either be the zero-indexed position or the
                // name of the column
                let db_name: &str = row
                    .get(0)
                    .ok_or_else(|| anyhow::anyhow!("row.get error"))?;
                println!("Database name: {}", db_name);
            }
        }
        // Or use `into_row_stream(self)` to get a stream of rows directly
    }

    // Close the connection
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    unsafe {
        // Refer to the compose*.yaml files
        std::env::set_var("MSSQL_HOST", "rust_howto_dev-mssql-1");
    }
    main()?;
    Ok(())
}
// [fix heavy test](https://github.com/john-cd/rust_howto/issues/1019)
