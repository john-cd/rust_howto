// ANCHOR: example
use postgres::Client;
use postgres::Error;
use postgres::NoTls;

struct Nation {
    nationality: String,
    count: i64,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:postgres@127.0.0.1/moma",
        NoTls,
    )?;

    for row in client.query(
        "SELECT nationality, COUNT(nationality) AS count
 FROM artists GROUP BY nationality ORDER BY count DESC",
        &[],
    )? {
        let (nationality, count): (Option<String>, Option<i64>) =
            (row.get(0), row.get(1));

        if nationality.is_some() && count.is_some() {
            let nation = Nation {
                nationality: nationality.unwrap(),
                count: count.unwrap(),
            };
            println!("{} {}", nation.nationality, nation.count);
        }
    }

    Ok(())
}

// ANCHOR_END: example
#[ignore = "requires a running postgres instance"]
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
