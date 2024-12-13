#![allow(dead_code)]
// ANCHOR: example
use deadpool::managed;

#[derive(Debug)]
enum Error {
    Fail,
}

struct Server;

impl Server {
    async fn get_answer(&self) -> i32 {
        42
    }
}

struct Manager;

impl managed::Manager for Manager {
    type Error = Error;
    type Type = Server;

    async fn create(&self) -> Result<Server, Error> {
        Ok(Server)
    }

    async fn recycle(
        &self,
        _: &mut Server,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        Ok(())
    }
}

type Pool = managed::Pool<Manager>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mgr = Manager;
    let pool = Pool::builder(mgr).build()?;

    let conn = pool.get().await.map_err(|err| {
        anyhow::anyhow!("Could not retrieve from the Pool: {:?}", err)
    })?;
    let answer = conn.get_answer().await;
    assert_eq!(answer, 42);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main().unwrap();
}
