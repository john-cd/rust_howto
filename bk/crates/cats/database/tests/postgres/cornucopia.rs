// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use cornucopia_async::GenericClient;
// use tokio_postgres::{NoTls, Error};

// // Cornucopia is a Rust library for managing database connections and
// executing queries. It provides type-safe SQL in Rust. // https://cornucopia-rs.netlify.app/

// // tokio = { version = "1", features = ["full"] }
// // cornucopia = "0.3.0"
// // cornucopia_async = "0.3.0"
// // postgres = { version = "0.19", features = ["tokio-runtime"] }

// // CREATE TABLE greetings (
// //     id SERIAL PRIMARY KEY,
// //     message TEXT NOT NULL
// // );
// // INSERT INTO greetings (message) VALUES ('Hello, Cornucopia!');

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let (client, connection) =
//         tokio_postgres::connect("host=localhost user=your_user
// dbname=your_db", NoTls).await?;

//     // The connection object performs the actual communication with the
// database,     // so spawn it off to run on its own.
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("connection error: {}", e);
//         }
//     });

//     // Select and print the greetings
//     let rows = client
//         .query("SELECT message FROM greetings", &[])
//         .await?;

//     for row in rows {
//         let message: &str = row.get(0);
//         println!("{}", message);
//     }

//     Ok(())
// }

#[test]
fn require_external_svc() {
    // main();
}
// [finish](https://github.com/john-cd/rust_howto/issues/708)
