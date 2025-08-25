#![allow(dead_code)]
// ANCHOR: example
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
pub fn main() -> Result<(), Error> {
    // The connection URL is formatted as
    // `postgresql://<user>:<password>@<host>/<db>`, for example
    // `postgresql://postgres:postgres@127.0.0.1/moma`.
    let mut client = Client::connect(
        "postgresql://postgres:mysecretpassword@rust_howto_dev-postgres-1/moma",
        NoTls,
    )?;

    for row in client.query(
        "SELECT nationality, COUNT(nationality) AS count
 FROM artists GROUP BY nationality ORDER BY count DESC",
        &[],
    )? {
        let (nat, cnt): (Option<String>, Option<i64>) =
            (row.get(0), row.get(1));

        if let Some(nationality) = nat
            && let Some(count) = cnt
        {
            let nation = Nation { nationality, count };
            println!("{} {}", nation.nationality, nation.count);
        }
    }

    Ok(())
}
// ANCHOR_END: example
// [review](https://github.com/john-cd/rust_howto/issues/1162)
