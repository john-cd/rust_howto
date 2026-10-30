// TODO Reviewed ??-??-2026.

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

/// The errors of the connection manager.
#[derive(Debug)]
enum Error {
    /// A failure in the connection or in the manager.
    Fail, // Represents a failure in the connection or manager.
}

/// A server, which stands for a database connection.
struct Server;

impl Server {
    // Represents a connection to a server.
    /// Returns the answer of the server.
    async fn get_answer(&self) -> i32 {
        42
    }
}

/// Creates and recycles the `Server` connections of the pool.
struct Manager {
    /// Whether `create` should fail, to show how errors surface.
    fail: bool,
}

// The `Manager` is responsible for creating and recycling `Server` instances.
impl managed::Manager for Manager {
    type Error = Error;
    type Type = Server;

    /// Creates a new `Server` instance.
    async fn create(&self) -> Result<Server, Error> {
        if self.fail {
            return Err(Error::Fail);
        }
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
/// A pool of `Server` connections.
type Pool = managed::Pool<Manager>;

/// Builds a pool, gets a connection from it and queries the server.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A manager that cannot create connections reports its error:
    let failing_pool = Pool::builder(Manager { fail: true }).build()?;
    assert!(failing_pool.get().await.is_err());

    let mgr = Manager { fail: false };
    // Create a new connection pool with the given manager.
    let pool = Pool::builder(mgr).max_size(1).build()?;

    // Get a connection from the pool.
    let conn = pool.get().await.map_err(|err| {
        anyhow::anyhow!("Could not retrieve from the Pool: {err:?}")
    })?;
    let answer = conn.get_answer().await;
    assert_eq!(answer, 42);
    drop(conn);

    // A dropped object is available for the next checkout, so the one-slot
    // pool does not need to allocate another connection.
    assert_eq!(pool.status().size, 1);
    assert_eq!(pool.status().available, 1);
    let reused_conn = pool.get().await.map_err(|err| {
        anyhow::anyhow!(
            "failed to get a reused connection from the pool: {err:?}"
        )
    })?;
    assert_eq!(pool.status().size, 1);
    drop(reused_conn);

    println!("The answer is {answer}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that dropping and rechecking out a guard reuses the pool slot.
    /// Runs the deadpool example through its entry point.
    #[test]
    fn test_deadpool_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
