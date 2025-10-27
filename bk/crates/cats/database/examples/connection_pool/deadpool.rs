#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `deadpool` crate to manage a pool
//! of connections.
//!
//! The `deadpool` crate provides a generic connection pool implementation that
//! can be used with any type that implements the `managed::Manager` trait.
//!
//! In this example, we create a simple `Server` type that represents a
//! connection to a server. The `Manager` type is responsible for creating and
//! recycling `Server` instances.
use deadpool::managed;

#[derive(Debug)]
enum Error {
    Fail, // Represents a failure in the connection or manager.
}

struct Server;

impl Server {
    // Represents a connection to a server.
    async fn get_answer(&self) -> i32 {
        42
    }
}

struct Manager;

// The `Manager` is responsible for creating and recycling `Server` instances.
impl managed::Manager for Manager {
    type Error = Error;
    type Type = Server;

    /// Creates a new `Server` instance.
    async fn create(&self) -> Result<Server, Error> {
        Ok(Server) // In a real-world scenario, this would establish a connection.
    }

    async fn recycle(
        &self,
        _: &mut Server,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        Ok(())
    }
}

// Type alias for the connection pool.
type Pool = managed::Pool<Manager>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mgr = Manager;
    // Create a new connection pool with the given manager.
    let pool = Pool::builder(mgr).build()?;

    // Get a connection from the pool.
    let conn = pool.get().await.map_err(|err| {
        anyhow::anyhow!("Could not retrieve from the Pool: {err:?}")
    })?;
    let answer = conn.get_answer().await;
    assert_eq!(answer, 42);
    println!("The answer is {answer}");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main().unwrap();
}
