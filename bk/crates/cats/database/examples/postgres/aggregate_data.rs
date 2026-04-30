#![allow(dead_code)]
// ANCHOR: example
// This example demonstrates aggregating query results in PostgreSQL using the
// `postgres` crate.
use postgres::Client;
use postgres::Error;
use postgres::NoTls;

/// Represents a nation with its nationality and the count of artists from that
/// nation.
struct Nation {
    nationality: String,
    count: i64,
}

/// Connects to a PostgreSQL database, queries artist nationalities and their
/// counts, and prints the results. Data from
// <https://github.com/MuseumofModernArt/collection/tree/main>.
fn main() -> Result<(), Error> {
    // The connection URL is formatted as
    // `postgresql://<user>:<password>@<host>/<db>`, for example
    // `postgresql://postgres:postgres@127.0.0.1/moma`.
    let url = std::env::var("PG_URL").expect("PG_URL must be set");
    let mut client = Client::connect(&url, NoTls)?;

    let query = r#"SELECT nationality, COUNT(nationality) AS count
        FROM artists
        GROUP BY nationality
        ORDER BY count DESC"#;

    for row in client.query(query, &[])? {
        let (nationality, count): (Option<String>, Option<i64>) =
            (row.get(0), row.get(1));

        if let (Some(nationality), Some(count)) = (nationality, count) {
            let nation = Nation { nationality, count };
            println!("{} {}", nation.nationality, nation.count);
        }
    }

    Ok(())
}
// ANCHOR_END: example
pub fn run() -> Result<(), Error> {
    main()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_postgres_environment() -> Result<(), Error> {
        if std::env::var("PG_URL").is_err() {
            eprintln!(
                "Skipping PostgreSQL aggregation example test; set PG_URL to run this test."
            );
            return Ok(());
        }

        main()?;
        Ok(())
    }
}
// [review](https://github.com/john-cd/rust_howto/issues/1162)
